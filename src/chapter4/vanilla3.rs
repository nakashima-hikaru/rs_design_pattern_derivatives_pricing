use crate::chapter4::payoff3::Payoff;
/// コンストラクタの引数がPayoffBridgeになったけど、これでPayoffCallの引数が受け取れるようになった。
/// ただし、毎回クローンするのが遅いので、パラメータは常に参照渡しにすべき。
/// このクローンをいい感じにするとより速くなるかも（Boxポインタとか？）
use crate::chapter4::payoff_bridge::PayoffBridge;

#[derive(Clone, Copy)]
pub struct VanillaOption<'a> {
    expiry: f64,
    the_payoff: PayoffBridge<'a>,
}

impl<'a> VanillaOption<'a> {
    pub fn new(the_payoff: &dyn Payoff, expiry: f64) -> VanillaOption {
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

    #[cfg(test)]
    fn change_the_payoff(&mut self, new_payoff: &'a dyn Payoff) {
        self.the_payoff = PayoffBridge::new(new_payoff);
    }
}

#[test]
fn test_payoff_copy() {
    use crate::chapter4::payoff3;
    let strike = 100.0;
    let expiry = 30.0;
    let spot = 50.0;
    let call_payoff = payoff3::PayoffCall::new(strike);
    let put_payoff = payoff3::PayoffPut::new(strike + 1.0);
    let call_option = VanillaOption::new(&call_payoff, expiry);
    let mut copied_call_option = call_option;
    copied_call_option.change_the_payoff(&put_payoff);
    println!(
        "{} {}",
        call_option.the_payoff.option_payoff(spot),
        copied_call_option.the_payoff.option_payoff(spot)
    );
    assert_ne!(
        call_option.the_payoff.option_payoff(spot),
        copied_call_option.the_payoff.option_payoff(spot)
    );
}
