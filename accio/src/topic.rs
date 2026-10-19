//! The current topic: the marker at the git toplevel, the exclude line that
//! hides it, and the project and slug names that locate a topic in the vault.

use std::fs::{self, OpenOptions};
use std::io::{self, Write as _};
use std::path::{Component, Path, PathBuf};

use anyhow::{Context as _, Result, bail};
use serde::Serialize;

use crate::atomic;

/// The file at the git toplevel that holds the topic folder's absolute path.
pub const MARKER: &str = ".topic";

/// The exclude pattern that hides the marker at every worktree root.
const EXCLUDE_LINE: &str = "/.topic";

/// A topic folder in the vault, located by its project and slug.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Topic {
    pub project: String,
    pub slug: String,
    pub path: PathBuf,
}

impl Topic {
    /// Reads the project and slug from an absolute path whose last four
    /// components are `projects/<project>/topics/<slug>`. Any other path is
    /// not a topic.
    pub fn from_path(path: PathBuf) -> Option<Self> {
        if !path.is_absolute() {
            return None;
        }
        let mut components = path.components().rev().map(|component| match component {
            Component::Normal(name) => name.to_str(),
            _ => None,
        });
        let slug = components.next()??;
        if components.next()?? != "topics" {
            return None;
        }
        let project = components.next()??;
        if components.next()?? != "projects" {
            return None;
        }
        Some(Topic {
            project: project.to_string(),
            slug: slug.to_string(),
            path,
        })
    }
}

/// Reads the topic from `<toplevel>/.topic`. The marker's first line must be
/// an absolute path into a vault's `projects/<project>/topics/<slug>`, and
/// that folder must exist.
pub fn read_marker(toplevel: &Path) -> Result<Topic> {
    let marker = toplevel.join(MARKER);
    let text = match fs::read_to_string(&marker) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            bail!("no {MARKER} here; run accio topic use <slug>")
        }
        Err(err) => return Err(err).with_context(|| format!("cannot read {}", marker.display())),
    };
    let path = PathBuf::from(text.lines().next().unwrap_or_default());
    let Some(topic) = Topic::from_path(path) else {
        bail!(
            "{MARKER} at {} does not point into a vault topic; run accio topic use <slug>",
            marker.display()
        );
    };
    if !topic.path.is_dir() {
        bail!(
            "{MARKER} points to {}, which does not exist; run accio topic use <slug>",
            topic.path.display()
        );
    }
    Ok(topic)
}

/// Points `<toplevel>/.topic` at a topic folder.
pub fn write_marker(toplevel: &Path, topic_path: &Path) -> Result<()> {
    let marker = toplevel.join(MARKER);
    let line = format!("{}\n", topic_path.display());
    atomic::write(&marker, line.as_bytes())
        .with_context(|| format!("cannot write {}", marker.display()))
}

/// Appends the `/.topic` line to git's exclude file unless a line already
/// equals it, creating the file and its directory when missing and starting
/// a new line when the file does not end with one.
pub fn ensure_exclude_line(exclude: &Path) -> Result<()> {
    let existing = match fs::read_to_string(exclude) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => String::new(),
        Err(err) => return Err(err).with_context(|| format!("cannot read {}", exclude.display())),
    };
    if existing.lines().any(|line| line == EXCLUDE_LINE) {
        return Ok(());
    }
    let appended = (|| {
        if let Some(dir) = exclude.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut file = OpenOptions::new().append(true).create(true).open(exclude)?;
        if !existing.is_empty() && !existing.ends_with('\n') {
            file.write_all(b"\n")?;
        }
        file.write_all(format!("{EXCLUDE_LINE}\n").as_bytes())
    })();
    appended.with_context(|| format!("cannot write {}", exclude.display()))
}

/// The shape every project name and slug must have.
pub const NAME_RULE: &str = "^[a-z0-9]+(-[a-z0-9]+)*$";

/// Rejects a typed name that is not kebab-case, naming what it was (`slug`
/// or `project`) and the rule.
pub fn validate_name(kind: &str, name: &str) -> Result<()> {
    if !is_kebab_case(name) {
        bail!("{kind} {name:?} must match {NAME_RULE}");
    }
    Ok(())
}

/// The default project name: the repository name from the `origin` URL, or
/// the toplevel directory name when there is no `origin`, with leading dots
/// dropped so `gulkaran/.agents` becomes `agents`. The name is lowercased
/// before the kebab-case check, so a repository named `World` is project
/// `world`; a name that still fails asks the caller to pass `--project`.
pub fn derive_project(origin: Option<&str>, toplevel: &Path) -> Result<String> {
    let from_origin = origin.map(repository_name).filter(|name| !name.is_empty());
    let derived = match from_origin {
        Some(name) => name,
        None => toplevel
            .file_name()
            .map(|name| name.to_string_lossy().trim_start_matches('.').to_string())
            .unwrap_or_default(),
    };
    let name = derived.to_lowercase();
    if !is_kebab_case(&name) {
        bail!("derived project {name:?} is not kebab-case; pass --project <name>");
    }
    Ok(name)
}

