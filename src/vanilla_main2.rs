mod chapter1;
mod chapter3;
mod chapter4;
use crate::chapter4::payoff3::PayoffCall;
use crate::chapter4::payoff3::PayoffPut;
use crate::chapter4::simple_mc4::simple_montecarlo3;
use crate::chapter4::vanilla2::VanillaOption;
use std::rc::Rc;

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

    let the_payoff = Rc::new(PayoffCall::new(strike));
    let the_option = VanillaOption::new(the_payoff, expiry);

    let result = simple_montecarlo3(&the_option, spot, vol, r, number_of_paths);

    println!("the call price is {} \n", result);

    let second_option = VanillaOption::from(&the_option);
    let result = simple_montecarlo3(&second_option, spot, vol, r, number_of_paths);
    println!("the call price is {} \n", result);
    let other_payoff = PayoffPut::new(strike);
    let third_option = VanillaOption::new(Rc::new(other_payoff), expiry);
    let result = simple_montecarlo3(&third_option, spot, vol, r, number_of_paths);
    println!("the put price is {} \n", result);
}
