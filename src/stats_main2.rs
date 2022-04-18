//! Decoratorパターンを援用することで、インターフェースを変更することなく機能を追加している。
use crate::chapter4::parameters::Parameters;
use crate::chapter4::payoff3::PayoffCall;
use crate::chapter4::payoff_bridge::PayoffBridge;
use crate::chapter4::vanilla3::VanillaOption;
use crate::chapter5::convergence_table::ConvergenceTable;
use crate::chapter5::mc_statistics::{StatisticsMC, StatisticsMean};
use crate::chapter5::simple_mc7::simple_montecarlo5;
use std::sync::{Arc, Mutex};

pub fn main() {
    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter strike\n");
    let strike = text_io::read!();

    println!("\nEnter spot\n");
    let spot = text_io::read!();

    println!("\nEnter vol\n");
    let vol: f64 = text_io::read!();

    println!("\nEnter r\n");
    let r: f64 = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths = text_io::read!();

    let the_payoff = PayoffBridge::new(Box::new(PayoffCall::new(strike)));
    let the_option = VanillaOption::new(the_payoff, expiry);
    let vol_param = Parameters::from(vol);
    let r_param = Parameters::from(r);
    let gatherer = Arc::new(Mutex::new(StatisticsMean::default()));
    let mut gatherer_two = ConvergenceTable::new(gatherer);
    simple_montecarlo5(
        &the_option,
        spot,
        &vol_param,
        &r_param,
        number_of_paths,
        &mut gatherer_two,
    );
    let results = gatherer_two.get_results_so_far();
    println!("`\nFor the call price the results are \n");
    for i in 0..results.len() {
        for j in 0..results[i].len() {
            print!("{} ", results[i][j]);
        }
        println!("\n");
    }
}
