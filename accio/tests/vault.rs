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
