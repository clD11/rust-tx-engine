use serde::Serialize;
use std::borrow::Borrow;

use std::error::Error;
use std::io;
use std::process;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex, RwLock};

mod client;
mod errors;

use serde::Deserialize;

/// Actions that can be performed on an account.

/// Credits clients asset account.
const DEPOSIT: &str = "deposit";

/// Debit a clients account.
const WITHDRAWAL: &str = "withdrawal";

/// Dispute an erroneous transaction.
const DISPUTE: &str = "dispute";

/// Represents a resolution to a dispute.
const RESOLVE: &str = "resolve";

/// Chargeback represents a final state of reversing transaction.
/// Calling chargeback will result in an account being locked and no further actions can be performed
/// on that account.
const CHARGEBACK: &str = "chargeback";

fn main() {
    if let Err(e) = synchronous() {
        eprintln!("{}", e);
    }
}

fn synchronous() -> Result<(), io::Error> {
    let mut clients: HashMap<u16, client::Client> = HashMap::new();

    // read from csv IS THIS BUFFERED
    let mut rdr = csv::Reader::from_path("tests/transactions_1.csv")?;

    for result in rdr.deserialize() {
        let txn: client::Transaction = result?;
        println!("{:?}", txn);

        let account = clients
            .entry(txn.client)
            .or_insert(client::Client::new(txn.client));

        match txn.tx_type.as_str() {
            DEPOSIT => account.deposit(txn).unwrap_or_else(|e| log(e)),
            WITHDRAWAL => account.withdrawal(txn).unwrap_or_else(|e| log(e)),
            DISPUTE => account.dispute(txn).unwrap_or_else(|e| log(e)),
            RESOLVE => account.resolve(txn).unwrap_or_else(|e| log(e)),
            CHARGEBACK => account.chargeback(txn).unwrap_or_else(|e| log(e)),
            _ => {
                log(errors::Error::InvalidAccountAction(txn.tx_type))
            }
        }
    }

    for x in clients {
        println!("{:?}", x.1);
    }

    println!("Done");
    Ok(())
}

/// logs client errors to centralised logging
fn log(e: errors::Error) {
    // log error -> println!(client error: e)
}

// fn shared_mem() {
//     //let mut accounts: HashMap<u16, Mutex<client::Client>> = HashMap::new();
//
//     // read from csv IS THIS BUFFERED
//     let mut rdr = csv::Reader::from_path("tests/transactions.csv")?;
//
//     let headers = rdr.headers()?;
//     println!("{:?}", headers);
//
//     for result in rdr.deserialize() {
//         let txn: client::Transaction = result?;
//         println!("{:?}", txn);
//     }
//
//     println!("Done");
//     Ok(())
//
//     // for t in txns {
//     //     let mux_account = accounts.entry(t.client).
//     //         or_insert(Mutex::new(client::Client::new(t.client)));
//     //
//     //     let mut account = mux_account.lock().unwrap();
//     //
//     //     match txn.tx_type.as_str() {
//     //         DEPOSIT => account.deposit(t),
//     //         WITHDRAWAL => account.withdrawal(txn),
//     //         DISPUTE => account.dispute(txn),
//     //         RESOLVE => account.resolve(txn),
//     //         CHARGEBACK => account.chargeback(txn),
//     //         _ => {}
//     //     }
//     // }
//     //
//     // let mut txns = vec![];
//     // txns.push(txn.to_owned());
//     // txns.push(txn1.to_owned());
//     // txns.push(txn2.to_owned());
//     //
//     //
//     // for m in accounts.values() {
//     //     let a = m.lock().unwrap();
//     //     println!("{:?}", a.account.total);
//     //     for (k, t) in &a.transactions {
//     //         println!("{:?}:{:?}", k, t.amount);
//     //     }
//     //     println!();
//     // }
// }
//
// fn mpsc() {}

#[test]
fn process() {
    main();
}
