use anyhow::{Result, anyhow, bail};

use crate::context::Context;
use crate::git;

pub fn run(ctx: &Context, message: &str) -> Result<()> {
    let topic = ctx.topic()?;
    let dir = topic.path.as_path();
    let pathspec = dir
        .to_str()
        .ok_or_else(|| anyhow!("{} is not valid UTF-8", dir.display()))?;
    git::run(dir, &["rev-parse", "--show-toplevel"]).map_err(|_| {
        anyhow!(
            "{} is not inside a git repository; run git init in the vault root",
            dir.display()
        )
    })?;
    let status = git::run(
        dir,
        &[
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--",
            pathspec,
        ],
    )?;
    if status.is_empty() {
        bail!("nothing changed in {}/{}", topic.project, topic.slug);
    }
    git::run(dir, &["add", "-A", "--", pathspec])?;
    let subject = format!("docs({}): {message}", topic.slug);
    git::run(dir, &["commit", "-q", "-m", &subject, "--", pathspec])?;
    println!("{}", git::run(dir, &["log", "-1", "--format=%h %s"])?);
    // `--relative` run from the topic folder prints paths under it, so each
    // line reads `<status>\t<path inside the topic>`.
    let changes = git::run(
        dir,
        &[
            "-c",
            "core.quotePath=false",
            "show",
            "--relative",
            "--no-renames",
            "--name-status",
            "--format=",
            "HEAD",
        ],
    )?;
    for change in changes.lines().filter(|line| !line.is_empty()) {
        let (status, path) = change
            .split_once('\t')
            .ok_or_else(|| anyhow!("git show printed an unexpected line: {change}"))?;
        println!("{status} {path}");
    }
    Ok(())
}
