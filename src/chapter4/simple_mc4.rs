use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use crate::chapter4::vanilla2::VanillaOption;
use rand::SeedableRng;

#[allow(dead_code)]
pub fn simple_montecarlo3(
    the_option: &VanillaOption,
    spot: f64,
    vol: f64,
    r: f64,
    number_of_paths: u32,
) -> f64 {
    let expiry = the_option.get_expiry();
    let variance = vol * vol * expiry;
    let root_variance = variance.sqrt();
    let ito_correlation = -0.5 * variance;
    let moved_spot = spot * (r * expiry + ito_correlation).exp();
    let mut this_spot;
    let mut runnning_sum = 0.0;
    let mut rng = SeedableRng::from_entropy();
    for _i in 0..number_of_paths {
        let this_gaussian = get_one_gaussian_by_box_muller(&mut rng);
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        let this_payoff = the_option.option_payoff(this_spot);
        runnning_sum += this_payoff;
    }
    let mut mean = runnning_sum / number_of_paths as f64;
    mean *= (-r * expiry).exp();
    mean
}
