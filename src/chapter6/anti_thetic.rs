//! デコレーターパターンを用いることで、anti-thetic法を任意の乱数生成と任意のモンテカルロシミュレーションに適用することができるようにした。

use crate::chapter6::random2::Random;

/// Implemented in the decorator pattern.
#[derive(Clone)]
pub struct AntiThetic<T: Random> {
    dimensionality: usize,
    generator: T,
    odd_even: bool,
    next_variates: Vec<f64>,
}

impl<T: Random> AntiThetic<T> {
    pub fn new(generator: T) -> AntiThetic<T> {
        let dimensionality = generator.get_dimensionality();
        AntiThetic {
            dimensionality,
            generator,
            odd_even: true,
            next_variates: vec![0.0; dimensionality],
        }
    }
}

impl<T: Random> Random for AntiThetic<T> {
    fn get_dimensionality(&self) -> usize {
        self.dimensionality
    }
    fn get_uniforms(&mut self, variates: &mut [f64]) {
        if self.odd_even {
            self.generator.get_uniforms(variates);
            self.next_variates = variates.iter().map(|variate| 1.0 - variate).collect();
            self.odd_even = false;
        } else {
            variates.copy_from_slice(&self.next_variates);
            self.odd_even = true;
        }
    }
    fn skip(&mut self, mut number_of_paths: usize) {
        if number_of_paths == 0 {
            return;
        }
        if self.odd_even {
            self.odd_even = false;
            number_of_paths -= 1;
        }
        self.generator.skip(number_of_paths / 2);
        if number_of_paths % 2 == 1 {
            let mut tmp = vec![0.0; self.get_dimensionality()];
            self.get_uniforms(&mut tmp);
        }
    }
    fn set_seed(&mut self, seed: u64) {
        self.generator.set_seed(seed);
        self.odd_even = true;
    }
    fn reset(&mut self) {
        self.generator.reset();
        self.odd_even = true;
    }
    fn reset_dimensionality(&mut self, new_dimensionality: usize) {
        self.dimensionality = new_dimensionality;
        self.next_variates.resize(new_dimensionality, 0.0);
        self.generator.reset_dimensionality(new_dimensionality);
    }
}
