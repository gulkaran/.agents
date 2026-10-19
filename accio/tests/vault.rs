mod common;

use std::fs;

use common::Harness;

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
