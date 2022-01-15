mod chapter1;
mod chapter3;
mod chapter4;

use crate::chapter4::parameters::ParametersConstant;

use crate::chapter4::payoff3::Payoff;
use crate::chapter4::payoff3::PayoffCall;
use crate::chapter4::payoff3::PayoffPut;
use crate::chapter4::simple_mc6::simple_montecarlo4;
use crate::chapter4::vanilla3::VanillaOption;

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

    let vol_param = ParametersConstant::new(vol);
    let r_param = ParametersConstant::new(r);

    let the_payoff = &PayoffCall::new(strike);
    let the_option = VanillaOption::new(the_payoff as &dyn Payoff, expiry);
    let result = simple_montecarlo4(&the_option, spot, &vol_param, &r_param, number_of_paths);

    println!("the call price is {} \n", result);

    let second_option = the_option.clone();
    let result = simple_montecarlo4(&second_option, spot, &vol_param, &r_param, number_of_paths);
    println!("the call price is {} \n", result);
    let other_payoff = PayoffPut::new(strike);
    let third_option = VanillaOption::new(&other_payoff, expiry);
    let result = simple_montecarlo4(&third_option, spot, &vol_param, &r_param, number_of_paths);
    println!("the put price is {} \n", result);
}
