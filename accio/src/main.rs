mod cli;
mod commands;
mod config;
mod context;
mod git;
mod vault;

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use context::Context;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("accio: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> Result<()> {
    let ctx = Context::from_env()?;
    match command {
        Command::Vault(command) => commands::vault::run(&ctx, command),
    }
}
