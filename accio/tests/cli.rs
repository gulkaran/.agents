//! The error contract shared by every command: clap usage errors exit 2.

mod common;

use common::Harness;

#[test]
fn unknown_subcommand_is_a_usage_error() {
    Harness::new().accio().arg("bogus").assert().code(2);
}

#[test]
fn missing_subcommand_is_a_usage_error() {
    Harness::new().accio().assert().code(2);
}
