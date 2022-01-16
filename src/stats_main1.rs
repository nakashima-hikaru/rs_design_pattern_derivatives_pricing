use crate::chapter4::parameters::ParametersConstant;

use crate::chapter4::vanilla3::VanillaOption;

use crate::chapter4::payoff3::PayoffCall;

use crate::chapter5::mc_statistics::{StatisticsMC, StatisticsMean};
use crate::chapter5::simple_mc7::simple_montecarlo5;

mod chapter1;
mod chapter3;
mod chapter4;
mod chapter5;

fn main() {
    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter strike\n");
    let strike = text_io::read!();

    println!("\nEnter spot\n");
    let spot = text_io::read!();

    println!("\nEnter vol\n");
    let vol = text_io::read!();

    println!("\nEnter r\n");
    let r = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths = text_io::read!();

    let the_payoff = PayoffCall::new(strike);
    let the_option = VanillaOption::new(&the_payoff, expiry);
    let vol_param = ParametersConstant::new(vol);
    let r_param = ParametersConstant::new(r);
    let mut gatherer = StatisticsMean::default();
    simple_montecarlo5(
        &the_option,
        spot,
        &vol_param,
        &r_param,
        number_of_paths,
        &mut gatherer,
    );
    let results = gatherer.get_results_so_far();
    println!("`\nFor the call price the results are \n");
    for i in 0..results.len() {
        for j in 0..results[i].len() {
            println!("{} ", results[i][j]);
        }
        println!("\n");
    }
}
