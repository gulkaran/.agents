mod common;

use std::fs;
use std::path::{Path, PathBuf};

use common::Harness;
use predicates::prelude::*;

/// Commits topics `accio-cli` and `other` with a brief and a design each,
/// points the work repository at `accio-cli`, and returns its folder.
fn two_committed_topics(h: &Harness) -> PathBuf {
    let topic = h.topic_with_marker("accio-cli");
    let other = topic.parent().unwrap().join("other");
    fs::create_dir(&other).unwrap();
    for dir in [&topic, &other] {
        fs::write(dir.join("brief.md"), "brief\n").unwrap();
        fs::write(dir.join("design.md"), "design\n").unwrap();
    }
    h.git(h.vault.path(), &["add", "-A"]);
    h.git(h.vault.path(), &["commit", "-q", "-m", "Add two topics"]);
    topic
}

fn lines(text: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = text.lines().collect();
    lines.sort_unstable();
    lines
}

fn short_head(h: &Harness, repo: &Path) -> String {
    h.git(repo, &["rev-parse", "--short", "HEAD"])
        .trim_end()
        .to_string()
}

#[test]
fn commit_records_exactly_the_topic_folder_and_leaves_other_work_alone() {
    let h = Harness::new();
    let topic = two_committed_topics(&h);
    let other = topic.parent().unwrap().join("other");
    fs::write(topic.join("brief.md"), "brief v2\n").unwrap();
    fs::create_dir(topic.join("research")).unwrap();
    fs::write(topic.join("research/new.md"), "new\n").unwrap();
    fs::remove_file(topic.join("design.md")).unwrap();
    fs::write(other.join("brief.md"), "other brief v2\n").unwrap();
    h.git(
        h.vault.path(),
        &["add", "projects/agents/topics/other/brief.md"],
    );
    fs::write(other.join("design.md"), "other design v2\n").unwrap();

    h.accio_in(h.work.path())
        .args(["commit", "-m", "save the research"])
        .assert()
        .success()
        .stdout(predicate::str::ends_with(
            " docs(accio-cli): save the research\n",
        ));

    let vault = h.vault.path();
    assert_eq!(
        h.git(vault, &["log", "-1", "--format=%s"]).trim_end(),
        "docs(accio-cli): save the research"
    );
    assert_eq!(
        lines(&h.git(vault, &["show", "--name-status", "--format=", "HEAD"])),
        [
            "A\tprojects/agents/topics/accio-cli/research/new.md",
            "D\tprojects/agents/topics/accio-cli/design.md",
            "M\tprojects/agents/topics/accio-cli/brief.md",
        ]
    );
    assert_eq!(
        h.git(vault, &["diff", "--cached", "--name-only"])
            .trim_end(),
        "projects/agents/topics/other/brief.md"
    );
    assert_eq!(
        h.git(vault, &["diff", "--name-only"]).trim_end(),
        "projects/agents/topics/other/design.md"
    );
}

#[test]
fn commit_prints_the_short_sha_and_subject_and_passes_the_message_through() {
    let h = Harness::new();
    let topic = two_committed_topics(&h);
    fs::write(topic.join("brief.md"), "brief v2\n").unwrap();
    let message = "say \"hi\"  to  'everyone' & co";

    h.accio_in(h.work.path())
        .args(["commit", "-m", message])
        .assert()
        .success()
        .stdout(format!(
            "{} docs(accio-cli): {message}\n",
            short_head(&h, h.vault.path())
        ));

    assert_eq!(
        h.git(h.vault.path(), &["log", "-1", "--format=%s"])
            .trim_end(),
        format!("docs(accio-cli): {message}")
    );
}

