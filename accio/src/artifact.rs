//! The eight artifacts a topic folder can hold, in canonical order.

use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_names_without_parent_components_are_valid() {
        for name in ["evidence.md", "screenshots/login.png", "a/b/c", "round-1.md"] {
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
}
