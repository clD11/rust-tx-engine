use csv::Trim::All;
use std::collections::HashMap;
use std::{env, io};

mod client;
mod errors;

fn main() -> Result<(), io::Error> {
    // read in command line arguments

    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output = &args[3];

    // process the transactions

    let mut clients: HashMap<u16, client::Client> = HashMap::new();

    let mut rdr = csv::ReaderBuilder::new().trim(All).from_path(input_path)?;

    for result in rdr.deserialize() {
        let txn: client::Transaction = result?;

        let account = clients
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

    // write output

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    wtr.write_record(&[
        "client",
        "available",
        "held",
        "total",
        "locked",
    ])?;

    for x in clients {
        wtr.serialize(x.1)?;
    }

    wtr.flush()?;

    Ok(())
}

/// logs a client error to centralised logging
fn log(e: errors::Error) {
    // log error -> println!(client error: e)
}

#[test]
fn process() {
    main();
}
