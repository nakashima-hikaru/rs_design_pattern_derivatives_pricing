//! Payoffやパラメータに応じて容易に拡張できるようになった
//! *課題点
//! Montecarloシミュレーションの収束に関する指標がない。
use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use crate::chapter4::parameters::Parameters;
use crate::chapter4::vanilla3::VanillaOption;

pub fn simple_montecarlo4(
    the_option: &VanillaOption,
    spot: f64,
    vol: impl Parameters,
    r: impl Parameters,
    number_of_paths: u64,
) -> f64 {
    let expiry = the_option.get_expiry();
    let variance = vol.integral_square(0.0, expiry);
    let root_variance = variance.sqrt();
    let ito_correlation = -0.5 * variance;
    let drift = r.integral(0.0, expiry);

    let moved_spot = spot * (drift + ito_correlation).exp();
    let payoff_sum = (0..number_of_paths)
        .map(|_| {
            let this_gaussian = get_one_gaussian_by_box_muller();
            let this_spot = moved_spot * (root_variance * this_gaussian).exp();
            the_option.option_payoff(this_spot)
        })
        .sum::<f64>();
    payoff_sum / number_of_paths as f64 * (-drift).exp()
}
