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

mod direct {
    use super::*;

    #[test]
    fn it_syncs_aliases_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("--sync-aliases").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        insta::assert_snapshot!(stdout, @r###"
    [32mDone![39m
    "###);
    }

    #[test]
    fn it_install_all_bins_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("--install").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        insta::assert_snapshot!(stdout, @r###"
    [32mDone![39m
    "###);
    }

    #[test]
    fn it_builds_all_bins_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("--build").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        insta::assert_snapshot!(stdout, @r###"
    [32mDone![39m
    "###);
    }

    #[test]
    fn it_runs_help_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("--help").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        assert!(stdout.contains(env!("CARGO_PKG_DESCRIPTION")));
    }

    #[test]
    fn it_runs_nextest_help_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("cargo-nextest").arg("--help").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        assert!(stdout.contains("cargo nextest"));
    }

    #[test]
    fn it_fails_when_binary_is_not_configured() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("not-real").assert();

        let res = assert.failure();
        let stderr = String::from_utf8(res.get_output().stderr.clone()).unwrap();
        insta::assert_snapshot!(stderr, @r###"
    [31mrun-bin failed: No package found for binary not-real[39m
    "###);
    }
}

mod bin_prefix {
    use super::*;

    #[test]
    fn it_syncs_aliases_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("bin").arg("--sync-aliases").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        insta::assert_snapshot!(stdout, @r###"
    [32mDone![39m
    "###);
    }

    #[test]
    fn it_install_all_bins_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("bin").arg("--install").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        insta::assert_snapshot!(stdout, @r###"
    [32mDone![39m
    "###);
    }

    #[test]
    fn it_builds_all_bins_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("bin").arg("--build").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        insta::assert_snapshot!(stdout, @r###"
    [32mDone![39m
    "###);
    }

    #[test]
    fn it_runs_nextest_help_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("bin").arg("cargo-nextest").arg("--help").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        assert!(stdout.contains("cargo nextest"));
    }

    #[test]
    fn it_runs_help_successfully() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("bin").arg("--help").assert();

        let res = assert.success();
        let stdout = String::from_utf8(res.get_output().stdout.clone()).unwrap();
        assert!(stdout.contains(env!("CARGO_PKG_DESCRIPTION")));
    }

    #[test]
    fn it_fails_when_binary_is_not_configured() {
        let mut cmd = Command::cargo_bin(get_bin()).unwrap();
        let assert = cmd.arg("bin").arg("not-real").assert();

        let res = assert.failure();
        let stderr = String::from_utf8(res.get_output().stderr.clone()).unwrap();
        insta::assert_snapshot!(stderr, @r###"
    [31mrun-bin failed: No package found for binary not-real[39m
    "###);
    }
}
