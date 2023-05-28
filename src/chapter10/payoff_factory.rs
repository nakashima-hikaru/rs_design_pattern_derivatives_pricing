use crate::chapter10::payoff_registration::register_all_payoffs;
use crate::chapter4::payoff3::Payoff;
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

type CreatePayoffFunction = dyn Fn(f64) -> Box<dyn Payoff> + Send + Sync;

pub static FACTORY: OnceCell<Mutex<PayoffFactory>> = OnceCell::new();

#[derive(Default)]
pub struct PayoffFactory {
    the_creator_functions: HashMap<String, Arc<RwLock<CreatePayoffFunction>>>,
}

impl PayoffFactory {
    pub fn instance() -> &'static Mutex<PayoffFactory> {
        let mut init = false;
        let ret = FACTORY.get_or_init(|| {
            init = true;
            Mutex::new(PayoffFactory::default())
        });
        if init {
            register_all_payoffs();
        }
        ret
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
            println!("{} is an unknown payoff", payoff_id);
            None
        }
    }
}
