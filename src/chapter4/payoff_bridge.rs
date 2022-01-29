/// vanilla2.rsからメンバ変数expiryとメソッドget_expiryを取り除いた。
use crate::chapter4::payoff3::Payoff;
#[derive(Clone, Copy)]
pub struct PayoffBridge<'a> {
    the_payoff_ptr: &'a dyn Payoff,
}

impl<'a> PayoffBridge<'a> {
    pub fn new(inner_payoff: &dyn Payoff) -> PayoffBridge {
        PayoffBridge {
            the_payoff_ptr: inner_payoff,
        }
    }

    pub fn value(&self, spot: f64) -> f64 {
        self.the_payoff_ptr.value(spot)
    }
}
