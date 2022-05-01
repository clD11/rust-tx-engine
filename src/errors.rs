use crate::client::TransactionType;
use std::result;

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug, Clone, PartialEq)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum Error {
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
    #[error("error transaction must be of type deposit found {0}")]
    InvalidTxnTypeError(TransactionType),

    /// Client error account locked. Once an account is locked no more actions are permitted
    #[error("error account locked {0}")]
    AccountLockedError(u16),
}
