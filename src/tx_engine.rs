use crate::{account, errors, Transaction};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

pub struct TxEngine {
    pub(crate) accounts: HashMap<u16, account::Account>,
}

impl TxEngine {
    pub(crate) fn new() -> TxEngine {
        TxEngine {
            accounts: HashMap::new(),
        }
    }

    pub(crate) fn process(&mut self, event: &Event) {
        let account = self
            .accounts
            .entry(event.client)
            .or_insert(account::Account::new(event.client));

        let txn: Transaction = event.into();

        match event.event_type {
            EventType::Deposit => account.deposit(txn).unwrap_or_else(|e| log(e)),
            EventType::Withdrawal => account.withdrawal(txn).unwrap_or_else(|e| log(e)),
            EventType::Dispute => account.dispute(txn).unwrap_or_else(|e| log(e)),
            EventType::Resolve => account.resolve(txn).unwrap_or_else(|e| log(e)),
            EventType::Chargeback => account.chargeback(txn).unwrap_or_else(|e| log(e)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Event {
    #[serde(rename = "type")]
    event_type: EventType,
    client: u16,
    tx: u32,
    amount: Option<Decimal>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

/// Maps events to transactions
impl From<&Event> for Transaction {
    fn from(event: &Event) -> Self {
        match event.event_type {
            EventType::Deposit | EventType::Withdrawal => {
                let amount = event.amount.map(|mut num| {
                    num.rescale(4);
                    num
                });
                Transaction::new(event.tx, amount)
            }
            EventType::Dispute | EventType::Resolve | EventType::Chargeback => {
                Transaction::new(event.tx, None)
            }
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EventType::Deposit => write!(f, "{0}", "deposit"),
            EventType::Withdrawal => write!(f, "{0}", "withdrawal"),
            EventType::Dispute => write!(f, "{0}", "dispute"),
            EventType::Chargeback => write!(f, "{0}", "chargeback"),
            _ => {
                write!(f, "{0}", "unknown")
            }
        }
    }
}

// Would normally send error to logs
fn log(_e: errors::Error) {
    //println!("client error: {:?}", e);
}
