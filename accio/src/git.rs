//! The one way accio talks to git: a subprocess that inherits the caller's
//! environment unchanged.

use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, Result, bail};

/// Runs `git -C <dir> <args>` and returns its stdout without trailing
/// newlines. A non-zero exit becomes an error carrying git's stderr.
pub fn run(dir: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .context("cannot run git")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = match stderr.trim() {
            "" => output.status.to_string(),
            stderr => stderr.to_string(),
        };
        bail!("git {} in {}: {detail}", args.join(" "), dir.display());
    }
    let stdout = String::from_utf8(output.stdout).context("git printed invalid UTF-8")?;
    Ok(stdout.trim_end_matches('\n').to_string())
}
