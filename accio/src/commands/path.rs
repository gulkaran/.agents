use std::fs;
use std::path::Path;

use anyhow::{Context as _, Result, anyhow, bail};

use crate::artifact::{self, Kind};
use crate::context::Context;

pub fn run(ctx: &Context, artifact: &str, name: Option<&str>) -> Result<()> {
    let artifact = artifact::find(artifact)?;
    if let Some(name) = name {
        if let Kind::File = artifact.kind {
            bail!(
                "{} is a file; --name applies only to research, feasibility, verification, and review",
                artifact.name
            );
        }
        artifact::validate_name(name)?;
    }
    let path = ctx.topic()?.path.join(artifact.path);
    let path = match (&artifact.kind, name) {
        (Kind::File, _) => path,
        (Kind::Directory, None) => {
            ensure_dir(&path)?;
            path
        }
        (Kind::Directory, Some(name)) => {
            let entry = path.join(name);
            let dir = entry
                .parent()
                .ok_or_else(|| anyhow!("{} has no parent directory", entry.display()))?;
            ensure_dir(dir)?;
            entry
        }
    };
    println!("{}", path.display());
    Ok(())
}

fn ensure_dir(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("cannot create {}", dir.display()))
}
