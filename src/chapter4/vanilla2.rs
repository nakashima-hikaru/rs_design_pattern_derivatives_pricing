use crate::chapter4::payoff3::Payoff;
use std::rc::Rc;

pub struct VanillaOption {
    expiry: f64,
    the_payoff_ptr: Rc<dyn Payoff>,
}

impl VanillaOption {
    pub fn new(the_payoff_ptr: Rc<dyn Payoff>, expiry: f64) -> Self {
        VanillaOption {
            the_payoff_ptr: the_payoff_ptr.clone(),
            expiry,
        }
    }
    pub fn from(original: &VanillaOption) -> Self {
        Self::new(original.the_payoff_ptr.clone(), original.expiry)
    }
    pub fn get_expiry(&self) -> f64 {
        self.the_payoff_ptr.as_ref();
        self.expiry
    }
    pub fn option_payoff(&self, spot: f64) -> f64 {
        self.the_payoff_ptr.value(spot)
    }
}
impl Clone for VanillaOption {
    fn clone(&self) -> Self {
        VanillaOption::from(self)
    }
}

impl Drop for VanillaOption {
    fn drop(&mut self) {
        drop(&self.the_payoff_ptr);
    }
}

#[test]
fn main() {
    use crate::chapter4::payoff3::PayoffCall;
    let payoff1 = PayoffCall::new(105.0);
    let mut option1 = VanillaOption::new(Rc::new(payoff1), 30.0);
    let payoff2 = PayoffCall::new(110.0);
    let option2 = VanillaOption::new(Rc::new(payoff2), 315.0);
    option1.clone_from(&option2);
}
