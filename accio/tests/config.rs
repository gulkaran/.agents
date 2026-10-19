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
