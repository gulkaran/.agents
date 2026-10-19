use std::fs;
use std::path::Path;

use anyhow::{Context as _, Result};

use crate::artifact::{self, Kind};
use crate::context::Context;

pub fn run(ctx: &Context, artifact: &str) -> Result<()> {
    let artifact = artifact::find(artifact)?;
    let path = ctx.topic()?.path.join(artifact.path);
    if let Kind::Directory = artifact.kind {
        ensure_dir(&path)?;
    }
    println!("{}", path.display());
    Ok(())
}

fn ensure_dir(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("cannot create {}", dir.display()))
}
