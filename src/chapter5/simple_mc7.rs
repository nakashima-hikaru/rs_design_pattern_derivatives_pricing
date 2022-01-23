/// gathererを参照とすることによって、この関数内でのgathererの変更を関数の外でも反映されるようにしている。
/// 平均を求める処理が一行で済むようになり、可読性が向上した。
use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use crate::chapter4::parameters::ParametersInner;
use crate::chapter4::vanilla3::VanillaOption;
use crate::chapter5::mc_statistics::StatisticsMC;
use rand::SeedableRng;

pub fn simple_montecarlo5(
    the_option: &VanillaOption,
    spot: f64,
    vol: &dyn ParametersInner,
    r: &dyn ParametersInner,
    number_of_paths: u32,
    gatherer: &mut dyn StatisticsMC,
) {
    let expiry = the_option.get_expiry();
    let variance = vol.integral_square(0.0, expiry);
    let root_variance = variance.sqrt();
    let ito_correlation = -0.5 * variance;
    let moved_spot = spot * (r.integral(0.0, expiry) + ito_correlation).exp();
    let discounting = (-r.integral(0.0, expiry)).exp();
    let mut this_spot;
    let mut rng = SeedableRng::from_entropy();
    for _i in 0..number_of_paths {
        let this_gaussian = get_one_gaussian_by_box_muller(&mut rng);
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        let this_payoff = the_option.option_payoff(this_spot);
        gatherer.dump_one_result(this_payoff * discounting);
    }
}