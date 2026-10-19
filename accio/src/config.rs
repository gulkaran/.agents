//! The config file at `$HOME/.config/accio/config.toml`: one flat TOML table
//! of string keys to string values.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, Write as _};
use std::path::Path;
use std::process;

use anyhow::{Context as _, Result, anyhow};

/// The one key accio interprets. `vault set` writes it; `vault path` and the
/// topic commands read it.
pub const VAULT_KEY: &str = "vault";

pub type Table = BTreeMap<String, String>;

/// Reads the config file. A missing file is an empty table.
pub fn load(path: &Path) -> Result<Table> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Table::new()),
        Err(err) => return Err(err).with_context(|| format!("cannot read {}", path.display())),
    };
    toml::from_str(&text).with_context(|| format!("cannot parse {}", path.display()))
}

/// Writes the table as TOML, creating the directory when missing. The text
/// goes to a temporary file beside the config file and is renamed over it, so
/// an interrupted write never leaves a truncated config.
pub fn save(path: &Path, table: &Table) -> Result<()> {
    let text = toml::to_string(table).context("cannot serialize the config table")?;
    write_atomically(path, text.as_bytes())
        .with_context(|| format!("cannot write {}", path.display()))
}

fn write_atomically(path: &Path, contents: &[u8]) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_then_load_returns_the_same_table() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("config.toml");
        let mut table = Table::new();
        table.insert("tracker".to_string(), "github".to_string());
        table.insert("vault".to_string(), "/Users/me/Desktop/vault".to_string());

        save(&path, &table).unwrap();

        assert_eq!(load(&path).unwrap(), table);
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "tracker = \"github\"\nvault = \"/Users/me/Desktop/vault\"\n"
        );
    }

    #[test]
    fn non_string_value_is_an_error_naming_the_key() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "tracker = \"github\"\nretries = 3\n").unwrap();

        let err = load(&path).unwrap_err();

        assert_eq!(err.to_string(), "config value for retries is not a string");
    }
}
