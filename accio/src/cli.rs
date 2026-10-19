//! The clap command tree. Each verb dispatches to one module under `commands`.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "accio",
    version,
    about = "Vault and topic paths for workflow skills"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Point accio at the vault repository.
    #[command(subcommand)]
    Vault(VaultCommand),
    /// Read and write the flat key-value config file.
    #[command(subcommand)]
    Config(ConfigCommand),
}

#[derive(Subcommand)]
pub enum VaultCommand {
    /// Record the root of the vault repository.
    Set { path: PathBuf },
    /// Print the configured vault root.
    Path,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Store a value under a key.
    Set { key: String, value: String },
}
