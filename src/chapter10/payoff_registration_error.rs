use crate::chapter10::payoff_constructible::ErrorType::DuplicateError;
use crate::chapter10::payoff_constructible::RegistrationError;
use crate::chapter10::payoff_factory::PayoffFactory;
use crate::chapter4::payoff3::{Payoff, PayoffCall, PayoffPut};
use std::sync::Arc;

impl PayoffFactory {
    fn register<T: Payoff + 'static>() -> Result<(), RegistrationError> {
        let factory = PayoffFactory::instance()?.lock()?;
        let payoff_id = T::name();
        if factory.is_registered(&payoff_id) {
            return Err(RegistrationError::new(DuplicateError(payoff_id)));
        }
        let mut factory = factory;
        factory.register_payoff(&payoff_id, Arc::new(|strike| Box::<T>::new(T::new(strike))));
        Ok(())
    }

    pub fn register_all_payoffs() -> Result<(), RegistrationError> {
        Self::register::<PayoffCall>()?;
        Self::register::<PayoffPut>()?;
        Ok(())
    }
}
