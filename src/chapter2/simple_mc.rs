use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use crate::chapter2::payoff1::Payoff;

pub fn simple_montecarlo2(
    the_payoff: &Payoff,
    expiry: f64,
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
    for _i in 0..number_of_paths {
        let this_gaussian = get_one_gaussian_by_box_muller();
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        let this_payoff = the_payoff.forward_value(this_spot);
        running_sum += this_payoff;
    }
    let mut mean = running_sum / number_of_paths as f64;
    mean *= (-r * expiry).exp();
    mean
}
