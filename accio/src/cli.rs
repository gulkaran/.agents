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
    /// Create, switch, and query the topic this checkout works on.
    #[command(subcommand)]
    Topic(TopicCommand),
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
    /// Print the value stored under a key.
    Get { key: String },
    /// Store a value under a key.
    Set { key: String, value: String },
    /// Print every key and value.
    List {
        /// Print a JSON object instead of key: value lines.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
pub enum TopicCommand {
    /// Create the topic folder when missing and point this checkout's
    /// .topic marker at it.
    Use {
        slug: String,
        /// Project to file the topic under instead of the one derived from
        /// the origin remote.
        #[arg(long)]
        project: Option<String>,
    },
    /// Print the topic the .topic marker at the git toplevel points to.
    Current,
}
