//! コンストラクタの引数がPayoffBridgeになったけど、これでPayoffCallの引数が受け取れるようになった。
//! ただし、毎回クローンするのが遅いので、パラメータは常に参照渡しにすべき。
//! このクローンをいい感じにするとより速くなるかも（Boxポインタとか？）

use crate::chapter4::payoff3::Payoff;

pub struct VanillaOption<'a> {
    expiry: f64,
    the_payoff: &'a dyn Payoff,
}

impl<'a> VanillaOption<'a> {
    pub fn new(the_payoff: &'a impl Payoff, expiry: f64) -> VanillaOption<'a> {
        VanillaOption { expiry, the_payoff }
    }
    pub fn get_expiry(&self) -> f64 {
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff.forward_value(spot)
    }
}
