//! Payoffやパラメータに応じて容易に拡張できるようになった
//! *課題点
//! Montecarloシミュレーションの収束に関する指標がない。
use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use crate::chapter4::parameters::Parameters;
use crate::chapter4::vanilla3::VanillaOption;

pub fn simple_montecarlo4(
    the_option: &VanillaOption,
    spot: f64,
    vol: &Parameters,
    r: &Parameters,
    number_of_paths: u64,
) -> f64 {
    let expiry = the_option.get_expiry();
    let variance = vol.integral_square(0.0, expiry);
    let root_variance = variance.sqrt();
    let ito_correlation = -0.5 * variance;
    let moved_spot = spot * (r.integral(0.0, expiry) + ito_correlation).exp();
    let mut this_spot;
    let mut running_sum = 0.0;
    for _i in 0..number_of_paths {
        let this_gaussian = get_one_gaussian_by_box_muller();
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        let this_payoff = the_option.option_payoff(this_spot);
        running_sum += this_payoff;
    }
    let mut mean = running_sum / number_of_paths as f64;
    mean *= (-r.integral(0.0, expiry)).exp();
    mean
}
