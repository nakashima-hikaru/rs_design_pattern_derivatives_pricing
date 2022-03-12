use crate::chapter4::payoff_bridge::PayoffBridge;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use std::rc::Rc;

#[derive(Clone)]
pub struct PathDependentAsian {
    delivery_time: f64,
    the_payoff: Rc<PayoffBridge>,
    number_of_times: u64,
    look_at_times: Vec<f64>,
}

impl PathDependentAsian {
    pub fn new(
        look_at_times: &Vec<f64>,
        delivery_time: f64,
        the_payoff: Rc<PayoffBridge>,
    ) -> PathDependentAsian {
        PathDependentAsian {
            delivery_time,
            the_payoff,
            number_of_times: look_at_times.len() as u64,
            look_at_times: look_at_times.clone(),
        }
    }
}

impl PathDependent for PathDependentAsian {
    fn get_look_at_times(&self) -> &Vec<f64> {
        &self.look_at_times
    }
    fn max_number_of_cash_flow(&self) -> u64 {
        1
    }
    fn possible_cash_flow_times(&self) -> Vec<f64> {
        let mut tmp = vec![1.0];
        tmp[0] = self.delivery_time;
        tmp
    }
    fn cash_flows(&self, spot_values: &[f64], generated_flows: &mut [CashFlow]) -> u64 {
        let sum: f64 = spot_values.iter().sum();
        let mean = sum / self.number_of_times as f64;
        generated_flows[0].time_index = 0;
        generated_flows[0].amount = self.the_payoff.value(mean);
        1
    }
}
