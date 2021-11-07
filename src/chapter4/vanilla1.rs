/// オプションの情報をexpiryとPayoffの情報として設計し直した。
/// Payoffはtraitなのでそのままメンバ変数にすることはできない。
/// よってメンバ変数としてポインタや参照を用いる。
/// 欠点
/// ・VanillaOption-structの外部で定義されているPayoff-structのオブジェクトをメンバに持っているため、
/// VanillaOptionオブジェクトは外部でのPayoffの変更の影響を受けてしまう。
use crate::chapter3::payoff2::Payoff;
pub struct VanillaOption<'a> {
    expiry: f64,
    the_payoff: &'a dyn Payoff,
}

impl<'a> VanillaOption<'a> {
    pub fn new(the_payoff: &'a dyn Payoff, expiry: f64) -> Self {
        VanillaOption { the_payoff, expiry }
    }
    pub fn get_expiry(&self) -> f64 {
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff.value(spot)
    }
}
