use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_default_greeting() {
    Command::cargo_bin("hello-world-rust-cli-homebrew")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, world!"));
}

#[test]
fn prints_named_greeting() {
    Command::cargo_bin("hello-world-rust-cli-homebrew")
        .unwrap()
        .arg("Peter")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Peter!"));
}
