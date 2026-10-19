//! The current topic: the marker at the git toplevel, the exclude line that
//! hides it, and the project and slug names that locate a topic in the vault.

use anyhow::{Result, bail};

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
}
