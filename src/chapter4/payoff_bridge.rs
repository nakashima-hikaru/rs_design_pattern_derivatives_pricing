/// vanilla2.rsからメンバ変数expiryとメソッドget_expiryを取り除いた。
use crate::chapter4::payoff3::Payoff;
use std::rc::Rc;
#[derive(Clone)]
pub struct PayoffBridge {
    the_payoff_ptr: Rc<dyn Payoff>,
}

impl PayoffBridge {
    pub fn new(inner_payoff: Rc<dyn Payoff>) -> PayoffBridge {
        PayoffBridge {
            the_payoff_ptr: inner_payoff,
        }
    }

    pub fn value(&self, spot: f64) -> f64 {
        self.the_payoff_ptr.value(spot)
    }
}
