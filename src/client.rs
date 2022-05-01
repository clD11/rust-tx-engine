use rand::Rng;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::format;

use crate::{errors, DEPOSIT, DISPUTE};

use errors::Result;

/// Holds the client transactions and account information.
/// Once created a transactions can be deposited, withdrawn, disputed, resolved and charged back.
/// Accounts can be locked once a chargeback happens. Once an account is locked no more actions are permitted.
/// This code is not thread safe and should be used with appropriate concurrency techniques.
#[derive(Debug)]
pub(crate) struct Client {
    client_id: u16,
    account: Account,
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

        if transaction.tx_type != DISPUTE {
            return Err(errors::Error::InvalidTxnTypeError(
                DEPOSIT,
                transaction.tx_type,
            ));
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

        if transaction.tx_type != DISPUTE {
            return Err(errors::Error::InvalidTxnTypeError(
                DISPUTE,
                transaction.tx_type,
            ));
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

#[derive(Debug)]
struct Account {
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: u16,
    tx: u32,
    amount: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEPOSIT;

    #[test]
    fn test_deposit_error() {
        let transaction = new_transaction();

        let mut client = Client::new(1);
        client.deposit(transaction.clone());

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
            tx_type: DEPOSIT.parse().unwrap(),
            client: rng.gen::<u16>(),
            tx: rng.gen::<u32>(),
            amount: rng.gen::<f32>(),
        }
    }
}
