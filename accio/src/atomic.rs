//! Whole-file writes that never leave a truncated file behind.

use std::fs::{self, File};
use std::io::Write as _;
use std::path::Path;
use std::process;

use anyhow::{Result, anyhow};

/// Writes `contents` to a temporary file beside `path`, syncs it, and renames
/// it over `path`, creating the parent directory when missing. An interrupted
/// write leaves the previous file intact.
pub fn write(path: &Path, contents: &[u8]) -> Result<()> {
    let dir = path
        .parent()
        .ok_or_else(|| anyhow!("{} has no parent directory", path.display()))?;
    let name = path
        .file_name()
        .ok_or_else(|| anyhow!("{} has no file name", path.display()))?;
    fs::create_dir_all(dir)?;
    let temp = dir.join(format!(".{}.{}.tmp", name.to_string_lossy(), process::id()));
    let written = File::create(&temp).and_then(|mut file| {
        file.write_all(contents)?;
        file.sync_all()?;
        fs::rename(&temp, path)
    });
    if written.is_err() {
        let _ = fs::remove_file(&temp);
    }
    Ok(written?)
}
