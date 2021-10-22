mod chapter1;
mod chapter3;
use crate::chapter3::payoff2::Payoff;
use crate::chapter3::payoff2::PayoffCall;
use crate::chapter3::payoff2::PayoffPut;
use crate::chapter3::simple_mc2::simple_montecarlo2;
use std::rc::Rc;
fn main() {
    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter Strike\n");
    let strike = text_io::read!();

    println!("\nEnter spot\n");
    let spot = text_io::read!();

    println!("\nEnter vol\n");
    let vol = text_io::read!();

    println!("\nEnter r\n");
    let r = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths = text_io::read!();

    println!("\n0 for call, otherwise put\n");
    let option_type = text_io::read!();
    let the_payoff_ptr;
    match option_type {
        0 => {
            the_payoff_ptr = Rc::new(PayoffCall::new(strike)) as Rc<dyn Payoff>;
        }
        _ => {
            the_payoff_ptr = Rc::new(PayoffPut::new(strike)) as Rc<dyn Payoff>;
        }
    };

    let result = simple_montecarlo2(
        the_payoff_ptr.as_ref(),
        expiry,
        spot,
        vol,
        r,
        number_of_paths,
    );

    println!("the price is {} \n", result);
    drop(the_payoff_ptr);
}
