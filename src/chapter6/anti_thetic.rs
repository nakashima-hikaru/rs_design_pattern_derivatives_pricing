use crate::chapter6::random2::RandomBase;
use crate::chapter6::random2::RandomBaseField;

#[derive(Clone)]
pub struct AntiThetic {
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

// #[test]
// fn test_distribution() {
//     let n = 100000;
//     let random_base = RandomBaseField::new(n);
//     let mut x = AntiThetic::new(random_base, 25435344);
//     let mut v = Vec::<f64>::with_capacity(n as usize);
//     for _i in 0..n {
//         v.push(0.0);
//     }

//     x.get_gaussians(&mut v.as_mut_slice());
//     let mut mean = 0.0;
//     let mut variant = 0.0;
//     for u in v {
//         mean += u;
//         variant += u * u;
//     }
//     mean /= n as f64;
//     variant /= n as f64;
//     x.reset_dimensionality(50);
//     println!("{}, {}", mean, variant);
// }
