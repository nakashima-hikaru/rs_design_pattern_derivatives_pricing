use crate::chapter4::payoff3::Payoff;
use crate::chapter4::payoff3::PayoffCall;
use crate::chapter4::payoff3::PayoffPut;
use crate::chapter4::simple_mc5::simple_montecarlo3;
use crate::chapter4::vanilla3::VanillaOption;

pub fn main() {
    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter strike\n");
    let strike = text_io::read!();

    println!("\nEnter spot\n");
    let spot: f64 = text_io::read!();

    println!("\nEnter vol\n");
    let vol: f64 = text_io::read!();

    println!("\nEnter r\n");
    let r: f64 = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths: u64 = text_io::read!();

    let the_payoff = &PayoffCall::new(strike);
    let the_option = VanillaOption::new(the_payoff as &dyn Payoff, expiry);
    let result = simple_montecarlo3(&the_option, spot, vol, r, number_of_paths);

    println!("the call price is {} \n", result);

    let second_option = the_option.clone();
    let result = simple_montecarlo3(&second_option, spot, vol, r, number_of_paths);
    println!("the call price is {} \n", result);
    let other_payoff = PayoffPut::new(strike);
    let third_option = VanillaOption::new(&other_payoff, expiry);
    let result = simple_montecarlo3(&third_option, spot, vol, r, number_of_paths);
    println!("the put price is {} \n", result);
}
