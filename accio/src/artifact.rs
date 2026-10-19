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
