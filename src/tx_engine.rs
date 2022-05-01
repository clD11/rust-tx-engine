use crate::{client, errors};
use std::collections::HashMap;

/// Holds the clients and processes the transactions.
/// This code is NOT thread safe and should be used with appropriate concurrency techniques.
pub struct TxEngine {
    pub(crate) clients: HashMap<u16, client::Client>,
}

impl TxEngine {
    pub(crate) fn new() -> TxEngine {
        TxEngine {
            clients: HashMap::new(),
        }
    }

    pub(crate) fn process(&mut self, txn: client::Transaction) {
        let account = self
            .clients
            .entry(txn.client)
            .or_insert(client::Client::new(txn.client));

        match txn.tx_type {
            client::TransactionType::Deposit => account.deposit(txn).unwrap_or_else(|e| log(e)),
            client::TransactionType::Withdrawal => {
                account.withdrawal(txn).unwrap_or_else(|e| log(e))
            }
            client::TransactionType::Dispute => account.dispute(txn).unwrap_or_else(|e| log(e)),
            client::TransactionType::Resolve => account.resolve(txn).unwrap_or_else(|e| log(e)),
            client::TransactionType::Chargeback => {
                account.chargeback(txn).unwrap_or_else(|e| log(e))
            }
        }
    }
}

/// logs a client error to centralised logging
fn log(e: errors::Error) {
    // log error -> println!(client error: e)
}
