//! コンストラクタの引数がPayoffBridgeになったけど、これでPayoffCallの引数が受け取れるようになった。
//! ただし、毎回クローンするのが遅いので、パラメータは常に参照渡しにすべき。
//! このクローンをいい感じにするとより速くなるかも（Boxポインタとか？）
use crate::chapter4::payoff_bridge::PayoffBridge;

#[derive(Clone)]
pub struct VanillaOption {
    expiry: f64,
    the_payoff: PayoffBridge,
}

impl VanillaOption {
    pub fn new(the_payoff: &PayoffBridge, expiry: f64) -> VanillaOption {
        VanillaOption {
            expiry,
            the_payoff: the_payoff.clone(),
        }
    }
    pub fn get_expiry(&self) -> f64 {
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff.forward_value(spot)
    }
}
