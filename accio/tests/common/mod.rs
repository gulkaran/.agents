//! Shared fixtures for the integration tests: a temporary `HOME`, a vault
//! repository with one commit, a work repository with an `origin` URL, and
//! `git` and `accio` commands that never read the real user configuration or
//! reach the network.

// Each test binary compiles its own copy of this module and uses a different
// subset of it.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

pub struct Harness {
    pub home: TempDir,
    pub vault: TempDir,
    pub work: TempDir,
}

impl Harness {
    pub fn new() -> Self {
        let home = tempfile::tempdir().unwrap();
        fs::write(home.path().join("gitconfig"), "").unwrap();
        let vault = tempfile::tempdir().unwrap();
        let work = tempfile::tempdir().unwrap();
        let harness = Harness { home, vault, work };
        harness.init_repo(harness.vault.path());
        harness.init_repo(harness.work.path());
        harness.git(
            harness.work.path(),
            &["remote", "add", "origin", "git@github.com:gulkaran/.agents.git"],
        );
        harness
    }

    /// The config file accio reads and writes under the temporary `HOME`.
    pub fn config_path(&self) -> PathBuf {
        self.home.path().join(".config/accio/config.toml")
    }

    /// `accio` run from the temporary `HOME`, which is not a git repository.
    pub fn accio(&self) -> assert_cmd::Command {
        self.accio_in(self.home.path())
    }

    /// `accio` run from `dir` with the temporary `HOME` and git isolated.
    pub fn accio_in(&self, dir: &Path) -> assert_cmd::Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_accio"));
        cmd.current_dir(dir);
        self.isolate(&mut cmd);
        assert_cmd::Command::from_std(cmd)
    }

    /// Runs `git -C <dir> <args>` with the isolated environment and panics
    /// when it fails.
    pub fn git(&self, dir: &Path, args: &[&str]) -> String {
        let mut cmd = Command::new("git");
        cmd.arg("-C").arg(dir).args(args);
        self.isolate(&mut cmd);
        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "git {args:?} failed in {}: {}",
            dir.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }

    /// Turns `dir` into a git repository with one commit.
    pub fn init_repo(&self, dir: &Path) {
        self.git(dir, &["init", "-q", "-b", "main"]);
        fs::write(dir.join("README.md"), "fixture\n").unwrap();
        self.git(dir, &["add", "README.md"]);
        self.git(dir, &["commit", "-q", "-m", "Initial commit"]);
    }

    fn isolate(&self, cmd: &mut Command) {
        cmd.env("HOME", self.home.path())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", self.home.path().join("gitconfig"))
            .env("GIT_AUTHOR_NAME", "accio tests")
            .env("GIT_AUTHOR_EMAIL", "accio@example.invalid")
            .env("GIT_COMMITTER_NAME", "accio tests")
            .env("GIT_COMMITTER_EMAIL", "accio@example.invalid");
    }
}
