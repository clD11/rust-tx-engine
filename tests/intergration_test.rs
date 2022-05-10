use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

/// Provides integration tests for different scenarios within the transactor engine.
/// These test run the full flow i.e. provide a file in a given state and assert the result.
/// Could use BDD framework for this

const ACCOUNT_HEADERS: &str = "client,available,held,total,locked\n";

#[test]
fn should_test_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-tx-engine").unwrap();
    cmd.arg("tests/scenarios/transactions.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCOUNT_HEADERS))
        .stdout(predicate::str::contains("1,1.5000,0,1.5000,false\n"))
        .stdout(predicate::str::contains("2,2.0000,0,2.0000,false\n"));
    Ok(())
}

#[test]
fn should_deposit_funds() -> Result<(), Box<dyn std::error::Error>> {
    // given two empty accounts
    // when multiple deposits are called
    // than the total off all deposits is recognised for each account
    let mut cmd = Command::cargo_bin("rust-tx-engine").unwrap();
    cmd.arg("tests/scenarios/deposits.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCOUNT_HEADERS))
        .stdout(predicate::str::contains("1,10.0000,0,10.0000,false\n"))
        .stdout(predicate::str::contains("2,15.0000,0,15.0000,false\n"));
    Ok(())
}

#[test]
fn should_not_allow_further_actions_after_chargeback() -> Result<(), Box<dyn std::error::Error>> {
    // given a transaction is in a disputed state
    // when a chargeback is called
    // than a further deposit is not recognised
    // and account is locked
    // and total has been reduced
    // and available has been reduced
    // and held has been reduced
    let mut cmd = Command::cargo_bin("rust-tx-engine").unwrap();
    cmd.arg("tests/scenarios/chargeback.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCOUNT_HEADERS))
        .stdout(predicate::str::contains("1,0.0000,0.0000,0.0000,true\n"));
    Ok(())
}

#[test]
fn should_dispute_transaction() -> Result<(), Box<dyn std::error::Error>> {
    // given several transaction deposits
    // when a transaction is disputed
    // then total has been increased
    // and available has been reduced
    // and held has been increased
    let mut cmd = Command::cargo_bin("rust-tx-engine").unwrap();
    cmd.arg("tests/scenarios/dispute.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCOUNT_HEADERS))
        .stdout(predicate::str::contains("1,2.0000,10.0000,12.0000,false\n"));
    Ok(())
}

#[test]
fn should_resolve_dispute() -> Result<(), Box<dyn std::error::Error>> {
    // given a transaction is in a disputed state
    // when a transaction has been resolved
    // than held funds is reduced
    // and available funds has been increased
    // and total remains the same
    let mut cmd = Command::cargo_bin("rust-tx-engine").unwrap();
    cmd.arg("tests/scenarios/resolve.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(ACCOUNT_HEADERS))
        .stdout(predicate::str::contains("1,10.0000,0.0000,10.0000,false\n"));
    Ok(())
}
