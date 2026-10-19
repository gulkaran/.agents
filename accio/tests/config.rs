mod common;

use std::fs;

use common::Harness;
use serde_json::json;

#[test]
fn config_set_creates_the_directory_and_file() {
    let h = Harness::new();
    assert!(!h.config_path().parent().unwrap().exists());

    h.accio()
        .args(["config", "set", "tracker", "github"])
        .assert()
        .success()
        .stdout("");

    assert_eq!(
        fs::read_to_string(h.config_path()).unwrap(),
        "tracker = \"github\"\n"
    );
}

#[test]
fn config_get_prints_a_stored_value() {
    let h = Harness::new();
    h.accio()
        .args(["config", "set", "tracker", "github"])
        .assert()
        .success();

    h.accio()
        .args(["config", "get", "tracker"])
        .assert()
        .success()
        .stdout("github\n");
}

#[test]
fn config_get_of_a_missing_key_exits_1_with_hint() {
    let h = Harness::new();
    h.accio()
        .args(["config", "get", "missing"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: no value for missing; run accio config set missing <value>\n");
}

#[test]
fn config_set_vault_is_refused_and_writes_nothing() {
    let h = Harness::new();

    h.accio()
        .args(["config", "set", "vault", "/somewhere"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: vault is managed by accio vault set; run accio vault set <path>\n");

    assert!(!h.config_path().exists());
}

#[test]
fn config_get_vault_reads_the_value_written_by_vault_set() {
    let h = Harness::new();
    let canonical = fs::canonicalize(h.vault.path()).unwrap();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();

    h.accio()
        .args(["config", "get", "vault"])
        .assert()
        .success()
        .stdout(format!("{}\n", canonical.display()));
}

#[test]
fn config_list_prints_key_value_lines_sorted_by_key() {
    let h = Harness::new();
    let canonical = fs::canonicalize(h.vault.path()).unwrap();
    h.accio().args(["config", "set", "zeta", "last"]).assert().success();
    h.accio()
        .args(["vault", "set"])
        .arg(h.vault.path())
        .assert()
        .success();
    h.accio().args(["config", "set", "alpha", "first"]).assert().success();

    h.accio()
        .args(["config", "list"])
        .assert()
        .success()
        .stdout(format!("alpha: first\nvault: {}\nzeta: last\n", canonical.display()));
}

#[test]
fn config_list_json_prints_an_object() {
    let h = Harness::new();
    h.accio().args(["config", "set", "zeta", "last"]).assert().success();
    h.accio().args(["config", "set", "alpha", "first"]).assert().success();

    let output = h
        .accio()
        .args(["config", "list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let parsed: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(parsed, json!({"alpha": "first", "zeta": "last"}));
}

#[test]
fn config_set_rejects_a_key_that_needs_quoting() {
    let h = Harness::new();

    h.accio()
        .args(["config", "set", "bad key", "value"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("accio: config key \"bad key\" must match ^[A-Za-z0-9_-]+$\n");

    assert!(!h.config_path().exists());
}
