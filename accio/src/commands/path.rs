use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, bail};

use crate::artifact::{self, Kind};
use crate::context::Context;

pub fn run(ctx: &Context, artifact: &str, name: Option<&str>, next: bool) -> Result<()> {
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
    if next && artifact.name != "review" {
        bail!("--next applies only to review; run accio path review --next");
    }
    let path = ctx.topic()?.path.join(artifact.path);
    let path = match (&artifact.kind, name, next) {
        (Kind::File, ..) => path,
        (Kind::Directory, None, false) => {
            ensure_dir(&path)?;
            path
        }
        (Kind::Directory, None, true) => {
            let round = artifact::next_round(&path)?;
            ensure_dir(&path)?;
            path.join(round)
        }
        (Kind::Directory, Some(name), _) => {
            ensure_dir(&path.join(entry_dir(name)))?;
            path.join(name)
        }
    };
    println!("{}", path.display());
    Ok(())
}

fn ensure_dir(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("cannot create {}", dir.display()))
}

/// The directory that must exist before a file can be written at `<dir>/<name>`:
/// every segment of `name` but the last, without `.` and empty segments.
/// `Path::parent` would drop a trailing `.` segment and point one level too high.
fn entry_dir(name: &str) -> PathBuf {
    let segments: Vec<&str> = name.trim_end_matches('/').split('/').collect();
    segments[..segments.len() - 1]
        .iter()
        .filter(|segment| !segment.is_empty() && **segment != ".")
        .collect()
}
