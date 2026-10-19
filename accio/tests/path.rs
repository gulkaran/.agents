mod common;

use common::Harness;

const FILE_ARTIFACTS: [(&str, &str); 4] = [
    ("brief", "brief.md"),
    ("investigation", "investigation.md"),
    ("design", "design.md"),
    ("decisions", "decisions.md"),
];

const DIRECTORY_ARTIFACTS: [&str; 4] = ["research", "feasibility", "verification", "review"];

#[test]
fn path_prints_each_file_artifact_without_creating_it() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    for (artifact, file) in FILE_ARTIFACTS {
        h.accio_in(h.work.path())
            .args(["path", artifact])
            .assert()
            .success()
            .stdout(format!("{}\n", topic.join(file).display()));
        assert!(!topic.join(file).exists(), "{file} must not be created");
    }
}

#[test]
fn path_prints_and_creates_each_directory_artifact() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    for artifact in DIRECTORY_ARTIFACTS {
        assert!(!topic.join(artifact).exists());
        h.accio_in(h.work.path())
            .args(["path", artifact])
            .assert()
            .success()
            .stdout(format!("{}\n", topic.join(artifact).display()));
        assert!(topic.join(artifact).is_dir(), "{artifact} must be created");
        h.accio_in(h.work.path())
            .args(["path", artifact])
            .assert()
            .success()
            .stdout(format!("{}\n", topic.join(artifact).display()));
    }
}

#[test]
fn path_with_an_unknown_artifact_exits_1_naming_the_eight() {
    let h = Harness::new();
    h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "notes"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: unknown artifact \"notes\"; expected one of brief, investigation, design, decisions, research, feasibility, verification, review\n");
}

#[test]
fn path_outside_a_repository_exits_1_with_hint() {
    let h = Harness::new();
    h.accio()
        .args(["path", "design"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: not inside a git repository; run accio from a checkout\n");
}

#[test]
fn path_without_a_marker_exits_1_with_hint() {
    let h = Harness::new();
    h.accio_in(h.work.path())
        .args(["path", "design"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: no .topic here; run accio topic use <slug>\n");
}
