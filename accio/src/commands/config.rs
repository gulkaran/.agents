use anyhow::{Result, anyhow};

use crate::cli::ConfigCommand;
use crate::config;
use crate::context::Context;

pub fn run(ctx: &Context, command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::Get { key } => get(ctx, &key),
        ConfigCommand::Set { key, value } => set(ctx, key, value),
    }
}

fn get(ctx: &Context, key: &str) -> Result<()> {
    let value = ctx
        .config()?
        .get(key)
        .ok_or_else(|| anyhow!("no value for {key}; run accio config set {key} <value>"))?;
    println!("{value}");
    Ok(())
}

fn set(ctx: &Context, key: String, value: String) -> Result<()> {
    let mut table = config::load(ctx.config_path())?;
    table.insert(key, value);
    config::save(ctx.config_path(), &table)
}
