use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("The Payoff {0} is not found in the factory")]
    NotFound(String),
    #[error("The payoff {0} is already registered")]
    DuplicateError(String),
}
