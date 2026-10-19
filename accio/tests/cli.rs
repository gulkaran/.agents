//! The error contract shared by every command: clap usage errors exit 2.

use assert_cmd::Command;

#[test]
fn unknown_subcommand_is_a_usage_error() {
    Command::new(env!("CARGO_BIN_EXE_accio"))
        .arg("bogus")
        .assert()
        .code(2);
}

#[test]
fn missing_subcommand_is_a_usage_error() {
    Command::new(env!("CARGO_BIN_EXE_accio")).assert().code(2);
}
