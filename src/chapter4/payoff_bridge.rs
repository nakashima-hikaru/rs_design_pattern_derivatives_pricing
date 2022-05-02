//! vanilla2.rsからメンバ変数expiryとメソッドget_expiryを取り除いた。
use crate::chapter4::payoff3::Payoff;

pub struct PayoffBridge {
    the_payoff_ptr: Box<dyn Payoff>,
}

impl PayoffBridge {
    pub fn new(inner_payoff: Box<dyn Payoff>) -> PayoffBridge {
        PayoffBridge {
            the_payoff_ptr: inner_payoff,
        }
    }

    pub fn forward_value(&self, spot: f64) -> f64 {
        self.the_payoff_ptr.forward_value(spot)
    }
}
