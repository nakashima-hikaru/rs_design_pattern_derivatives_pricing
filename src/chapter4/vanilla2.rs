//! vanilla.rsからの変更点: Clone trait, Copy traitの実装
//! Payoffオブジェクトを値渡しに変更したため、vanilla.rsで挙げた欠点(Payoffオブジェクトが外部で変更される危険性)は解消された。
//! 三原則
//! ・Assignment (Clone trait)
//! a=b=cのような用法、自己代入を考慮して戻り値が&VanillaOptionであることに注意。
//! ・Construction (Copy trait)
//! ・Destruction (Copy traitを実装する場合は不要)
//! 欠点
//! ・VanillaOptionと同様のstructを定義するたびに毎回newやget_expiryなどの同じコードを書く必要がある。
use crate::chapter4::payoff3::Payoff;

#[derive(Clone, Copy)]
pub struct VanillaOption<'a> {
    expiry: f64,
    the_payoff_ptr: &'a dyn Payoff,
}

impl<'a> VanillaOption<'a> {
    pub fn new(the_payoff_ptr: &'a dyn Payoff, expiry: f64) -> Self {
        Self {
            the_payoff_ptr: the_payoff_ptr.clone(),
            expiry,
        }
    }
    pub fn get_expiry(&self) -> f64 {
        self.the_payoff_ptr;
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff_ptr.forward_value(spot)
    }
}

#[test]
fn main() {
    use crate::chapter4::payoff3::PayoffCall;
    let payoff1 = PayoffCall::new(105.0);
    let mut option1 = VanillaOption::new(&payoff1, 30.0);
    let payoff2 = PayoffCall::new(110.0);
    let option2 = VanillaOption::new(&payoff2, 315.0);
    option1.clone_from(&option2);
}
