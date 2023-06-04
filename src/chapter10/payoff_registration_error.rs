use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::sync::PoisonError;

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
    NotFound(String),
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
            ErrorType::NotFound(s) => {
                write!(f, "The Payoff {} is not found in the factory", s)
            }
            ErrorType::DuplicateError(s) => {
                write!(f, "The payoff {} is already registered", s)
            }
            ErrorType::PoisonError(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}
