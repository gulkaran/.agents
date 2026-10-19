use anyhow::{Result, anyhow};

use crate::context::Context;
use crate::git;

pub fn run(ctx: &Context, message: &str) -> Result<()> {
    let topic = ctx.topic()?;
    let dir = topic.path.as_path();
    let pathspec = dir
        .to_str()
        .ok_or_else(|| anyhow!("{} is not valid UTF-8", dir.display()))?;
    git::run(dir, &["add", "-A", "--", pathspec])?;
    let subject = format!("docs({}): {message}", topic.slug);
    git::run(dir, &["commit", "-q", "-m", &subject, "--", pathspec])?;
    println!("{}", git::run(dir, &["log", "-1", "--format=%h %s"])?);
    Ok(())
}
