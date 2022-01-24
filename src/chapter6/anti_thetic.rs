use crate::chapter6::random2::RandomBase;
use crate::chapter6::random2::RandomBaseField;

#[derive(Clone)]
struct AntiThetic<'a> {
    random_base: RandomBaseField,
    inner_generator: Box<dyn RandomBase>,
    odd_even: bool,
    next_variates: &'a [f64],
}

// impl<'a> AntiThetic<'a> {
//     pub fn new(
//         random_base: RandomBaseField,
//         inner_generator: Box<dyn RandomBase>,
//     ) -> AntiThetic<'a> {
//         AntiThetic {
//             random_base,
//             inner_generator,
//             odd_even: true,
//             next_variates: &Vec::with_capacity(random_base.dimensionality as usize),
//         }
//     }
// }

// impl RandomBase for AntiThetic {
//     fn box_clone(&self) -> Box<dyn RandomBase> {
//         Box::new((*self).clone())
//     }
//     fn get_dimensionality(&self) -> u64 {
//         self.random_base.dimensionality
//     }
//     fn get_uniforms(&mut self, variates: &mut [f64]) {}
// }
