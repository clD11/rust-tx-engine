use std::result;
use rust_decimal::Decimal;

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug, Clone, PartialEq)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum Error {

    #[error("error transaction already deposited: tx {0}")]
    DepositError(u32),

    #[error("error transaction already disputed: tx {0}")]
    DisputeError(u32),

    #[error("error transaction does not exist: tx {0}")]
    NonExistentTxnError(u32),

    #[error("error insufficient funds for withdrawal: available {0} requested {1}")]
    InsufficientFundsError(Decimal, Decimal),

    #[error("error transaction {0} is not in the dispute state")]
    InvalidAccountAction(u32),

    #[error("error account locked {0}")]
    AccountLockedError(u16),
}
