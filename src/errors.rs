use crate::client::Transaction;
use std::result;
use thiserror::Error;

pub type Result<'a, T> = result::Result<T, Error<'a>>;

#[derive(Error, Debug, Clone, PartialEq)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum Error<'a> {
    /// Client error transaction already deposited
    #[error("error transaction already deposited: tx {0}")]
    DepositError(u32),

    /// Client error transaction does not exist"
    #[error("error transaction does not exist: tx {0}")]
    NonExistentTxnError(u32),

    /// Client error insufficient funds for withdrawal
    #[error("error insufficient funds for withdrawal: available {0} requested {1}")]
    InsufficientFundsError(f32, f32),

    /// Client error invalid transaction type for action
    #[error("error transaction must be of type {0} found {1}")]
    InvalidTxnTypeError(&'a str, String),

    /// Client error account locked. Once an account is locked no more actions are permitted
    #[error("error account locked {0}")]
    AccountLockedError(u16),

    /// Client error unsupported account action
    #[error("error unsupported account action {0}")]
    InvalidAccountAction(String),
}
