mod common;

use std::fs;

use common::Harness;

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
