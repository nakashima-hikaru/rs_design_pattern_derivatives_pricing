use crate::chapter4::payoff3::Payoff;
use once_cell::sync::Lazy;
use std::collections::HashMap;

type CreatePayoffFunction = dyn Fn(f64) -> Box<dyn Payoff> + Sync + Send;

#[derive(Default)]
pub struct PayoffFactory {
    the_creator_functions: HashMap<String, Box<CreatePayoffFunction>>,
}

impl PayoffFactory {
    pub fn instance() -> &'static mut PayoffFactory {
        static mut FACTORY: Lazy<PayoffFactory> = Lazy::new(PayoffFactory::default);
        unsafe { &mut FACTORY }
    }

    pub fn register_payoff(
        &mut self,
        payoff_id: String,
        creator_function: Box<CreatePayoffFunction>,
    ) {
        self.the_creator_functions
            .insert(payoff_id, creator_function);
    }

    pub fn create_payoff(&self, payoff_id: &str, strike: f64) -> Option<Box<dyn Payoff>> {
        if let Some(creator_function) = self.the_creator_functions.get(payoff_id) {
            Some(creator_function(strike))
        } else {
            println!("{} is an unknown payoff", payoff_id);
            None
        }
    }
}