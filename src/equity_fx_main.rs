/// Decoratorパターンを援用することで、インターフェースを変更することなく機能を追加している。
use crate::chapter4::parameters::Parameters;
use crate::chapter4::payoff3::PayoffCall;
use crate::chapter4::payoff_bridge::PayoffBridge;
use crate::chapter5::convergence_table::ConvergenceTable;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter5::mc_statistics::StatisticsMean;
use crate::chapter6::anti_thetic::AntiThetic;
use crate::chapter6::park_miller::RandomParkMiller;
use crate::chapter7::exotic_bs_engine::ExoticBSEngine;
use crate::chapter7::exotic_engine::{ExoticEngine, ExoticEngineField};
use crate::chapter7::path_dependent_asian::PathDependentAsian;
use std::cell::RefCell;
use std::rc::Rc;

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
    let the_payoff = Rc::new(PayoffBridge::new(Rc::new(PayoffCall::new(strike))));
    let mut times = vec![0.0; number_of_dates];
    for i in 0..times.len() {
        times[i] = (i as f64 + 1.0) * expiry / number_of_dates as f64;
    }
    let vol_param = Rc::new(Parameters::from(vol));
    let r_param = Rc::new(Parameters::from(r));
    let d_param = Rc::new(Parameters::from(d));
    let the_option = Rc::new(PathDependentAsian::new(&times, expiry, the_payoff));
    let gatherer = Rc::new(RefCell::new(StatisticsMean::default()));
    let mut gatherer_two = ConvergenceTable::new(gatherer);
    let generator = Rc::new(RefCell::new(RandomParkMiller::new(
        number_of_dates as u64,
        1,
    )));
    let gen_two = Rc::new(RefCell::new(AntiThetic::new(generator)));
    let exotic_engine_field = ExoticEngineField::new(the_option, r_param);
    let mut the_engine =
        ExoticBSEngine::new(exotic_engine_field, d_param, vol_param, gen_two, spot);
    the_engine.do_simulation(&mut gatherer_two, number_of_paths);
    let results = gatherer_two.get_results_so_far();
    println!("\nFor the Asian call price the results are \n");
    for i in 0..results.len() {
        for j in 0..results[i].len() {
            println!("{} ", results[i][j]);
        }
        println!("\n");
    }
}
