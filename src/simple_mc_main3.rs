mod chapter1;
mod chapter3;
use crate::chapter3::{
    payoff2::{PayoffCall, PayoffPut},
    simple_mc2::simple_montecarlo2,
};

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

    let call_payoff = PayoffCall::new(strike);
    let put_payoff = PayoffPut::new(strike);
    let result_call = simple_montecarlo2(&call_payoff, expiry, spot, vol, r, number_of_paths);

    let result_put = simple_montecarlo2(&put_payoff, expiry, spot, vol, r, number_of_paths);
    println!(
        "the prices are {} for the call and {} for the put\n",
        result_call, result_put
    );
}
