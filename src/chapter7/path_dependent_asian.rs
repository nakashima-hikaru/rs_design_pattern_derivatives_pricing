use crate::chapter4::payoff3::Payoff;

use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;

/// Payoff: \frac{1}{length of `look_at_times`} \sum_{t \in `look_at_times`} SpotValue(t)
pub struct PathDependentAsian {
    delivery_time: f64,
    the_payoff: Box<dyn Payoff>,
    number_of_times: u64,
    look_at_times: Vec<f64>,
}

impl PathDependentAsian {
    pub fn new(
        look_at_times: Vec<f64>,
        delivery_time: f64,
        the_payoff: Box<dyn Payoff>,
    ) -> PathDependentAsian {
        PathDependentAsian {
            delivery_time,
            the_payoff,
            number_of_times: look_at_times.len() as u64,
            look_at_times,
        }
    }
}

impl PathDependent for PathDependentAsian {
    fn get_look_at_times(&self) -> &Vec<f64> {
        &self.look_at_times
    }
    fn max_number_of_cash_flows(&self) -> u64 {
        1
    }
    fn possible_cash_flow_times(&self) -> Vec<f64> {
        vec![self.delivery_time]
    }

    fn cash_flows(&self, spot_values: &[f64], generated_flows: &mut [CashFlow]) -> u64 {
        let sum: f64 = spot_values.iter().sum();
        let mean = sum / self.number_of_times as f64;
        generated_flows[0].time_index = 0;
        generated_flows[0].amount = self.the_payoff.forward_value(mean);
        1
    }
}
