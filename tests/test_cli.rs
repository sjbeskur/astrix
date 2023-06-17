use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = env!("CARGO_PKG_NAME");
const TEST_DATA: &str = "data/test.csv";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}


// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;        
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();
    //    .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn run_test_data() -> TestResult {
    run(&[TEST_DATA], "data/test.csv")
}