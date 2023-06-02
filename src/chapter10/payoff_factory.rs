use crate::chapter4::payoff3::Payoff;
use std::{
    collections::HashMap,
    sync::OnceLock,
    sync::{Arc, Mutex},
};

type CreatePayoffFunction = dyn Fn(f64) -> Box<dyn Payoff> + Send + Sync;

static FACTORY: OnceLock<Mutex<PayoffFactory>> = OnceLock::new();

#[derive(Default)]
pub struct PayoffFactory {
    the_creator_functions: HashMap<String, Arc<CreatePayoffFunction>>,
}

impl PayoffFactory {
    pub fn instance() -> &'static Mutex<PayoffFactory> {
        let mut init = false;
        let ret = FACTORY.get_or_init(|| {
            init = true;
            Mutex::new(PayoffFactory::default())
        });
        if init {
            PayoffFactory::register();
        }
        ret
    }

    pub fn register_payoff(
        &mut self,
        payoff_id: String,
        creator_function: Arc<CreatePayoffFunction>,
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