/// The text after the last `/` or `:` of a remote URL, without a trailing `/`,
/// a `.git` suffix, or leading dots.
fn repository_name(url: &str) -> String {
    let trimmed = url.trim_end_matches('/');
    let trimmed = trimmed.strip_suffix(".git").unwrap_or(trimmed);
    let name = trimmed.rsplit(['/', ':']).next().unwrap_or(trimmed);
    name.trim_start_matches('.').to_string()
}

/// Lowercase ASCII letters and digits in hyphen-separated runs, with no run
/// empty: the hand-written form of `^[a-z0-9]+(-[a-z0-9]+)*$`.
fn is_kebab_case(name: &str) -> bool {
    !name.is_empty()
        && name.split('-').all(|run| {
            !run.is_empty()
                && run
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn kebab_case_names_are_valid() {
        for name in ["a", "agents", "accio-cli", "cd-123-retries", "0", "a-1-b"] {
            validate_name("slug", name).unwrap();
        }
    }

    #[test]
    fn names_that_are_not_kebab_case_are_rejected_with_the_rule() {
        for name in [
            "",
            "Bad_Slug",
            "World",
            "-leading",
            "trailing-",
            "double--hyphen",
            "has space",
            "dotted.name",
            "slash/name",
            "caf\u{e9}",
        ] {
            let err = validate_name("slug", name).unwrap_err();
            assert_eq!(
                err.to_string(),
                format!("slug {name:?} must match ^[a-z0-9]+(-[a-z0-9]+)*$")
            );
        }
        let err = validate_name("project", "Some_Repo").unwrap_err();
        assert_eq!(
            err.to_string(),
            "project \"Some_Repo\" must match ^[a-z0-9]+(-[a-z0-9]+)*$"
        );
    }

    #[test]
    fn project_is_derived_from_the_origin_repository_name() {
        let toplevel = Path::new("/work/checkout");
        for (origin, expected) in [
            ("git@github.com:gulkaran/.agents.git", "agents"),
            (
                "https://github.com/Shopify/infra-central.git",
                "infra-central",
            ),
            ("https://github.com/Shopify/infra-central", "infra-central"),
            ("ssh://git@github.com/owner/repo", "repo"),
            ("https://github.com/owner/repo.git/", "repo"),
        ] {
            assert_eq!(derive_project(Some(origin), toplevel).unwrap(), expected);
        }
    }

    #[test]
    fn project_falls_back_to_the_toplevel_basename_without_leading_dots() {
        assert_eq!(
            derive_project(None, Path::new("/Users/me/.agents")).unwrap(),
            "agents"
        );
        assert_eq!(
            derive_project(None, Path::new("/srv/infra-central")).unwrap(),
            "infra-central"
        );
        assert_eq!(
            derive_project(Some(".git"), Path::new("/srv/fallback")).unwrap(),
            "fallback"
        );
    }

    #[test]
    fn derived_project_is_lowercased_before_validation() {
        let toplevel = Path::new("/work/checkout");
        assert_eq!(
            derive_project(Some("https://github.com/Shopify/World.git"), toplevel).unwrap(),
            "world"
        );
        assert_eq!(
            derive_project(None, Path::new("/work/World")).unwrap(),
            "world"
        );
    }

    #[test]
    fn derived_project_that_is_not_kebab_case_asks_for_project_flag() {
        let toplevel = Path::new("/work/checkout");
        let err = derive_project(Some("git@github.com:owner/Some_Repo.git"), toplevel).unwrap_err();
        assert_eq!(
            err.to_string(),
            "derived project \"some_repo\" is not kebab-case; pass --project <name>"
        );
        let err = derive_project(None, Path::new("/")).unwrap_err();
        assert_eq!(
            err.to_string(),
            "derived project \"\" is not kebab-case; pass --project <name>"
        );
    }

    #[test]
    fn topic_is_parsed_from_the_last_four_path_components() {
        let topic =
            Topic::from_path(PathBuf::from("/vault/projects/agents/topics/accio-cli")).unwrap();
        assert_eq!(topic.project, "agents");
        assert_eq!(topic.slug, "accio-cli");
        assert_eq!(
            topic.path,
            PathBuf::from("/vault/projects/agents/topics/accio-cli")
        );
    }

    #[test]
    fn paths_that_are_not_a_vault_topic_are_rejected() {
        for path in [
            "/vault/projects/agents/topics",
            "/vault/projects/agents/accio-cli",
            "/vault/notes/agents/topics/accio-cli",
            "/vault/projects/agents/topics/accio-cli/design.md",
            "projects/agents/topics/accio-cli",
            "/",
            "",
        ] {
            assert!(
                Topic::from_path(PathBuf::from(path)).is_none(),
                "{path:?} should not parse as a topic"
            );
        }
    }
}
