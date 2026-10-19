//! The clap command tree. Each verb dispatches to one module under `commands`.

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
}

#[derive(Subcommand)]
pub enum VaultCommand {
    /// Print the configured vault root.
    Path,
}
