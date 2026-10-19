use std::fs;
use std::path::Path;

use anyhow::{Context as _, Result};
use serde::Serialize;

use crate::artifact;
use crate::cli::TopicCommand;
use crate::context::Context;
use crate::git;
use crate::topic;

pub fn run(ctx: &Context, command: TopicCommand) -> Result<()> {
    match command {
        TopicCommand::Use { slug, project } => use_topic(ctx, &slug, project),
        TopicCommand::Current { path, json } => current(ctx, path, json),
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

#[derive(Serialize)]
struct Current<'a> {
    project: &'a str,
    slug: &'a str,
    path: &'a Path,
    artifacts: Vec<&'static str>,
}

fn current(ctx: &Context, path_only: bool, json: bool) -> Result<()> {
    let topic = ctx.topic()?;
    if path_only {
        println!("{}", topic.path.display());
        return Ok(());
    }
    let artifacts: Vec<&str> = artifact::ALL
        .iter()
        .filter(|artifact| artifact.exists_in(&topic.path))
        .map(|artifact| artifact.name)
        .collect();
    if json {
        let current = Current {
            project: &topic.project,
            slug: &topic.slug,
            path: &topic.path,
            artifacts,
        };
        println!("{}", serde_json::to_string(&current)?);
    } else {
        println!("project: {}", topic.project);
        println!("slug: {}", topic.slug);
        println!("path: {}", topic.path.display());
        let names: String = artifacts.iter().map(|name| format!(" {name}")).collect();
        println!("artifacts:{names}");
    }
    Ok(())
}
