mod common;

use std::fs;

use common::Harness;
use predicates::prelude::*;

#[test]
fn vault_path_without_config_exits_1_with_hint() {
    let h = Harness::new();
    h.accio()
        .args(["vault", "path"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: no vault configured; run accio vault set <path>\n");
}

#[test]
fn vault_set_stores_the_canonical_repository_root() {
    let h = Harness::new();
    let canonical = fs::canonicalize(h.vault.path()).unwrap();
    let line = format!("{}\n", canonical.display());

    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success()
        .stdout(line.clone());

    assert_eq!(
        fs::read_to_string(h.config_path()).unwrap(),
        format!("vault = \"{}\"\n", canonical.display())
    );
    h.accio()
        .args(["vault", "path"])
        .assert()
        .success()
        .stdout(line);
}

#[test]
fn vault_set_rejects_a_missing_path_and_writes_nothing() {
    let h = Harness::new();
    let missing = h.home.path().join("missing");

    h.accio()
        .args(["vault", "set"])
        .arg(&missing)
        .assert()
        .code(1)
        .stdout("")
        .stderr(format!(
            "accio: vault cannot be found: {} is not a directory\n",
            missing.display()
        ));

    assert!(!h.config_path().exists());
}

#[test]
fn vault_set_rejects_a_directory_that_is_not_a_repository() {
    let h = Harness::new();
    let plain = h.home.path().join("plain");
    fs::create_dir(&plain).unwrap();

    h.accio()
        .args(["vault", "set"])
        .arg(&plain)
        .assert()
        .code(1)
        .stdout("")
        .stderr(
            predicate::str::starts_with("accio: vault cannot be found: ")
                .and(predicate::str::contains("not a git repository")),
        );

    assert!(!h.config_path().exists());
}

#[test]
fn vault_set_rejects_a_subdirectory_of_a_repository() {
    let h = Harness::new();
    let sub = h.vault.path().join("projects");
    fs::create_dir(&sub).unwrap();
    let canonical_sub = fs::canonicalize(&sub).unwrap();
    let canonical_root = fs::canonicalize(h.vault.path()).unwrap();

    h.accio()
        .args(["vault", "set"])
        .arg(&sub)
        .assert()
        .code(1)
        .stdout("")
        .stderr(format!(
            "accio: vault cannot be found: {} is not a repository root; run accio vault set {}\n",
            canonical_sub.display(),
            canonical_root.display()
        ));

    assert!(!h.config_path().exists());
}
