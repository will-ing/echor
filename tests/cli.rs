use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn compares() -> TestResult {
    let outfile = "tests/expected/hello.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello there").assert().success().stdout(expected);
    Ok(())
}
