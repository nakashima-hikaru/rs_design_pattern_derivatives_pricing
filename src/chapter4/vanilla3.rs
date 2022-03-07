use std::rc::Rc;

use crate::chapter4::payoff3::Payoff;
/// コンストラクタの引数がPayoffBridgeになったけど、これでPayoffCallの引数が受け取れるようになった。
/// ただし、毎回クローンするのが遅いので、パラメータは常に参照渡しにすべき。
/// このクローンをいい感じにするとより速くなるかも（Boxポインタとか？）
use crate::chapter4::payoff_bridge::PayoffBridge;

#[derive(Clone)]
pub struct VanillaOption {
    expiry: f64,
    the_payoff: PayoffBridge,
}

impl VanillaOption {
    pub fn new(the_payoff: Rc<dyn Payoff>, expiry: f64) -> VanillaOption {
        VanillaOption {
            expiry,
            the_payoff: PayoffBridge::new(the_payoff),
        }
    }
    pub fn get_expiry(&self) -> f64 {
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff.value(spot)
    }
}

#[test]
fn test_payoff_copy() {
    use crate::chapter4::payoff3;
    let strike = 100.0;
    let expiry = 30.0;
    let spot = 50.0;
    let call_payoff = Rc::new(payoff3::PayoffCall::new(strike));
    let put_payoff = Rc::new(payoff3::PayoffPut::new(strike + 1.0));
    let call_option = VanillaOption::new(call_payoff, expiry);
    let mut copied_call_option = call_option.clone();
    copied_call_option.the_payoff = PayoffBridge::new(put_payoff);
    assert_ne!(
        call_option.the_payoff.value(spot),
        copied_call_option.the_payoff.value(spot)
    );
}
