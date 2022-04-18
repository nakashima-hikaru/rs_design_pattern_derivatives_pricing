//! デコレーターパターンを用いることで、anti-thetic法を任意の乱数生成と任意のモンテカルロシミュレーションに適用することができるようにした。

use crate::chapter6::random2::RandomBase;
use std::sync::{Arc, Mutex};

/// Implemented in the decorator pattern.
pub struct AntiThetic {
    dimensionality: u64,
    inner_generator: Arc<Mutex<dyn RandomBase>>,
    odd_even: bool,
    next_variates: Vec<f64>,
}

impl AntiThetic {
    pub fn new(inner_generator: Arc<Mutex<dyn RandomBase>>) -> AntiThetic {
        let dimensionality = inner_generator.lock().as_ref().unwrap().get_dimensionality();
        AntiThetic {
            dimensionality,
            inner_generator,
            odd_even: true,
            next_variates: vec![0.0; dimensionality as usize],
        }
    }
}

impl RandomBase for AntiThetic {
    fn get_dimensionality(&self) -> u64 {
        self.dimensionality
    }
    fn get_uniforms(&mut self) -> Vec<f64> {
        let variates;
        if self.odd_even {
            variates = self.inner_generator.lock().as_mut().unwrap().get_uniforms();
            self.next_variates = variates.iter().map(|variate| 1.0 - variate).collect();
            self.odd_even = false;
        } else {
            variates = self.next_variates.clone();
            self.odd_even = true;
        }
        variates
    }
    fn skip(&mut self, mut number_of_paths: u64) {
        if number_of_paths == 0 {
            return;
        }
        if self.odd_even {
            self.odd_even = false;
            number_of_paths -= 1;
        }
        self.inner_generator.lock().as_mut().unwrap().skip(number_of_paths / 2);
        if number_of_paths % 2 == 1 {
            self.get_uniforms();
        }
    }
    fn set_seed(&mut self, seed: u64) {
        self.inner_generator.lock().as_mut().unwrap().set_seed(seed);
        self.odd_even = true;
    }
    fn reset(&mut self) {
        self.inner_generator.lock().as_mut().unwrap().reset();
        self.odd_even = true;
    }
    fn reset_dimensionality(&mut self, new_dimensionality: u64) {
        self.dimensionality = new_dimensionality;
        self.next_variates.resize(new_dimensionality as usize, 0.0);
        self.inner_generator
            .lock().as_mut().unwrap()
            .reset_dimensionality(new_dimensionality);
    }
}
