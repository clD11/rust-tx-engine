use crate::account::Transaction;
use crate::tx_engine::TxEngine;
use csv::Trim::All;
use std::path::PathBuf;
use std::{env, io};

mod account;
mod errors;
mod tx_engine;

/// Reads in the events, sends them for processing and outputs the result
fn main() -> Result<(), io::Error> {
    // read in command line arguments
    let args = parse_args();

    // process the transactions
    let mut tx_engine = TxEngine::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(All)
        .from_path(args.input_file)?;

    for event in rdr.deserialize::<tx_engine::Event>() {
        match event {
            Ok(event) => {
                tx_engine.process(&event);
            }
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
            }
        }
    }

    // write output
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    wtr.write_record(&["client", "available", "held", "total", "locked"])?;

    &tx_engine.accounts.iter().for_each(|(client_id, account)| {
        wtr.serialize((
            client_id,
            account.account_info.available,
            account.account_info.held,
            account.account_info.total(),
            account.account_info.locked,
        ))
        .expect("panic: failed to write account");
    });

    wtr.flush()?;

    Ok(())
}

#[derive(Debug)]
struct Args {
    pub input_file: String,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    Args {
        input_file: args[1].trim().parse().unwrap(),
    }
}
