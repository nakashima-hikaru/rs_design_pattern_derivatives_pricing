use crate::chapter3::payoff2::Payoff;
use std::rc::Rc;
pub struct VanillaOption {
    expiry: f64,
    the_payoff: Rc<dyn Payoff>,
}

#[allow(dead_code)] // ?
impl VanillaOption {
    pub fn new(the_payoff: Rc<dyn Payoff>, expiry: f64) -> Self {
        VanillaOption { the_payoff, expiry }
    }
    pub fn get_expiry(&self) -> f64 {
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff.forward_value(spot)
    }
}
