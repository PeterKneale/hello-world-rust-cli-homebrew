use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("hello-world-rust-cli-homebrew").unwrap()
}

#[test]
fn prints_default_greeting() {
    bin()
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, world!"));
}

#[test]
fn prints_named_greeting() {
    bin()
        .arg("Peter")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Peter!"));
}

#[test]
fn version_long_flag() {
    bin()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn version_short_flag() {
    bin()
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_long_flag() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE").and(predicate::str::contains("--version")));
}

#[test]
fn help_short_flag() {
    bin()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
}

#[test]
fn unknown_flag_errors() {
    bin()
        .arg("--nope")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("unrecognised flag '--nope'"));
}
