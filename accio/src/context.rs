//! What every command needs to know about where it runs: the current
//! directory, the config file, and the values derived from them.

use std::cell::OnceCell;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, anyhow};

use crate::config::{self, Table};
use crate::git;
use crate::topic::{self, Topic};

pub struct Context {
    cwd: PathBuf,
    config_path: PathBuf,
    config: OnceCell<Table>,
    toplevel: OnceCell<PathBuf>,
    topic: OnceCell<Topic>,
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
            toplevel: OnceCell::new(),
            topic: OnceCell::new(),
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

    /// The root of the git working tree that contains the current directory,
    /// found on first use.
    pub fn toplevel(&self) -> Result<&Path> {
        if let Some(path) = self.toplevel.get() {
            return Ok(path);
        }
        let path = git::run(&self.cwd, &["rev-parse", "--show-toplevel"])
            .map_err(|_| anyhow!("not inside a git repository; run accio from a checkout"))?;
        let path = fs::canonicalize(&path).with_context(|| format!("cannot resolve {path}"))?;
        Ok(self.toplevel.get_or_init(|| path))
    }

    /// The topic the marker at the toplevel points to, read on first use.
    pub fn topic(&self) -> Result<&Topic> {
        if let Some(topic) = self.topic.get() {
            return Ok(topic);
        }
        let topic = topic::read_marker(self.toplevel()?)?;
        Ok(self.topic.get_or_init(|| topic))
    }
}
