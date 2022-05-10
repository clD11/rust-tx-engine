use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub};
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

use crate::errors;
use errors::Result;

pub(crate) struct Account {
    client_id: u16,
    pub account_info: AccountInfo,
    deposits: HashMap<u32, Transaction>,
}

type Disputed = bool;

impl Account {
    pub(crate) fn new(client_id: u16) -> Account {
        Account {
            client_id,
            account_info: AccountInfo {
                available: Decimal::default(),
                held: Decimal::default(),
                locked: false,
            },
            deposits: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, transaction: Transaction) -> Result<()> {
        if self.account_info.locked {
            return Err(errors::Error::AccountLockedError(self.client_id));
        }

        if self.deposits.contains_key(&transaction.tx) {
            return Err(errors::Error::DepositError(transaction.tx));
        }

        self.account_info.available += &transaction.amount.unwrap();
        self.deposits.insert(transaction.tx, transaction);

        Ok(())
    }

    pub fn withdrawal(&mut self, transaction: Transaction) -> Result<()> {
        if self.account_info.locked {
            return Err(errors::Error::AccountLockedError(self.client_id));
        }

        if self.account_info.available - &transaction.amount.unwrap() < Decimal::zero() {
            return Err(errors::Error::InsufficientFundsError(
                self.account_info.available,
                transaction.amount.unwrap(),
            ));
        }

        self.account_info.available -= &transaction.amount.unwrap();

        Ok(())
    }

    pub fn dispute(&mut self, transaction: Transaction) -> Result<()> {
        if self.account_info.locked {
            return Err(errors::Error::AccountLockedError(self.client_id));
        }
        match self.deposits.get_mut(&transaction.tx) {
            Some(txn) => {
                if txn.disputed {
                    return Err(errors::Error::DisputeError(transaction.tx));
                }

                self.account_info.available -= &txn.amount.unwrap();
                self.account_info.held += &txn.amount.unwrap();
                txn.disputed = true;

                Ok(())
            }
            _ => return Err(errors::Error::NonExistentTxnError(transaction.tx)),
        }
    }

    pub fn resolve(&mut self, transaction: Transaction) -> Result<()> {
        if self.account_info.locked {
            return Err(errors::Error::AccountLockedError(self.client_id));
        }
        match self.deposits.get_mut(&transaction.tx) {
            Some(txn) => {
                if !txn.disputed {
                    return Err(errors::Error::DisputeError(transaction.tx));
                }

                self.account_info.held -= &txn.amount.unwrap();
                self.account_info.available += &txn.amount.unwrap();
                txn.disputed = false;

                Ok(())
            }
            _ => return Err(errors::Error::NonExistentTxnError(transaction.tx)),
        }
    }

    pub fn chargeback(&mut self, transaction: Transaction) -> Result<()> {
        if self.account_info.locked {
            return Err(errors::Error::AccountLockedError(self.client_id));
        }
        match self.deposits.get(&transaction.tx) {
            Some(txn) => {
                if !txn.disputed {
                    return Err(errors::Error::InvalidAccountAction(transaction.tx));
                }
                self.account_info.held -= &txn.amount.unwrap();
                self.account_info.locked = true;
                Ok(())
            }
            _ => return Err(errors::Error::NonExistentTxnError(transaction.tx)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountInfo {
    pub available: Decimal,
    pub held: Decimal,
    pub locked: bool,
}

impl AccountInfo {
    pub fn total(&self) -> Decimal {
        self.available + self.held
    }
}

pub(crate) struct Transaction {
    tx: u32,
    amount: Option<Decimal>,
    disputed: Disputed,
}

impl Transaction {
    pub fn new(tx: u32, amount: Option<Decimal>) -> Transaction {
        Transaction {
            tx,
            amount,
            disputed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use rand::{thread_rng, Rng};
    use super::*;
    use crate::errors;

    #[test]
    fn test_deposit() {
        let tx = thread_rng().gen::<u32>();
        let amount = Decimal::try_new(thread_rng().gen::<i64>(), 0).unwrap();

        let transaction = new_transaction(tx, Some(amount));

        let mut account = Account::new(1);

        let deposit = account.deposit(transaction);

        assert_eq!(account.deposits.contains_key(&tx), true);
        assert_eq!(account.account_info.available, amount);
    }

    #[test]
    fn test_withdrawal() {
        let mut account = Account::new(1);

        let tx_deposit = thread_rng().gen::<u32>();
        let t_deposit = new_transaction(tx_deposit, Some(Decimal::new(10, 0)));

        let deposit = account.deposit(t_deposit);

        let tx_withdrawal = thread_rng().gen::<u32>();
        let t_withdrawal = new_transaction(tx_withdrawal, Some(Decimal::new(5, 0)));

        let withdrawal = account.withdrawal(t_withdrawal);

        assert_eq!(account.deposits.contains_key(&tx_deposit), true);
        assert_ne!(account.deposits.contains_key(&tx_withdrawal), true);
        assert_eq!(account.account_info.available, Decimal::new(5, 4));
        assert_eq!(account.account_info.total(), Decimal::new(5, 4));
    }

    #[test]
    fn test_dispute() {
        let mut account = Account::new(1);

        let tx_deposit = thread_rng().gen::<u32>();
        let t_deposit = new_transaction(tx_deposit, Some(Decimal::new(10, 0)));

        let deposit = account.deposit(t_deposit);

        let t_dispute = new_transaction(tx_deposit, None);
        let dispute = account.dispute(t_dispute);

        assert_eq!(account.deposits.contains_key(&tx_deposit), true);
        assert_eq!(account.account_info.available, Decimal::new(0, 0));
        assert_eq!(account.account_info.held, Decimal::new(10, 0));
        assert_eq!(account.account_info.total(), Decimal::new(10, 0));
    }

    #[test]
    fn test_resolve() {
        let mut account = Account::new(1);

        let tx_deposit = thread_rng().gen::<u32>();
        let t_deposit = new_transaction(tx_deposit, Some(Decimal::new(10, 0)));

        let deposit = account.deposit(t_deposit);

        let t_dispute = new_transaction(tx_deposit, None);
        let dispute = account.dispute(t_dispute);

        let t_resolve = new_transaction(tx_deposit, None);
        let dispute = account.resolve(t_resolve);

        assert_eq!(account.deposits.contains_key(&tx_deposit), true);
        assert_eq!(account.account_info.available, Decimal::new(10, 0));
        assert_eq!(account.account_info.held, Decimal::new(0, 0));
        assert_eq!(account.account_info.total(), Decimal::new(10, 0));
    }

    #[test]
    fn test_chargeback() {
        let mut account = Account::new(1);

        let tx_deposit = thread_rng().gen::<u32>();
        let t_deposit = new_transaction(tx_deposit, Some(Decimal::new(10, 0)));

        let deposit = account.deposit(t_deposit);

        let t_dispute = new_transaction(tx_deposit, None);
        let dispute = account.dispute(t_dispute);

        let t_resolve = new_transaction(tx_deposit, None);
        let dispute = account.chargeback(t_resolve);

        assert_eq!(account.deposits.contains_key(&tx_deposit), true);
        assert_eq!(account.account_info.held, Decimal::new(0, 0));
        assert_eq!(account.account_info.total(), Decimal::new(0, 0));
        assert_eq!(account.account_info.locked, true);
    }

    fn new_transaction(tx: u32, amount: Option<Decimal>) -> Transaction {
        Transaction {
            tx,
            amount,
            disputed: false
        }
    }
}
