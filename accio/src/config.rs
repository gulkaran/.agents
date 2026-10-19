//! The config file at `$HOME/.config/accio/config.toml`: one flat TOML table
//! of string keys to string values.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use anyhow::{Context as _, Result};

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
}
