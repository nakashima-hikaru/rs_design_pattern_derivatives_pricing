use crate::chapter10::payoff_registration_error::ErrorType::NotFound;
use crate::chapter10::payoff_registration_error::RegistrationError;
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
    pub fn instance() -> Result<&'static Mutex<PayoffFactory>, RegistrationError> {
        let mut init = false;
        let ret = FACTORY.get_or_init(|| {
            init = true;
            Mutex::new(PayoffFactory::default())
        });
        if init {
            PayoffFactory::register_all_payoffs()?;
        }
        Ok(ret)
    }

    pub fn register_payoff(
        &mut self,
        payoff_id: &str,
        creator_function: Arc<CreatePayoffFunction>,
    ) {
        self.the_creator_functions
            .insert(payoff_id.to_string(), creator_function);
    }

    pub fn create_payoff(
        &self,
        payoff_id: &str,
        strike: f64,
    ) -> Result<Box<dyn Payoff>, RegistrationError> {
        if let Some(creator_function) = self.the_creator_functions.get(payoff_id) {
            Ok(creator_function(strike))
        } else {
            Err(RegistrationError::new(NotFound(payoff_id.to_string())))
        }
    }

    pub fn is_registered(&self, payoff_id: &str) -> bool {
        self.the_creator_functions.contains_key(payoff_id)
    }
}
