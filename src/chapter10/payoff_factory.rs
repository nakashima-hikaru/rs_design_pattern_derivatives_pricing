use crate::chapter4::payoff3::Payoff;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

type CreatePayoffFunction = dyn Fn(f64) -> Box<dyn Payoff> + Send + Sync;

pub static FACTORY: Lazy<Mutex<PayoffFactory>> = Lazy::new(|| Mutex::new(PayoffFactory::default()));

#[derive(Default)]
pub struct PayoffFactory {
    the_creator_functions: HashMap<String, Arc<RwLock<CreatePayoffFunction>>>,
}

impl PayoffFactory {
    pub fn instance() -> &'static Lazy<Mutex<PayoffFactory>> {
        &FACTORY
    }

    pub fn register_payoff(
        &mut self,
        payoff_id: String,
        creator_function: Arc<RwLock<CreatePayoffFunction>>,
    ) {
        self.the_creator_functions
            .insert(payoff_id, creator_function);
    }

    pub fn create_payoff(&self, payoff_id: &str, strike: f64) -> Option<Box<dyn Payoff>> {
        if let Some(creator_function) = self.the_creator_functions.get(payoff_id) {
            Some(creator_function.read().unwrap()(strike))
        } else {
            None
        }
    }
}
