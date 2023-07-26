//! コンストラクタの引数がPayoffBridgeになったけど、これでPayoffCallの引数が受け取れるようになった。
//! ただし、毎回クローンするのが遅いので、パラメータは常に参照渡しにすべき。
//! このクローンをいい感じにするとより速くなるかも（Boxポインタとか？）

use crate::chapter4::payoff3::Payoff;

pub struct VanillaOption<'a, T: Payoff> {
    expiry: f64,
    the_payoff: &'a T,
}

impl<'a, T: Payoff> VanillaOption<'a, T> {
    pub fn new(the_payoff: &'a T, expiry: f64) -> VanillaOption<'a, T> {
        VanillaOption { expiry, the_payoff }
    }
    pub fn get_expiry(&self) -> f64 {
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff.calculate(spot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chapter4::payoff3::PayoffCall;
    #[test]
    fn test_vanilla_option() {
        let payoff = PayoffCall::new(100.0);
        let option = VanillaOption::new(&payoff, 1.0);

        assert_eq!(option.get_expiry(), 1.0);
        assert_eq!(option.option_payoff(80.0), 0.0);
        assert_eq!(option.option_payoff(100.0), 0.0);
        assert_eq!(option.option_payoff(120.0), 20.0);
    }
}
