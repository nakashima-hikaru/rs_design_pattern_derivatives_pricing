use crate::chapter4::payoff_bridge::PayoffBridge;
use crate::chapter7::path_dependent::PathDependent;

#[derive(Clone)]
struct PathDependentAsian {
    delivery_time: f64,
    the_payoff: Box<PayoffBridge>,
    number_of_times: u64,
}
