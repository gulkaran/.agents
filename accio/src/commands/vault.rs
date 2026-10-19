use std::path::Path;

use anyhow::{Result, anyhow};

use crate::cli::VaultCommand;
use crate::config;
use crate::context::Context;
use crate::vault;

pub fn run(ctx: &Context, command: VaultCommand) -> Result<()> {
    match command {
        VaultCommand::Set { path } => set(ctx, &path),
        VaultCommand::Path => path(ctx),
    }
}

fn set(ctx: &Context, candidate: &Path) -> Result<()> {
    let root = vault::validate_root(&ctx.cwd().join(candidate))?;
    let root = root
        .to_str()
        .ok_or_else(|| anyhow!("{} is not valid UTF-8", root.display()))?;
    let mut table = config::load(ctx.config_path())?;
    table.insert(config::VAULT_KEY.to_string(), root.to_string());
    config::save(ctx.config_path(), &table)?;
    println!("{root}");
    Ok(())
}

fn path(ctx: &Context) -> Result<()> {
    println!("{}", ctx.vault()?.display());
    Ok(())
}
