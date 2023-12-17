//! Decoratorパターンを援用することで、インターフェースを変更することなく機能を追加している。

use rust_design_pattern_derivative_pricing::chapter4::parameters::ParametersConstant;
use rust_design_pattern_derivative_pricing::chapter4::payoff3::{Payoff, PayoffCall};
use rust_design_pattern_derivative_pricing::chapter4::vanilla3::VanillaOption;
use rust_design_pattern_derivative_pricing::chapter5::convergence_table::ConvergenceTable;
use rust_design_pattern_derivative_pricing::chapter5::mc_statistics::{
    StatisticsMC, StatisticsMean,
};
use rust_design_pattern_derivative_pricing::chapter6::anti_thetic::AntiThetic;
use rust_design_pattern_derivative_pricing::chapter6::park_miller::RandomParkMiller;
use rust_design_pattern_derivative_pricing::chapter6::simple_mc8::simple_montecarlo6;

pub fn main() {
    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter strike\n");
    let strike = text_io::read!();

    println!("\nEnter spot\n");
    let spot = text_io::read!();

    println!("\nEnter vol\n");
    let vol: f64 = text_io::read!();

    println!("\nr\n");
    let r: f64 = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths = text_io::read!();

    let the_payoff = PayoffCall::new(strike);
    let the_option = VanillaOption::new(&the_payoff, expiry);
    let vol_param = ParametersConstant::new(vol);
    let r_param = ParametersConstant::new(r);
    let gatherer = StatisticsMean::default();
    let mut gatherer_two = ConvergenceTable::new(gatherer);
    let generator = RandomParkMiller::new(1, 1);
    let mut gen_two = AntiThetic::new(generator);
    simple_montecarlo6(
        &the_option,
        spot,
        &vol_param,
        &r_param,
        number_of_paths,
        &mut gatherer_two,
        &mut gen_two,
    );
    let results = gatherer_two.get_results_so_far();
    println!("`\nFor the call price the results are \n");
    for result in &results {
        for data in result {
            print!("{} ", data);
        }
        println!("\n");
    }
}
