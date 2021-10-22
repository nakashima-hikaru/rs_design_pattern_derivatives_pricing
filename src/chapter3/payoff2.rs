use crate::chapter1::random1::get_one_gaussian_by_box_muller;
use rand::SeedableRng;

pub trait Payoff {
    fn forward_value(&self, spot: f64) -> f64;
}
impl dyn Payoff {
    #[allow(dead_code)]
    pub fn simple_montecarlo2(
        the_payoff: &dyn Payoff,
        expiry: f64,
        spot: f64,
        vol: f64,
        r: f64,
        number_of_paths: u32,
    ) -> f64 {
        let variance = vol * vol * expiry;
        let root_variance = variance.sqrt();
        let ito_correlation = -0.5 * variance;
        let moved_spot = spot * (r * expiry + ito_correlation).exp();
        let mut this_spot: f64;
        let mut runnning_sum = 0.0;
        let mut rng = SeedableRng::from_entropy();
        for _i in 0..number_of_paths {
            let this_gaussian: f64 = get_one_gaussian_by_box_muller(&mut rng);
            this_spot = moved_spot * (root_variance * this_gaussian).exp();
            let this_payoff = the_payoff.forward_value(this_spot);
            runnning_sum += this_payoff;
        }
        let mut mean = runnning_sum / number_of_paths as f64;
        mean *= (-r * expiry).exp();
        return mean;
    }
}

pub struct PayoffCall {
    strike: f64,
}
pub struct PayoffPut {
    strike: f64,
}
impl PayoffCall {
    #[allow(dead_code)]
    pub fn new(strike: f64) -> Self {
        Self { strike }
    }
}
impl PayoffPut {
    #[allow(dead_code)]
    pub fn new(strike: f64) -> Self {
        Self { strike }
    }
}

impl Payoff for PayoffCall {
    fn forward_value(&self, spot: f64) -> f64 {
        (spot - self.strike).max(0.0)
    }
}
impl Payoff for PayoffPut {
    fn forward_value(&self, spot: f64) -> f64 {
        (self.strike - spot).max(0.0)
    }
}
