use std::fs;
use std::path::Path;

use anyhow::{Context as _, Result};

use crate::cli::TopicCommand;
use crate::context::Context;
use crate::git;
use crate::topic;

pub fn run(ctx: &Context, command: TopicCommand) -> Result<()> {
    match command {
        TopicCommand::Use { slug, project } => use_topic(ctx, &slug, project),
        TopicCommand::Current => current(ctx),
    }
}

fn use_topic(ctx: &Context, slug: &str, project: Option<String>) -> Result<()> {
    topic::validate_name("slug", slug)?;
    if let Some(name) = &project {
        topic::validate_name("project", name)?;
    }
    let toplevel = ctx.toplevel()?;
    let project = match project {
        Some(name) => name,
        None => {
            let origin = git::run(toplevel, &["remote", "get-url", "origin"]).ok();
            topic::derive_project(origin.as_deref(), toplevel)?
        }
    };
    let path = ctx
        .vault()?
        .join("projects")
        .join(&project)
        .join("topics")
        .join(slug);
    let created = !path.is_dir();
    fs::create_dir_all(&path).with_context(|| format!("cannot create {}", path.display()))?;
    let exclude = git::run(
        toplevel,
        &["rev-parse", "--path-format=absolute", "--git-path", "info/exclude"],
    )?;
    topic::ensure_exclude_line(Path::new(&exclude))?;
    topic::write_marker(toplevel, &path)?;
    let verb = if created { "created" } else { "using" };
    println!("{verb} {}", path.display());
    Ok(())
}

fn current(ctx: &Context) -> Result<()> {
    ctx.topic()?;
    Ok(())
}
