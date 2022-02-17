// use crate::chapter4::parameters::Parameters;
// use crate::chapter6::random2::RandomBase;
// use crate::chapter7::exotic_engine::ExoticEngine;
// use crate::chapter7::exotic_engine::ExoticEngineField;
// use crate::chapter7::path_dependent::CashFlow;
// use crate::chapter7::path_dependent::PathDependent;
// use std::cell::RefCell;
//
// struct ExoticBSEngine {
//     exotic_engine_field: ExoticEngineField,
//     the_generator: Box<dyn RandomBase>,
//     drifts: Vec<f64>,
//     standard_derivations: Vec<f64>,
//     log_spot: f64,
//     number_of_times: u64,
//     variates: Vec<f64>,
// }
//
// impl ExoticBSEngine {
//     fn new(
//         exotic_engine_field: &ExoticEngineField,
//         the_product: Box<dyn PathDependent>,
//         r: &Parameters,
//         d: &Parameters,
//         vol: &Parameters,
//         mut the_generator: Box<dyn RandomBase>,
//         spot: f64,
//     ) -> ExoticBSEngine {
//         let times = the_product.get_look_at_times();
//         let number_of_times = times.len() as u64;
//
//         the_generator.reset_dimensionality(number_of_times as u64);
//         let mut drifts = vec![0.0; number_of_times as usize];
//         let mut standard_derivations = vec![0.0; number_of_times as usize];
//
//         let variance = vol.integral_square(0.0, times[0]);
//         drifts[0] = r.integral(0.0, times[0]) - d.integral(0.0, times[0]) - 0.5 * variance;
//         standard_derivations[0] = variance.sqrt();
//         for j in 1..number_of_times {
//             let this_variance = vol.integral_square(times[(j - 1) as usize], times[j as usize]);
//             drifts[j as usize] = r.integral(times[(j - 1) as usize], times[j as usize])
//                 - d.integral(times[(j - 1) as usize], times[j as usize])
//                 - 0.5 * this_variance;
//             standard_derivations[j as usize] = this_variance.sqrt();
//         }
//         let log_spot = spot.ln();
//         let variates = vec![0.0; number_of_times as usize];
//         let temp = exotic_engine_field;
//         ExoticBSEngine {
//             exotic_engine_field: temp,
//             the_generator,
//             drifts,
//             standard_derivations,
//             log_spot,
//             number_of_times,
//             variates,
//         }
//     }
// }
//
// impl ExoticEngine for ExoticBSEngine {
//     fn as_exotic_engine_filed(&self) -> &ExoticEngineField {
//         &self.exotic_engine_field
//     }
//     // fn as_mut_exotic_engine_filed(&mut self) -> &mut ExoticEngineField {
//     //     &mut self.exotic_engine_field
//     // }
//     fn get_one_path(&mut self, spot_values: &mut [f64]) {
//         self.the_generator.get_gaussians(&mut self.variates);
//         let mut current_log_spot = self.log_spot;
//         for j in 0..self.number_of_times {
//             current_log_spot += self.drifts[j as usize];
//             current_log_spot += self.standard_derivations[j as usize] * self.variates[j as usize];
//             spot_values[j as usize] = current_log_spot.exp();
//         }
//     }
// }
