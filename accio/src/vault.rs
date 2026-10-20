//! The vault root: validation of the directory `vault set` is asked to
//! record, and the listing of the topics under it.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, bail};

use crate::git;
use crate::topic::Topic;

/// Checks that `candidate` is a directory and the root of a git repository,
/// and resolves it to the canonical path stored as the vault root.
pub fn validate_root(candidate: &Path) -> Result<PathBuf> {
    check_root(candidate).context("vault cannot be found")
}

fn check_root(candidate: &Path) -> Result<PathBuf> {
    if !candidate.is_dir() {
        bail!("{} is not a directory", candidate.display());
    }
    let canonical = fs::canonicalize(candidate)?;
    let toplevel = git::run(&canonical, &["rev-parse", "--show-toplevel"])?;
    let toplevel = fs::canonicalize(toplevel)?;
    if toplevel != canonical {
        bail!(
            "{} is not a repository root; run accio vault set {}",
            canonical.display(),
            toplevel.display()
        );
    }
    Ok(canonical)
}

/// Every directory at `<vault>/projects/*/topics/*`, sorted by project and
/// slug. Files and hidden entries at either level are skipped, and a vault
/// without `projects/` has no topics.
pub fn list_topics(vault: &Path) -> Result<Vec<Topic>> {
    let mut topics = Vec::new();
    for project in visible_dirs(&vault.join("projects"))? {
        for path in visible_dirs(&project.join("topics"))? {
            topics.extend(Topic::from_path(path));
        }
    }
    Ok(topics)
}

/// The subdirectories of `dir` whose names do not start with a dot, sorted
/// by name. A missing `dir` has none.
fn visible_dirs(dir: &Path) -> Result<Vec<PathBuf>> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(err).with_context(|| format!("cannot read {}", dir.display())),
    };
    let mut dirs = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("cannot read {}", dir.display()))?;
        let hidden = entry.file_name().to_string_lossy().starts_with('.');
        if !hidden && entry.file_type()?.is_dir() {
            dirs.push(entry.path());
        }
    }
    dirs.sort();
    Ok(dirs)
}
