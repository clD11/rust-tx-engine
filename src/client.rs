use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::errors;
use errors::Result;

/// Holds the client transactions and account information.
/// Once created a transactions can be deposited, withdrawn, disputed, resolved and charged back.
/// Accounts will be locked once a chargeback happens. Once an account is locked no more actions are permitted.
/// This code is NOT thread safe and should be used with appropriate concurrency techniques.
#[derive(Debug, Serialize)]
pub(crate) struct Client {
    /// Client ID which is unique per client
    client_id: u16,
    /// Holds current state of the client account
    account: Account,
    /// Holds all the transactions associated with this account referenced by unique tx id
    #[serde(skip_serializing)]
    transactions: HashMap<u32, Transaction>,
}

impl Client {
    pub(crate) fn new(client_id: u16) -> Client {
        Client {
            client_id,
            account: Account {
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: false,
            },
            transactions: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, transaction: Transaction) -> Result<()> {
        if self.account.locked {
            return Err(errors::Error::AccountLockedError(transaction.client));
        }

        if self.transactions.contains_key(&transaction.tx) {
            return Err(errors::Error::DepositError(transaction.tx));
        }

        self.account.total += transaction.amount;
        self.account.available += transaction.amount;
        self.transactions.insert(transaction.tx, transaction);

        Ok(())
    }

    pub fn withdrawal(&mut self, transaction: Transaction) -> Result<()> {
        if self.account.locked {
            return Err(errors::Error::AccountLockedError(transaction.client));
        }

        if self.account.available - &transaction.amount < 0.0 {
            return Err(errors::Error::InsufficientFundsError(
                self.account.available,
                transaction.amount,
            ));
        }

        self.account.total -= &transaction.amount;
        self.account.available -= &transaction.amount;

        Ok(())
    }

    pub fn dispute(&mut self, transaction: Transaction) -> Result<()> {
        if self.account.locked {
            return Err(errors::Error::AccountLockedError(transaction.client));
        }

        if !self.transactions.contains_key(&transaction.tx) {
            return Err(errors::Error::NonExistentTxnError(transaction.tx));
        }

        self.account.available -= &transaction.amount;
        self.account.held += &transaction.amount;

        Ok(())
    }

    pub fn resolve(&mut self, transaction: Transaction) -> Result<()> {
        if self.account.locked {
            return Err(errors::Error::AccountLockedError(transaction.client));
        }

        if transaction.tx_type != TransactionType::Dispute {
            return Err(errors::Error::InvalidTxnTypeError(transaction.tx_type));
        }

        let txn = self.transactions.get(&transaction.tx);
        if txn.is_none() {
            return Err(errors::Error::NonExistentTxnError(transaction.tx));
        }

        self.account.held -= txn.unwrap().amount;
        self.account.available += txn.unwrap().amount;

        Ok(())
    }

    pub fn chargeback(&mut self, transaction: Transaction) -> Result<()> {
        if self.account.locked {
            return Err(errors::Error::AccountLockedError(transaction.client));
        }

        if transaction.tx_type != TransactionType::Dispute {
            return Err(errors::Error::InvalidTxnTypeError(transaction.tx_type));
        }

        let txn = self.transactions.get(&transaction.tx);
        if txn.is_none() {
            return Err(errors::Error::NonExistentTxnError(transaction.tx));
        }

        self.account.held -= txn.unwrap().amount;
        self.account.total -= txn.unwrap().amount;
        self.account.locked = true;

        Ok(())
    }
}

/// Holds account information
#[derive(Debug, Serialize)]
struct Account {
    /// Total funds that are available for trading, staking, withdrawal, etc.
    available: f32,
    /// Total funds that are held for dispute.
    held: f32,
    /// Total funds that are available or held.
    total: f32,
    /// Whether the account is locked. An account is locked if a charge back occurs.
    locked: bool,
}

/// Defines valid transactions types also these are the valid actions that can be
/// performed on an account.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    /// Credits a clients account.
    Deposit,
    /// Debit a clients account.
    Withdrawal,
    /// Dispute an erroneous transaction.
    Dispute,
    /// Represents a resolution to a dispute.
    Resolve,
    /// Chargeback represents a final state of reversing transaction. Calling chargeback will result
    /// in an account being locked and no further actions can be performed for that account.
    Chargeback,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransactionType::Deposit => write!(f, "{0}", "deposit"),
            TransactionType::Withdrawal => write!(f, "{0}", "withdrawal"),
            TransactionType::Dispute => write!(f, "{0}", "dispute"),
            TransactionType::Chargeback => write!(f, "{0}", "chargeback"),
            _ => {
                write!(f, "{0}", "unknown")
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    pub client: u16,
    tx: u32,
    amount: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors;

    #[test]
    fn test_deposit_error() {
        let transaction = new_transaction();

        let mut client = Client::new(1);

        let deposit = client.deposit(transaction.clone());
        assert_eq!(deposit.is_err(), false);

        let actual = client.deposit(transaction.clone());
        assert_eq!(
            actual.unwrap_err(),
            errors::Error::DepositError(transaction.tx)
        )
    }

    #[test]
    fn test_dispute_error() {
        let transaction = new_transaction();

        let mut client = Client::new(1);
        let actual = client.dispute(transaction.clone());

        assert_eq!(
            actual.unwrap_err(),
            errors::Error::NonExistentTxnError(transaction.tx)
        )
    }

    fn new_transaction() -> Transaction {
        let mut rng = rand::thread_rng();
        Transaction {
            tx_type: TransactionType::Dispute,
            client: rng.gen::<u16>(),
            tx: rng.gen::<u32>(),
            amount: rng.gen::<f32>(),
        }
    }
}
