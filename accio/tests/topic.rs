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

fn exclude_path(h: &Harness, dir: &std::path::Path) -> std::path::PathBuf {
    let printed = h.git(
        dir,
        &["rev-parse", "--path-format=absolute", "--git-path", "info/exclude"],
    );
    std::path::PathBuf::from(printed.trim_end())
}

fn count_marker_lines(text: &str) -> usize {
    text.lines().filter(|line| *line == "/.topic").count()
}

#[test]
fn topic_use_adds_the_exclude_line_once_across_two_runs() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let exclude = exclude_path(&h, h.work.path());
    let before = fs::read_to_string(&exclude).unwrap();
    assert_eq!(count_marker_lines(&before), 0);
    assert!(before.ends_with('\n'));

    h.accio_in(h.work.path())
        .args(["topic", "use", "first"])
        .assert()
        .success();
    h.accio_in(h.work.path())
        .args(["topic", "use", "second"])
        .assert()
        .success();

    let after = fs::read_to_string(&exclude).unwrap();
    assert_eq!(after, format!("{before}/.topic\n"));
    assert_eq!(count_marker_lines(&after), 1);
    assert_eq!(h.git(h.work.path(), &["status", "--porcelain"]), "");
}

#[test]
fn topic_use_creates_a_missing_exclude_file() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let exclude = exclude_path(&h, h.work.path());
    fs::remove_dir_all(exclude.parent().unwrap()).unwrap();

    h.accio_in(h.work.path())
        .args(["topic", "use", "first"])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&exclude).unwrap(), "/.topic\n");
}

#[test]
fn topic_use_starts_a_new_line_when_the_exclude_file_lacks_a_trailing_newline() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let exclude = exclude_path(&h, h.work.path());
    fs::write(&exclude, "*.log").unwrap();

    h.accio_in(h.work.path())
        .args(["topic", "use", "first"])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&exclude).unwrap(), "*.log\n/.topic\n");
}

#[test]
fn topic_use_from_a_linked_worktree_shares_the_exclude_line_and_owns_its_marker() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let topics = fs::canonicalize(h.vault.path())
        .unwrap()
        .join("projects/agents/topics");
    let worktree = h.home.path().join("wt");
    h.git(
        h.work.path(),
        &["worktree", "add", "-q", worktree.to_str().unwrap(), "-b", "feature"],
    );
    let worktree = fs::canonicalize(&worktree).unwrap();
    h.accio_in(h.work.path())
        .args(["topic", "use", "first"])
        .assert()
        .success();

    h.accio_in(&worktree)
        .args(["topic", "use", "second"])
        .assert()
        .success()
        .stdout(format!("created {}\n", topics.join("second").display()));

    let exclude = exclude_path(&h, h.work.path());
    assert_eq!(exclude, exclude_path(&h, &worktree));
    assert_eq!(count_marker_lines(&fs::read_to_string(&exclude).unwrap()), 1);
    assert_eq!(
        fs::read_to_string(h.work.path().join(".topic")).unwrap(),
        format!("{}\n", topics.join("first").display())
    );
    assert_eq!(
        fs::read_to_string(worktree.join(".topic")).unwrap(),
        format!("{}\n", topics.join("second").display())
    );
    assert_eq!(h.git(h.work.path(), &["status", "--porcelain"]), "");
    assert_eq!(h.git(&worktree, &["status", "--porcelain"]), "");
}

#[test]
fn topic_use_rejects_a_slug_that_is_not_kebab_case_and_writes_nothing() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let exclude = exclude_path(&h, h.work.path());

    h.accio_in(h.work.path())
        .args(["topic", "use", "Bad_Slug"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: slug \"Bad_Slug\" must match ^[a-z0-9]+(-[a-z0-9]+)*$\n");

    assert!(!h.vault.path().join("projects").exists());
    assert!(!h.work.path().join(".topic").exists());
    assert_eq!(
        count_marker_lines(&fs::read_to_string(&exclude).unwrap()),
        0
    );
}

#[test]
fn topic_use_honors_the_project_override() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    let topic = fs::canonicalize(h.vault.path())
        .unwrap()
        .join("projects/backlog/topics/cd-123-retries");

    h.accio_in(h.work.path())
        .args(["topic", "use", "cd-123-retries", "--project", "backlog"])
        .assert()
        .success()
        .stdout(format!("created {}\n", topic.display()));

    assert!(topic.is_dir());
    assert!(!h.vault.path().join("projects/agents").exists());
    assert_eq!(
        fs::read_to_string(h.work.path().join(".topic")).unwrap(),
        format!("{}\n", topic.display())
    );
}

#[test]
fn topic_use_rejects_a_project_override_that_is_not_kebab_case_and_writes_nothing() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();

    h.accio_in(h.work.path())
        .args(["topic", "use", "cd-123-retries", "--project", "Bad_Project"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: project \"Bad_Project\" must match ^[a-z0-9]+(-[a-z0-9]+)*$\n");

    assert!(!h.vault.path().join("projects").exists());
    assert!(!h.work.path().join(".topic").exists());
}
