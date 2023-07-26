//! gathererを参照とすることによって、この関数内でのgathererの変更を関数の外でも反映されるようにしている。
//! 平均を求める処理が一行で済むようになり、可読性が向上した。
use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use crate::chapter4::parameters::Parameters;
use crate::chapter4::payoff3::Payoff;
use crate::chapter4::vanilla3::VanillaOption;
use crate::chapter5::mc_statistics::StatisticsMC;

pub fn simple_montecarlo5<T: Payoff>(
    the_option: &VanillaOption<T>,
    spot: f64,
    vol: impl Parameters,
    r: impl Parameters,
    number_of_paths: u64,
    gatherer: &mut impl StatisticsMC,
) {
    let expiry = the_option.get_expiry();
    let variance = vol.integral_square(0.0, expiry);
    let root_variance = variance.sqrt();
    let ito_correlation = -0.5 * variance;
    let moved_spot = spot * (r.integral(0.0, expiry) + ito_correlation).exp();
    let discounting = (-r.integral(0.0, expiry)).exp();
    let mut this_spot;
    for _i in 0..number_of_paths {
        let this_gaussian = get_one_gaussian_by_box_muller();
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        let this_payoff = the_option.option_payoff(this_spot);
        gatherer.dump_one_result(this_payoff * discounting);
    }
}
