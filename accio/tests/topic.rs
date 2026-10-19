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
