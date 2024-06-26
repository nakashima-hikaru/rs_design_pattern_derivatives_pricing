use crate::chapter10::payoff_registration_error::FactoryError;
use crate::chapter10::payoff_registration_error::FactoryError::{DuplicateError, NotFound};
use crate::chapter4::payoff3::{Payoff, PayoffCall, PayoffPut};
use std::{collections::HashMap, sync::OnceLock};

type CreatePayoffFunction = fn(f64) -> Box<dyn Payoff>;

static FACTORY: OnceLock<PayoffFactory> = OnceLock::new();

#[derive(Default, Debug)]
pub struct PayoffFactory {
    the_creator_functions: HashMap<&'static str, CreatePayoffFunction>,
}

impl PayoffFactory {
    pub fn instance() -> Result<&'static PayoffFactory, FactoryError> {
        FACTORY.get_or_try_init(|| {
            let mut val = Self::default();
            val.register_all_payoffs()?;
            Ok(val)
        })
    }

    pub fn create_payoff(
        &self,
        payoff_id: &str,
        strike: f64,
    ) -> Result<Box<dyn Payoff>, FactoryError> {
        if let Some(creator_function) = self.the_creator_functions.get(payoff_id) {
            Ok(creator_function(strike))
        } else {
            Err(NotFound(format!(
                "The payoff with id: {} not found.",
                payoff_id
            )))
        }
    }

    fn register<T: Payoff + 'static>(&mut self) -> Result<(), FactoryError> {
        let payoff_id = T::name();
        if self.the_creator_functions.contains_key(payoff_id) {
            return Err(DuplicateError(payoff_id.to_string()));
        }
        self.the_creator_functions
            .insert(payoff_id, |strike| Box::<T>::new(T::new(strike)));
        Ok(())
    }

    fn register_all_payoffs(&mut self) -> Result<(), FactoryError> {
        self.register::<PayoffCall>()?;
        self.register::<PayoffPut>()?;
        Ok(())
    }
}
