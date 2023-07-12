use crate::chapter10::payoff_factory::PayoffFactory;
use std::fmt::Debug;
use std::sync::{MutexGuard, PoisonError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistrationError {
    #[error("The Payoff {0} is not found in the factory")]
    NotFound(String),
    #[error("The payoff {0} is already registered")]
    DuplicateError(String),
    #[error("some")]
    PoisonError(#[from] PoisonError<MutexGuard<'static, PayoffFactory>>),
}
