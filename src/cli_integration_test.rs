use assert_cmd::Command;

use crate::metadata;

fn get_bin() -> String {
    return metadata::get_project_root()
        .unwrap()
        .join("target/debug/cargo-bin")
        .to_str()
        .unwrap()
        .to_string();
}

#[test]
fn it_syncs_aliases_successfully() {
    let mut cmd = Command::cargo_bin(get_bin()).unwrap();
    let assert = cmd.arg("--sync-aliases").assert();

    assert.success();
}

#[test]
fn it_install_all_bins_successfully() {
    let mut cmd = Command::cargo_bin(get_bin()).unwrap();
    let assert = cmd.arg("--install").assert();

    assert.success();
}

#[test]
fn it_builds_all_bins_successfully() {
    let mut cmd = Command::cargo_bin(get_bin()).unwrap();
    let assert = cmd.arg("--build").assert();

    assert.success();
}

#[test]
fn it_runs_nextest_help_successfully() {
    let mut cmd = Command::cargo_bin(get_bin()).unwrap();
    let assert = cmd.arg("cargo-nextest").arg("--help").assert();

    assert.success();
}

#[test]
fn it_runs_nextest_help_from_cargo_successfully() {
    let mut cmd = Command::cargo_bin(get_bin()).unwrap();
    let assert = cmd.arg("bin").arg("cargo-nextest").arg("--help").assert();

    assert.success();
}

#[test]
fn it_runs_help_successfully() {
    let mut cmd = Command::cargo_bin(get_bin()).unwrap();
    let assert = cmd.arg("--help").assert();

    assert.success();
}
