use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use rand::SeedableRng;

pub fn simple_montecarlo1(
    expiry: f64,
    strike: f64,
    spot: f64,
    vol: f64,
    r: f64,
    number_of_paths: u64,
) -> f64 {
    let variance = vol * vol * expiry;
    let root_variance = variance.sqrt();
    let ito_correlation = -0.5 * variance;
    let moved_spot = spot * (r * expiry + ito_correlation).exp();
    let mut this_spot;
    let mut running_sum = 0.0;
    let mut rng = SeedableRng::from_entropy();
    for _i in 0..number_of_paths {
        let this_gaussian = get_one_gaussian_by_box_muller(&mut rng);
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        let mut this_payoff = this_spot - strike;
        this_payoff = if this_payoff > 0.0 { this_payoff } else { 0.0 };
        running_sum += this_payoff;
    }
    let mut mean = running_sum / number_of_paths as f64;
    mean *= (-r * expiry).exp();
    mean
}

pub fn main() {
    //read in parameters
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

    let result = simple_montecarlo1(expiry, strike, spot, vol, r, number_of_paths);
    println!("the price is {}\n", result);
}
