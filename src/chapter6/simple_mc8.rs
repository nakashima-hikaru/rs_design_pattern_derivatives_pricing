//! gathererを参照とすることによって、この関数内でのgathererの変更を関数の外でも反映されるようにしている。
//! 平均を求める処理が一行で済むようになり、可読性が向上した。
use crate::chapter4::parameters::Parameters;
use crate::chapter4::vanilla3::VanillaOption;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter6::random2::RandomBase;

pub fn simple_montecarlo6(
    the_option: &VanillaOption,
    spot: f64,
    vol: &dyn Parameters,
    r: &dyn Parameters,
    number_of_paths: u64,
    gatherer: &mut dyn StatisticsMC,
    generator: &mut dyn RandomBase,
) {
    generator.reset_dimensionality(1);

    let expiry = the_option.get_expiry();
    let variance = vol.integral_square(0.0, expiry);
    let root_variance = variance.sqrt();
    let ito_correlation = -0.5 * variance;
    let moved_spot = spot * (r.integral(0.0, expiry) + ito_correlation).exp();
    let mut this_spot;
    let discounting = (-r.integral(0.0, expiry)).exp();
    for _i in 0..number_of_paths {
        let variate_array = generator.get_gaussians();
        this_spot = moved_spot * (root_variance * variate_array[0]).exp();
        let this_payoff = the_option.option_payoff(this_spot);
        gatherer.dump_one_result(this_payoff * discounting);
    }
}
