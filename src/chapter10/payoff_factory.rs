use crate::chapter10::payoff_registration_error::RegistrationError;
use crate::chapter10::payoff_registration_error::RegistrationError::{DuplicateError, NotFound};
use crate::chapter4::payoff3::{Payoff, PayoffCall, PayoffPut};
use std::{collections::HashMap, sync::Mutex, sync::OnceLock};

type CreatePayoffFunction = fn(f64) -> Box<dyn Payoff>;

static FACTORY: OnceLock<Mutex<PayoffFactory>> = OnceLock::new();

#[derive(Default)]
pub struct PayoffFactory {
    the_creator_functions: HashMap<String, CreatePayoffFunction>,
}

impl PayoffFactory {
    pub fn instance() -> Result<&'static Mutex<PayoffFactory>, RegistrationError> {
        let mut init = false;
        let ret = FACTORY.get_or_init(|| {
            init = true;
            PayoffFactory::default().into()
        });
        if init {
            PayoffFactory::register_all_payoffs()?;
        }
        Ok(ret)
    }

    pub fn create_payoff(
        &self,
        payoff_id: &str,
        strike: f64,
    ) -> Result<Box<dyn Payoff>, RegistrationError> {
        if let Some(creator_function) = self.the_creator_functions.get(payoff_id) {
            Ok(creator_function(strike))
        } else {
            Err(NotFound(format!(
                "The payoff with id: {} not found.",
                payoff_id
            )))
        }
    }

    fn register<T: Payoff + 'static>() -> Result<(), RegistrationError> {
        let factory = PayoffFactory::instance()?.lock()?;
        let payoff_id = T::name();
        if factory.the_creator_functions.contains_key(payoff_id) {
            return Err(DuplicateError(payoff_id.to_string()));
        }
        let mut factory = factory;
        factory
            .the_creator_functions
            .insert(payoff_id.to_string(), |strike| {
                Box::<T>::new(T::new(strike))
            });
        Ok(())
    }

    fn register_all_payoffs() -> Result<(), RegistrationError> {
        Self::register::<PayoffCall>()?;
        Self::register::<PayoffPut>()?;
        Ok(())
    }
}
