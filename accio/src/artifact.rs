//! The eight artifacts a topic folder can hold, in canonical order, and the
//! rules for naming an entry under a directory artifact.

use std::path::{Component, Path};

use anyhow::{Result, bail};

pub enum Kind {
    File,
    Directory,
}

pub struct Artifact {
    pub name: &'static str,
    pub kind: Kind,
    /// Location under the topic folder.
    pub path: &'static str,
}

impl Artifact {
    /// Whether the topic holds this artifact as the expected kind of entry.
    pub fn exists_in(&self, topic: &Path) -> bool {
        let path = topic.join(self.path);
        match self.kind {
            Kind::File => path.is_file(),
            Kind::Directory => path.is_dir(),
        }
    }
}

pub const ALL: [Artifact; 8] = [
    Artifact {
        name: "brief",
        kind: Kind::File,
        path: "brief.md",
    },
    Artifact {
        name: "investigation",
        kind: Kind::File,
        path: "investigation.md",
    },
    Artifact {
        name: "design",
        kind: Kind::File,
        path: "design.md",
    },
    Artifact {
        name: "decisions",
        kind: Kind::File,
        path: "decisions.md",
    },
    Artifact {
        name: "research",
        kind: Kind::Directory,
        path: "research",
    },
    Artifact {
        name: "feasibility",
        kind: Kind::Directory,
        path: "feasibility",
    },
    Artifact {
        name: "verification",
        kind: Kind::Directory,
        path: "verification",
    },
    Artifact {
        name: "review",
        kind: Kind::Directory,
        path: "review",
    },
];

/// Accepts a `--name` that stays inside its directory artifact: relative,
/// non-empty, and free of `..` components. The name is otherwise used
/// verbatim, so `screenshots/login.png` names a file in a subfolder.
pub fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("--name must not be empty");
    }
    let path = Path::new(name);
    if path.is_absolute() {
        bail!("--name {name:?} must be a relative path");
    }
    if path
        .components()
        .any(|component| component == Component::ParentDir)
    {
        bail!("--name {name:?} must not contain \"..\" components");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn relative_names_without_parent_components_are_valid() {
        for name in [
            "evidence.md",
            "screenshots/login.png",
            "a/b/c",
            "round-1.md",
        ] {
            validate_name(name).unwrap();
        }
    }

    #[test]
    fn empty_name_is_rejected() {
        let err = validate_name("").unwrap_err();
        assert_eq!(err.to_string(), "--name must not be empty");
    }

    #[test]
    fn absolute_name_is_rejected() {
        let err = validate_name("/tmp/evidence.md").unwrap_err();
        assert_eq!(
            err.to_string(),
            "--name \"/tmp/evidence.md\" must be a relative path"
        );
    }

    #[test]
    fn name_with_parent_components_is_rejected() {
        for name in ["../design.md", "a/../../b", "screenshots/.."] {
            let err = validate_name(name).unwrap_err();
            assert_eq!(
                err.to_string(),
                format!("--name {name:?} must not contain \"..\" components")
            );
        }
    }

    #[test]
    fn next_round_is_round_1_when_review_is_missing_or_empty() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            next_round(&dir.path().join("review")).unwrap(),
            "round-1.md"
        );
        fs::create_dir(dir.path().join("review")).unwrap();
        assert_eq!(
            next_round(&dir.path().join("review")).unwrap(),
            "round-1.md"
        );
    }

    #[test]
    fn next_round_follows_the_highest_round_without_filling_gaps() {
        let dir = tempfile::tempdir().unwrap();
        for file in ["round-1.md", "round-3.md"] {
            fs::write(dir.path().join(file), "").unwrap();
        }
        assert_eq!(next_round(dir.path()).unwrap(), "round-4.md");
    }

    #[test]
    fn next_round_ignores_entries_that_are_not_round_files() {
        let dir = tempfile::tempdir().unwrap();
        for file in [
            "round-01.md",
            "notes.md",
            "round-2.md.bak",
            "round-0.md",
            "round-.md",
            "round-x.md",
        ] {
            fs::write(dir.path().join(file), "").unwrap();
        }
        fs::create_dir(dir.path().join("round-2")).unwrap();
        assert_eq!(next_round(dir.path()).unwrap(), "round-1.md");
        fs::write(dir.path().join("round-12.md"), "").unwrap();
        assert_eq!(next_round(dir.path()).unwrap(), "round-13.md");
    }
}
