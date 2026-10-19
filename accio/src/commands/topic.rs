use std::fs;

use anyhow::{Context as _, Result};

use crate::cli::TopicCommand;
use crate::context::Context;
use crate::git;
use crate::topic;

pub fn run(ctx: &Context, command: TopicCommand) -> Result<()> {
    match command {
        TopicCommand::Use { slug } => use_topic(ctx, &slug),
        TopicCommand::Current => current(ctx),
    }
}

fn use_topic(ctx: &Context, slug: &str) -> Result<()> {
    let toplevel = ctx.toplevel()?;
    let origin = git::run(toplevel, &["remote", "get-url", "origin"]).ok();
    let project = topic::derive_project(origin.as_deref(), toplevel)?;
    let path = ctx
        .vault()?
        .join("projects")
        .join(&project)
        .join("topics")
        .join(slug);
    let created = !path.is_dir();
    fs::create_dir_all(&path).with_context(|| format!("cannot create {}", path.display()))?;
    topic::write_marker(toplevel, &path)?;
    let verb = if created { "created" } else { "using" };
    println!("{verb} {}", path.display());
    Ok(())
}

fn current(ctx: &Context) -> Result<()> {
    ctx.topic()?;
    Ok(())
}
