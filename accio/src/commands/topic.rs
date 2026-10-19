use anyhow::Result;

use crate::cli::TopicCommand;
use crate::context::Context;

pub fn run(ctx: &Context, command: TopicCommand) -> Result<()> {
    match command {
        TopicCommand::Current => current(ctx),
    }
}

fn current(ctx: &Context) -> Result<()> {
    ctx.topic()?;
    Ok(())
}
