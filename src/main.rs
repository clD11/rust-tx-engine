use csv::Trim::All;

use crate::tx_engine::TxEngine;
use std::{env, io};

mod client;
mod errors;
mod tx_engine;

/// Takes filepath input and output params
fn main() -> Result<(), io::Error> {
    // read in command line arguments
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];

    // process the transactions
    let mut tx_engine = TxEngine::new();
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(All)
        .from_path(input_path)?;
    for result in rdr.deserialize() {
        let txn: client::Transaction = result?;
        tx_engine.process(txn);
    }

    // write output
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    wtr.write_record(&["client", "available", "held", "total", "locked"])?;
    for x in &tx_engine.clients {
        wtr.serialize(x.1)?;
    }
    wtr.flush()?;

    Ok(())
}

#[test]
fn process() {
    main();
}
