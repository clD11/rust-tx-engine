use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

/// Provides integration tests for different scenarios within the transactor engine.
/// These test run the full flow i.e. provide a file in a given state and assert the result.

const ACCOUNT_HEADERS: &str = "client,available,held,total,locked\n";

#[test]
fn should_deposit_funds() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-tx-engine").unwrap();
    cmd.arg("tests/scenarios/deposits.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCOUNT_HEADERS))
        .stdout(predicate::str::contains("2,15.0,0.0,15.0,false\n"))
        .stdout(predicate::str::contains("1,10.0,0.0,10.0,false\n"));
    Ok(())
}

#[test]
fn should_not_allow_further_actions_after_chargeback() -> Result<(), Box<dyn std::error::Error>> {
    // given a transaction is in a disputed state
    // deposit,1,1,1.0
    // dispute,1,1

    // then a chargeback is called
    // chargeback,1,1

    // than a further deposit is not recognised
    // deposit,1,2,10.0

    let mut cmd = Command::cargo_bin("rust-tx-engine").unwrap();
    cmd.arg("tests/scenarios/chargeback.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCOUNT_HEADERS))
        .stdout(predicate::str::contains("1,0,0,0,true\n"));
    Ok(())
}
