use assert_cmd::Command;

#[test]
fn it_syncs_aliases_successfully() {
    let mut cmd = Command::cargo_bin("cargo-bin").unwrap();
    let assert = cmd.arg("--sync-aliases").assert();

    assert.success();
}

#[test]
fn it_builds_all_bins_successfully() {
    let mut cmd = Command::cargo_bin("cargo-bin").unwrap();
    let assert = cmd.arg("--build").assert();

    assert.success();
}

#[test]
fn it_runs_nextest_help_successfully() {
    let mut cmd = Command::cargo_bin("cargo-bin").unwrap();
    let assert = cmd.arg("cargo-nextest").arg("--help").assert();

    assert.success();
}

#[test]
fn it_runs_nextest_help_from_cargo_successfully() {
    let mut cmd = Command::cargo_bin("cargo-bin").unwrap();
    let assert = cmd.arg("bin").arg("cargo-nextest").arg("--help").assert();

    assert.success();
}

#[test]
fn it_runs_help_successfully() {
    let mut cmd = Command::cargo_bin("cargo-bin").unwrap();
    let assert = cmd.arg("--help").assert();

    assert.success();
}
