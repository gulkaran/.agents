//! The current topic: the marker at the git toplevel, the exclude line that
//! hides it, and the project and slug names that locate a topic in the vault.

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
