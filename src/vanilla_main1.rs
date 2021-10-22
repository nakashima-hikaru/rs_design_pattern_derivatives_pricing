mod chapter1;
mod chapter3;
mod chapter4;
use crate::chapter3::double_digital;
use crate::chapter4::simple_mc3::simple_montecarlo3;
use crate::chapter4::vanilla1;
use std::rc::Rc;

fn main() {
    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter low barrier\n");
    let low = text_io::read!();

    println!("\nEnter up barrier\n");
    let up = text_io::read!();

    println!("\nEnter spot\n");
    let spot = text_io::read!();

    println!("\nEnter vol\n");
    let vol = text_io::read!();

    println!("\nEnter r\n");
    let r = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths = text_io::read!();

    let the_payoff_ptr = Rc::new(double_digital::PayoffDoubleDigital::new(low, up));
    let the_option = vanilla1::VanillaOption::new(the_payoff_ptr, expiry);

    let result = simple_montecarlo3(&the_option, spot, vol, r, number_of_paths);

    println!("the price is {} \n", result);
}
