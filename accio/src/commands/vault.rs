use anyhow::Result;

use crate::cli::VaultCommand;
use crate::context::Context;

pub fn run(ctx: &Context, command: VaultCommand) -> Result<()> {
    match command {
        VaultCommand::Path => path(ctx),
    }
}

fn path(ctx: &Context) -> Result<()> {
    println!("{}", ctx.vault()?.display());
    Ok(())
}
