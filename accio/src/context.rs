//! What every command needs to know about where it runs: the current
//! directory, the config file, and the values derived from them.

use std::cell::OnceCell;
use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, anyhow};

use crate::config::{self, Table};

pub struct Context {
    cwd: PathBuf,
    config_path: PathBuf,
    config: OnceCell<Table>,
}

impl Context {
    /// Reads the current directory and `HOME`; the config file is
    /// `$HOME/.config/accio/config.toml`.
    pub fn from_env() -> Result<Self> {
        let cwd = env::current_dir().context("cannot read the current directory")?;
        let home = env::var_os("HOME").ok_or_else(|| anyhow!("HOME is not set"))?;
        let config_path = PathBuf::from(home)
            .join(".config")
            .join("accio")
            .join("config.toml");
        Ok(Self {
            cwd,
            config_path,
            config: OnceCell::new(),
        })
    }

    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    /// The config table, read from disk on first use. A missing file is an
    /// empty table.
    pub fn config(&self) -> Result<&Table> {
        if let Some(table) = self.config.get() {
            return Ok(table);
        }
        let table = config::load(&self.config_path)?;
        Ok(self.config.get_or_init(|| table))
    }

    /// The configured vault root.
    pub fn vault(&self) -> Result<&Path> {
        let path = self
            .config()?
            .get(config::VAULT_KEY)
            .ok_or_else(|| anyhow!("no vault configured; run accio vault set <path>"))?;
        Ok(Path::new(path))
    }
}
