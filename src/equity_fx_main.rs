use rust_design_pattern_derivative_pricing::chapter10::payoff_factory::PayoffFactory;
use rust_design_pattern_derivative_pricing::chapter4::parameters::ParametersConstant;
use rust_design_pattern_derivative_pricing::chapter5::convergence_table::ConvergenceTable;
use rust_design_pattern_derivative_pricing::chapter5::mc_statistics::StatisticsMC;
use rust_design_pattern_derivative_pricing::chapter5::mc_statistics::StatisticsMean;
use rust_design_pattern_derivative_pricing::chapter6::anti_thetic::AntiThetic;
use rust_design_pattern_derivative_pricing::chapter6::park_miller::RandomParkMiller;
use rust_design_pattern_derivative_pricing::chapter7::exotic_bs_engine::ExoticBSEngine;
use rust_design_pattern_derivative_pricing::chapter7::exotic_engine::ExoticEngine;
use rust_design_pattern_derivative_pricing::chapter7::path_dependent_asian::PathDependentAsian;

#[allow(clippy::too_many_arguments)]
pub fn price(
    option_type: &str,
    expiry: f64,
    strike: f64,
    spot: f64,
    vol: f64,
    r: f64,
    d: f64,
    number_of_dates: usize,
    number_of_paths: usize,
) -> f64 {
    let payoff_factory = PayoffFactory::instance().unwrap().lock().unwrap();
    let payoff = payoff_factory.create_payoff(option_type, strike);
    if let Some(the_payoff) = payoff {
        let times = (0..number_of_dates)
            .map(|i| (i as f64 + 1.0) * expiry / number_of_dates as f64)
            .collect();
        let vol_param = ParametersConstant::from(vol);
        let r_param = ParametersConstant::from(r);
        let d_param = ParametersConstant::from(d);
        let the_option = PathDependentAsian::new(times, expiry, the_payoff.as_ref());
        let mut gatherer = StatisticsMean::default();
        let mut gatherer_two = ConvergenceTable::new(&mut gatherer);
        let mut generator = RandomParkMiller::new(number_of_dates, 1);
        let mut gen_two = AntiThetic::new(&mut generator);
        let mut the_engine = ExoticBSEngine::new(
            &the_option,
            &r_param,
            d_param,
            vol_param,
            &mut gen_two,
            spot,
        );
        the_engine.do_simulation(&mut gatherer_two, number_of_paths);
        let results = gatherer_two.get_results_so_far();
        println!("\nFor the Asian call price the results are \n");
        for result in &results {
            for data in result {
                print!("{} ", data);
            }
            println!("\n");
        }
        results[0][0]
    } else {
        panic!("payoff does not be found");
    }
}

#[test]
pub fn test_main() {
    let option_type = "call";
    let expiry = 30.0;
    let strike = 100.0;
    let spot = 100.0;
    let vol = 0.01;
    let r = 0.01;
    let d = 0.0;
    let number_of_dates = 1000;
    let number_of_paths = 1000;
    let payoff_factory = PayoffFactory::instance().unwrap().lock().unwrap();
    let payoff = payoff_factory.create_payoff(option_type, strike);
    if let Some(the_payoff) = payoff {
        let times = (0..number_of_dates)
            .map(|i| (i as f64 + 1.0) * expiry / number_of_dates as f64)
            .collect();
        let vol_param = ParametersConstant::from(vol);
        let r_param = ParametersConstant::from(r);
        let d_param = ParametersConstant::from(d);
        let the_option = PathDependentAsian::new(times, expiry, the_payoff.as_ref());
        let mut gatherer = StatisticsMean::default();
        let mut gatherer_two = ConvergenceTable::new(&mut gatherer);
        let mut generator = RandomParkMiller::new(number_of_dates, 1);
        let mut gen_two = AntiThetic::new(&mut generator);
        let mut the_engine = ExoticBSEngine::new(
            &the_option,
            &r_param,
            d_param,
            vol_param,
            &mut gen_two,
            spot,
        );
        the_engine.do_simulation(&mut gatherer_two, number_of_paths);
        let results = gatherer_two.get_results_so_far();
        assert_eq!(
            results,
            [
                [12.297472829420853, 2.0],
                [12.289559897724873, 4.0],
                [12.319043908132263, 8.0],
                [12.320459101851085, 16.0],
                [12.330259037906742, 32.0],
                [12.326630351283598, 64.0],
                [12.328747463751963, 128.0],
                [12.322480274711126, 256.0],
                [12.322134425092136, 512.0],
                [12.321899356254411, 1000.0]
            ]
        )
    } else {
        panic!("payoff does not be found");
    }
}
