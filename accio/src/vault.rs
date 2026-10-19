//! Validation of the directory `vault set` is asked to record.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, bail};

use crate::git;

/// Checks that `candidate` is a directory inside a git repository and
/// resolves it to the canonical path stored as the vault root.
pub fn validate_root(candidate: &Path) -> Result<PathBuf> {
    check_root(candidate).context("vault cannot be found")
}

fn check_root(candidate: &Path) -> Result<PathBuf> {
    if !candidate.is_dir() {
        bail!("{} is not a directory", candidate.display());
    }
    let canonical = fs::canonicalize(candidate)?;
    git::run(&canonical, &["rev-parse", "--show-toplevel"])?;
    Ok(canonical)
}
