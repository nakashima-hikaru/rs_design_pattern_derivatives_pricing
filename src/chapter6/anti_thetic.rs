//! デコレーターパターンを用いることで、anti-thetic法を任意の乱数生成と任意のモンテカルロシミュレーションに適用することができるようにした。

use crate::chapter6::random2::RandomBase;
use std::{cell::RefCell, rc::Rc};

/// Implemented in the decorator pattern.
#[derive(Clone)]
pub struct AntiThetic {
    dimensionality: u64,
    inner_generator: Rc<RefCell<dyn RandomBase>>,
    odd_even: bool,
    next_variates: Vec<f64>,
}

impl AntiThetic {
    pub fn new(inner_generator: Rc<RefCell<dyn RandomBase>>) -> AntiThetic {
        let dimensionality = inner_generator.borrow().get_dimensionality();
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
    fn get_uniforms(&mut self, variates: &mut [f64]) {
        if self.odd_even {
            self.inner_generator.borrow_mut().get_uniforms(variates);
            self.next_variates = variates.iter().map(|variate| 1.0 - variate).collect();
            self.odd_even = false;
        } else {
            variates.copy_from_slice(&self.next_variates);
            self.odd_even = true;
        }
    }
    fn skip(&mut self, mut number_of_paths: u64) {
        if number_of_paths == 0 {
            return;
        }
        if self.odd_even {
            self.odd_even = false;
            number_of_paths -= 1;
        }
        self.inner_generator.borrow_mut().skip(number_of_paths / 2);
        if number_of_paths % 2 == 1 {
            let mut tmp = vec![0.0; self.get_dimensionality() as usize];
            self.get_uniforms(&mut tmp);
        }
    }
    fn set_seed(&mut self, seed: u64) {
        self.inner_generator.borrow_mut().set_seed(seed);
        self.odd_even = true;
    }
    fn reset(&mut self) {
        self.inner_generator.borrow_mut().reset();
        self.odd_even = true;
    }
    fn reset_dimensionality(&mut self, new_dimensionality: u64) {
        self.dimensionality = new_dimensionality;
        self.next_variates.resize(new_dimensionality as usize, 0.0);
        self.inner_generator
            .borrow_mut()
            .reset_dimensionality(new_dimensionality);
    }
}