#[test]
fn commit_never_pushes_to_origin() {
    let h = Harness::new();
    let topic = two_committed_topics(&h);
    let bare = h.home.path().join("origin.git");
    h.git(
        h.home.path(),
        &["init", "-q", "--bare", bare.to_str().unwrap()],
    );
    h.git(
        h.vault.path(),
        &["remote", "add", "origin", bare.to_str().unwrap()],
    );
    let before = h.git(h.home.path(), &["ls-remote", bare.to_str().unwrap()]);
    fs::write(topic.join("brief.md"), "brief v2\n").unwrap();

    h.accio_in(h.work.path())
        .args(["commit", "-m", "stay local"])
        .assert()
        .success();

    assert_eq!(
        h.git(h.vault.path(), &["log", "-1", "--format=%s"])
            .trim_end(),
        "docs(accio-cli): stay local"
    );
    assert_eq!(
        h.git(h.home.path(), &["ls-remote", bare.to_str().unwrap()]),
        before
    );
    assert_eq!(before, "");
}

#[test]
fn commit_makes_the_first_commit_of_an_unborn_vault() {
    let h = Harness::new();
    let fresh = h.home.path().join("fresh");
    let topic = fresh.join("projects/agents/topics/first");
    fs::create_dir_all(&topic).unwrap();
    h.git(&fresh, &["init", "-q", "-b", "main"]);
    fs::write(topic.join("brief.md"), "brief\n").unwrap();
    let topic = fs::canonicalize(&topic).unwrap();
    fs::write(
        h.work.path().join(".topic"),
        format!("{}\n", topic.display()),
    )
    .unwrap();

    h.accio_in(h.work.path())
        .args(["commit", "-m", "first"])
        .assert()
        .success()
        .stdout(format!("{} docs(first): first\n", short_head(&h, &fresh)));

    assert_eq!(
        h.git(&fresh, &["log", "--format=%s"]).trim_end(),
        "docs(first): first"
    );
    assert_eq!(
        h.git(&fresh, &["show", "--name-status", "--format=", "HEAD"])
            .trim_end(),
        "A\tprojects/agents/topics/first/brief.md"
    );
}

#[test]
fn commit_leaves_the_topic_staged_when_a_hook_rejects_it() {
    let h = Harness::new();
    let topic = two_committed_topics(&h);
    let hooks = h.vault.path().join(".git/hooks");
    fs::create_dir_all(&hooks).unwrap();
    let hook = hooks.join("pre-commit");
    fs::write(&hook, "#!/bin/sh\necho rejected >&2\nexit 1\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    }
    fs::write(topic.join("brief.md"), "brief v2\n").unwrap();

    h.accio_in(h.work.path())
        .args(["commit", "-m", "blocked"])
        .assert()
        .code(1)
        .stdout("")
        .stderr(
            predicate::str::starts_with("accio: git commit ")
                .and(predicate::str::contains("rejected")),
        );

    assert_eq!(
        h.git(h.vault.path(), &["diff", "--cached", "--name-only"])
            .trim_end(),
        "projects/agents/topics/accio-cli/brief.md"
    );
    assert_eq!(
        h.git(h.vault.path(), &["log", "-1", "--format=%s"])
            .trim_end(),
        "Add two topics"
    );

    fs::remove_file(&hook).unwrap();
    h.accio_in(h.work.path())
        .args(["commit", "-m", "unblocked"])
        .assert()
        .success();
    assert_eq!(
        h.git(h.vault.path(), &["log", "-1", "--format=%s"])
            .trim_end(),
        "docs(accio-cli): unblocked"
    );
}

#[test]
fn commit_with_a_clean_topic_exits_1_and_records_nothing() {
    let h = Harness::new();
    let topic = two_committed_topics(&h);
    let other = topic.parent().unwrap().join("other");
    fs::write(other.join("brief.md"), "other brief v2\n").unwrap();
    h.git(
        h.vault.path(),
        &["add", "projects/agents/topics/other/brief.md"],
    );

    h.accio_in(h.work.path())
        .args(["commit", "-m", "nothing to save"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: nothing changed in agents/accio-cli\n");

    assert_eq!(
        h.git(h.vault.path(), &["log", "-1", "--format=%s"])
            .trim_end(),
        "Add two topics"
    );
    assert_eq!(
        h.git(h.vault.path(), &["diff", "--cached", "--name-only"])
            .trim_end(),
        "projects/agents/topics/other/brief.md"
    );
}
