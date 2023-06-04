use crate::chapter10::payoff_constructible::ErrorType::DuplicateError;
use crate::chapter10::payoff_factory::PayoffFactory;
use crate::chapter4::payoff3::Payoff;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::{Arc, PoisonError};

#[derive(Debug)]
pub struct RegistrationError {
    _error_type: ErrorType,
}

impl Error for RegistrationError {}

impl RegistrationError {
    pub fn new(error_type: ErrorType) -> Self {
        Self {
            _error_type: error_type,
        }
    }
}

#[derive(Debug)]
pub enum ErrorType {
    DuplicateError(String),
    PoisonError(Box<dyn Error>),
}

impl<T: 'static> From<PoisonError<T>> for RegistrationError {
    fn from(e: PoisonError<T>) -> Self {
        RegistrationError::new(ErrorType::PoisonError(e.into()))
    }
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self._error_type {
            ErrorType::DuplicateError(s) => {
                write!(f, "The payoff {} is already registered", s)
            }
            ErrorType::PoisonError(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

#[derive(Debug)]
pub struct PayoffHelper<T: Payoff> {
    _phantom: PhantomData<T>,
}

impl<T: 'static + Payoff> PayoffHelper<T> {
    fn create(strike: f64) -> Box<dyn Payoff> {
        Box::<T>::new(T::new(strike))
    }

    pub fn new() -> Result<Self, RegistrationError> {
        let factory = PayoffFactory::instance()?.lock()?;
        let payoff_id = T::name();
        if factory.is_registered(&payoff_id) {
            return Err(RegistrationError::new(DuplicateError(payoff_id)));
        }
        let mut factory = factory;
        factory.register_payoff(&payoff_id, Arc::new(PayoffHelper::<T>::create));
        Ok(Self {
            _phantom: PhantomData,
        })
    }
}
