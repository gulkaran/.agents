//! Validation of the directory `vault set` is asked to record.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

/// Resolves `candidate` to the canonical path stored as the vault root.
pub fn validate_root(candidate: &Path) -> Result<PathBuf> {
    fs::canonicalize(candidate).with_context(|| format!("cannot resolve {}", candidate.display()))
}
