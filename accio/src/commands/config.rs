use anyhow::Result;

use crate::cli::ConfigCommand;
use crate::config;
use crate::context::Context;

pub fn run(ctx: &Context, command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::Set { key, value } => set(ctx, key, value),
    }
}

fn set(ctx: &Context, key: String, value: String) -> Result<()> {
    let mut table = config::load(ctx.config_path())?;
    table.insert(key, value);
    config::save(ctx.config_path(), &table)
}
