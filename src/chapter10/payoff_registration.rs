use crate::chapter10::payoff_constructible::{PayoffHelper, RegistrationError};
use crate::chapter10::payoff_factory::PayoffFactory;
use crate::chapter4::payoff3::{PayoffCall, PayoffPut};

impl PayoffFactory {
    pub fn register() -> Result<(), RegistrationError> {
        PayoffHelper::<PayoffCall>::new()?;
        PayoffHelper::<PayoffPut>::new()?;
        Ok(())
    }
}
