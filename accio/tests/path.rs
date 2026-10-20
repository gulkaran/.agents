mod common;

use std::fs;

use common::Harness;
use predicates::prelude::*;

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

#[test]
fn path_name_prints_the_entry_and_creates_only_its_directory() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "research", "--name", "evidence.md"])
        .assert()
        .success()
        .stdout(format!(
            "{}\n",
            topic.join("research/evidence.md").display()
        ));

    assert!(topic.join("research").is_dir());
    assert!(!topic.join("research/evidence.md").exists());
}

#[test]
fn path_name_with_a_subfolder_creates_the_subfolder() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "verification", "--name", "screenshots/login.png"])
        .assert()
        .success()
        .stdout(format!(
            "{}\n",
            topic.join("verification/screenshots/login.png").display()
        ));

    assert!(topic.join("verification/screenshots").is_dir());
    assert!(!topic.join("verification/screenshots/login.png").exists());
}

#[test]
fn path_name_ending_in_a_dot_component_creates_the_printed_directory() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "research", "--name", "."])
        .assert()
        .success()
        .stdout(format!("{}\n", topic.join("research/.").display()));
    assert!(topic.join("research").is_dir());

    h.accio_in(h.work.path())
        .args(["path", "verification", "--name", "shots/."])
        .assert()
        .success()
        .stdout(format!(
            "{}\n",
            topic.join("verification/shots/.").display()
        ));
    assert!(topic.join("verification/shots").is_dir());

    assert_eq!(fs::read_dir(topic.join("research")).unwrap().count(), 0);
    assert_eq!(
        fs::read_dir(topic.join("verification/shots"))
            .unwrap()
            .count(),
        0
    );
}

#[test]
fn path_name_on_a_file_artifact_exits_1_and_creates_nothing() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "brief", "--name", "draft.md"])
        .assert()
        .code(1)
        .stdout("")
        .stderr(
            "accio: brief is a file; --name applies only to research, feasibility, verification, and review\n",
        );

    assert_eq!(fs::read_dir(&topic).unwrap().count(), 0);
}

#[test]
fn path_name_that_leaves_the_directory_exits_1_and_creates_nothing() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "research", "--name", "../design.md"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: --name \"../design.md\" must not contain \"..\" components\n");
    h.accio_in(h.work.path())
        .args(["path", "research", "--name", "/tmp/evidence.md"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: --name \"/tmp/evidence.md\" must be a relative path\n");
    h.accio_in(h.work.path())
        .args(["path", "research", "--name", ""])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: --name must not be empty\n");

    assert!(!topic.join("research").exists());
}

#[test]
fn path_review_next_prints_round_1_and_creates_review_when_missing() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "review", "--next"])
        .assert()
        .success()
        .stdout(format!("{}\n", topic.join("review/round-1.md").display()));

    assert!(topic.join("review").is_dir());
    assert!(!topic.join("review/round-1.md").exists());
}

#[test]
fn path_review_next_follows_the_highest_round_and_ignores_other_entries() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");
    let review = topic.join("review");
    fs::create_dir_all(review.join("round-2")).unwrap();
    for file in [
        "round-1.md",
        "round-3.md",
        "round-01.md",
        "notes.md",
        "round-2.md.bak",
    ] {
        fs::write(review.join(file), "").unwrap();
    }

    h.accio_in(h.work.path())
        .args(["path", "review", "--next"])
        .assert()
        .success()
        .stdout(format!("{}\n", review.join("round-4.md").display()));

    assert!(!review.join("round-4.md").exists());
}

#[test]
fn path_next_on_another_artifact_exits_1_and_creates_nothing() {
    let h = Harness::new();
    let topic = h.topic_with_marker("accio-cli");

    for artifact in ["research", "brief"] {
        h.accio_in(h.work.path())
            .args(["path", artifact, "--next"])
            .assert()
            .code(1)
            .stdout("")
            .stderr("accio: --next applies only to review; run accio path review --next\n");
    }

    assert_eq!(fs::read_dir(&topic).unwrap().count(), 0);
}

#[test]
fn path_name_and_next_are_a_usage_error_together() {
    let h = Harness::new();
    h.topic_with_marker("accio-cli");

    h.accio_in(h.work.path())
        .args(["path", "review", "--name", "round-9.md", "--next"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(predicate::str::contains("cannot be used with"));
}
