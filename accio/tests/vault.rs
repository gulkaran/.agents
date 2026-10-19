mod common;

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
