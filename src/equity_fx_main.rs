use rust_design_pattern_derivative_pricing::chapter4::parameters::ParametersConstant;
use rust_design_pattern_derivative_pricing::chapter4::payoff3::PayoffCall;
use rust_design_pattern_derivative_pricing::chapter5::convergence_table::ConvergenceTable;
use rust_design_pattern_derivative_pricing::chapter5::mc_statistics::StatisticsMC;
use rust_design_pattern_derivative_pricing::chapter5::mc_statistics::StatisticsMean;
use rust_design_pattern_derivative_pricing::chapter6::anti_thetic::AntiThetic;
use rust_design_pattern_derivative_pricing::chapter6::park_miller::RandomParkMiller;
use rust_design_pattern_derivative_pricing::chapter7::exotic_bs_engine::ExoticBSEngine;
use rust_design_pattern_derivative_pricing::chapter7::exotic_engine::{
    ExoticEngine, ExoticEngineField,
};
use rust_design_pattern_derivative_pricing::chapter7::path_dependent_asian::PathDependentAsian;

use std::sync::Arc;
use std::sync::Mutex;

pub fn main() {
    println!("\nEnter expiry\n");
    let expiry: f64 = text_io::read!();

    println!("\nEnter strike\n");
    let strike = text_io::read!();

    println!("\nEnter spot\n");
    let spot: f64 = text_io::read!();

    println!("\nEnter vol\n");
    let vol: f64 = text_io::read!();

    println!("\nr\n");
    let r: f64 = text_io::read!();

    println!("\nd\n");
    let d: f64 = text_io::read!();

    println!("\nNumber of dates\n");
    let number_of_dates = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths: u64 = text_io::read!();
    let the_payoff = PayoffCall::new(strike);
    let times = (0..number_of_dates)
        .map(|i| (i as f64 + 1.0) * expiry / number_of_dates as f64)
        .collect();
    let vol_param = ParametersConstant::from(vol);
    let r_param = ParametersConstant::from(r);
    let d_param = ParametersConstant::from(d);
    let the_option = PathDependentAsian::new(times, expiry, &the_payoff);
    let gatherer = Arc::new(Mutex::new(StatisticsMean::default()));
    let mut gatherer_two = ConvergenceTable::new(gatherer);
    let generator = Arc::new(Mutex::new(RandomParkMiller::new(number_of_dates as u64, 1)));
    let gen_two = Arc::new(Mutex::new(AntiThetic::new(generator)));
    let exotic_engine_field = ExoticEngineField::new(&the_option, &r_param);
    let mut the_engine =
        ExoticBSEngine::new(exotic_engine_field, d_param, vol_param, gen_two, spot);
    the_engine.do_simulation(&mut gatherer_two, number_of_paths);
    let results = gatherer_two.get_results_so_far();
    println!("\nFor the Asian call price the results are \n");
    for i in 0..results.len() {
        for j in 0..results[i].len() {
            print!("{} ", results[i][j]);
        }
        println!("\n");
    }
}

pub fn price(
    expiry: f64,
    strike: f64,
    spot: f64,
    vol: f64,
    r: f64,
    d: f64,
    number_of_dates: u64,
    number_of_paths: u64,
) -> f64 {
    let the_payoff = PayoffCall::new(strike);
    let times = (0..number_of_dates)
        .map(|i| (i as f64 + 1.0) * expiry / number_of_dates as f64)
        .collect();
    let vol_param = ParametersConstant::from(vol);
    let r_param = ParametersConstant::from(r);
    let d_param = ParametersConstant::from(d);
    let the_option = PathDependentAsian::new(times, expiry, &the_payoff);
    let gatherer = Arc::new(Mutex::new(StatisticsMean::default()));
    let mut gatherer_two = ConvergenceTable::new(gatherer);
    let generator = Arc::new(Mutex::new(RandomParkMiller::new(number_of_dates as u64, 1)));
    let gen_two = Arc::new(Mutex::new(AntiThetic::new(generator)));
    let exotic_engine_field = ExoticEngineField::new(&the_option, &r_param);
    let mut the_engine =
        ExoticBSEngine::new(exotic_engine_field, d_param, vol_param, gen_two, spot);
    the_engine.do_simulation(&mut gatherer_two, number_of_paths);
    let results = gatherer_two.get_results_so_far();
    println!("\nFor the Asian call price the results are \n");
    for i in 0..results.len() {
        for j in 0..results[i].len() {
            print!("{} ", results[i][j]);
        }
        println!("\n");
    }
    results[0][0]
}

#[test]
pub fn test_main() {
    let expiry = 30.0;
    let strike = 100.0;
    let spot = 100.0;
    let vol = 0.01;
    let r = 0.01;
    let d = 0.0;
    let number_of_dates = 1000;
    let number_of_paths = 1000;
    let the_payoff = PayoffCall::new(strike);
    let times = (0..number_of_dates)
        .map(|i| (i as f64 + 1.0) * expiry / number_of_dates as f64)
        .collect();
    let vol_param = ParametersConstant::from(vol);
    let r_param = ParametersConstant::from(r);
    let d_param = ParametersConstant::from(d);
    let the_option = PathDependentAsian::new(times, expiry, &the_payoff);
    let gatherer = Arc::new(Mutex::new(StatisticsMean::default()));
    let mut gatherer_two = ConvergenceTable::new(gatherer);
    let generator = Arc::new(Mutex::new(RandomParkMiller::new(number_of_dates as u64, 1)));
    let gen_two = Arc::new(Mutex::new(AntiThetic::new(generator)));
    let exotic_engine_field = ExoticEngineField::new(&the_option, &r_param);
    let mut the_engine =
        ExoticBSEngine::new(exotic_engine_field, d_param, vol_param, gen_two, spot);
    the_engine.do_simulation(&mut gatherer_two, number_of_paths);
    let results = gatherer_two.get_results_so_far();
    assert_eq!(
        results,
        vec![
            vec![12.297472829436929, 2.0],
            vec![12.289559897741082, 4.0],
            vec![12.319043908148446, 8.0],
            vec![12.320459101867279, 16.0],
            vec![12.33025903792299, 32.0],
            vec![12.326630351299876, 64.0],
            vec![12.32874746376825, 128.0],
            vec![12.322480274727422, 256.0],
            vec![12.322134425108427, 512.0],
            vec![12.321899356270709, 1000.0],
        ]
    )
}
