mod common;

use std::fs;

use common::Harness;

#[test]
fn topic_current_outside_a_repository_exits_1_with_hint() {
    let h = Harness::new();
    h.accio()
        .args(["topic", "current"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: not inside a git repository; run accio from a checkout\n");
}

#[test]
fn topic_current_without_a_marker_exits_1_with_hint() {
    let h = Harness::new();
    h.accio_in(h.work.path())
        .args(["topic", "current"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: no .topic here; run accio topic use <slug>\n");
}

#[test]
fn topic_current_with_a_marker_outside_a_vault_topic_exits_1_with_hint() {
    let h = Harness::new();
    let marker = fs::canonicalize(h.work.path()).unwrap().join(".topic");
    fs::write(&marker, "/somewhere/else\n").unwrap();

    h.accio_in(h.work.path())
        .args(["topic", "current"])
        .assert()
        .code(1)
        .stdout("")
        .stderr(format!(
            "accio: .topic at {} does not point into a vault topic; run accio topic use <slug>\n",
            marker.display()
        ));
}

#[test]
fn topic_current_with_a_stale_marker_exits_1_with_hint() {
    let h = Harness::new();
    let gone = fs::canonicalize(h.vault.path())
        .unwrap()
        .join("projects/agents/topics/gone");
    fs::write(
        h.work.path().join(".topic"),
        format!("{}\n", gone.display()),
    )
    .unwrap();

    h.accio_in(h.work.path())
        .args(["topic", "current"])
        .assert()
        .code(1)
        .stdout("")
        .stderr(format!(
            "accio: .topic points to {}, which does not exist; run accio topic use <slug>\n",
            gone.display()
        ));
}

#[test]
fn topic_use_creates_the_folder_and_marker_then_reuses_them() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let topic = fs::canonicalize(h.vault.path())
        .unwrap()
        .join("projects/agents/topics/http-retry-budget");
    let marker = h.work.path().join(".topic");

    h.accio_in(h.work.path())
        .args(["topic", "use", "http-retry-budget"])
        .assert()
        .success()
        .stdout(format!("created {}\n", topic.display()));

    assert!(topic.is_dir());
    assert_eq!(
        fs::read_to_string(&marker).unwrap(),
        format!("{}\n", topic.display())
    );

    h.accio_in(h.work.path())
        .args(["topic", "use", "http-retry-budget"])
        .assert()
        .success()
        .stdout(format!("using {}\n", topic.display()));

    assert_eq!(
        fs::read_to_string(&marker).unwrap(),
        format!("{}\n", topic.display())
    );
}

#[test]
fn topic_use_switches_the_marker_to_another_topic() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let topics = fs::canonicalize(h.vault.path())
        .unwrap()
        .join("projects/agents/topics");
    h.accio_in(h.work.path())
        .args(["topic", "use", "first"])
        .assert()
        .success();

    h.accio_in(h.work.path())
        .args(["topic", "use", "second"])
        .assert()
        .success()
        .stdout(format!("created {}\n", topics.join("second").display()));

    assert!(topics.join("first").is_dir());
    assert_eq!(
        fs::read_to_string(h.work.path().join(".topic")).unwrap(),
        format!("{}\n", topics.join("second").display())
    );
}

#[test]
fn topic_use_derives_a_lowercase_project_from_the_origin_repository() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    h.git(
        h.work.path(),
        &["remote", "set-url", "origin", "https://github.com/Shopify/World.git"],
    );
    let topic = fs::canonicalize(h.vault.path())
        .unwrap()
        .join("projects/world/topics/checkout-retries");

    h.accio_in(h.work.path())
        .args(["topic", "use", "checkout-retries"])
        .assert()
        .success()
        .stdout(format!("created {}\n", topic.display()));

    assert!(topic.is_dir());
}

#[test]
fn topic_use_without_a_vault_exits_1_with_hint_and_writes_nothing() {
    let h = Harness::new();

    h.accio_in(h.work.path())
        .args(["topic", "use", "http-retry-budget"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: no vault configured; run accio vault set <path>\n");

    assert!(!h.work.path().join(".topic").exists());
    assert!(!h.vault.path().join("projects").exists());
}
