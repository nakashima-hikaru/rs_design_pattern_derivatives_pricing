use crate::chapter6::random2::RandomBase;
use crate::chapter6::random2::RandomBaseField;

#[derive(Clone)]
struct AntiThetic {
    random_base: RandomBaseField,
    inner_generator: Box<dyn RandomBase>,
    odd_even: bool,
    next_variates: Vec<f64>,
}

impl<'a> AntiThetic {
    pub fn new(random_base: RandomBaseField, inner_generator: Box<dyn RandomBase>) -> AntiThetic {
        AntiThetic {
            random_base,
            inner_generator,
            odd_even: true,
            next_variates: Vec::<f64>::with_capacity(random_base.dimensionality as usize),
        }
    }
}

impl<'a> RandomBase for AntiThetic {
    fn box_clone(&self) -> Box<dyn RandomBase> {
        Box::new(self.clone())
    }
    fn get_dimensionality(&self) -> u64 {
        self.random_base.dimensionality
    }
    fn get_uniforms(&mut self, variates: &mut [f64]) {
        if self.odd_even {
            self.inner_generator.get_uniforms(variates);
            for i in 0..self.get_dimensionality() {
                self.next_variates[i as usize] = 1.0 - variates[i as usize];
            }
            self.odd_even = false;
        } else {
            variates.clone_from_slice(&self.next_variates);
            self.odd_even = true;
        }
    }
    fn set_seed(&mut self, seed: u64) {
        self.inner_generator.set_seed(seed);
        self.odd_even = true;
    }
    fn skip(&mut self, mut number_of_paths: u64) {
        if number_of_paths == 0 {
            return;
        }
        if self.odd_even {
            self.odd_even = false;
            number_of_paths -= 1;
        }
        self.inner_generator.skip(number_of_paths / 2);
        if number_of_paths % 2 == 1 {
            let mut tmp = Vec::<f64>::with_capacity(self.get_dimensionality() as usize);
            self.get_uniforms(&mut tmp);
        }
    }
    fn reset_dimensionality(&mut self, new_dimensionality: u64) {
        self.random_base.dimensionality = new_dimensionality;
        self.next_variates.resize(new_dimensionality as usize, 0.0);
        self.inner_generator
            .reset_dimensionality(new_dimensionality);
    }
    fn reset(&mut self) {
        self.inner_generator.reset();
        self.odd_even = true;
    }
}
