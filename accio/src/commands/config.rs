use anyhow::{Result, anyhow, bail};

use crate::cli::ConfigCommand;
use crate::config;
use crate::context::Context;

pub fn run(ctx: &Context, command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::Get { key } => get(ctx, &key),
        ConfigCommand::Set { key, value } => set(ctx, key, value),
        ConfigCommand::List { json } => list(ctx, json),
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
    if key == config::VAULT_KEY {
        bail!("vault is managed by accio vault set; run accio vault set <path>");
    }
    config::validate_key(&key)?;
    let mut table = config::load(ctx.config_path())?;
    table.insert(key, value);
    config::save(ctx.config_path(), &table)
}

fn list(ctx: &Context, json: bool) -> Result<()> {
    let table = ctx.config()?;
    if json {
        println!("{}", serde_json::to_string(table)?);
    } else {
        for (key, value) in table {
            println!("{key}: {value}");
        }
    }
    Ok(())
}
