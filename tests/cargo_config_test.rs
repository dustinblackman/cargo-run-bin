mod sync_aliases {
    use std::fs;

    use cargo_run_bin::cargo_config::sync_aliases;

    #[test]
    fn it_removes_and_adds_aliases() {
        let res = sync_aliases();
        let toml_str: String = fs::read_to_string(".cargo/config.toml")
            .unwrap()
            .parse()
            .unwrap();

        assert!(res.is_ok());
        assert!(toml_str.contains("nextest = [\"bin\", \"cargo-nextest\"]"));
    }
}

mod binstall_alias_exists {
    use cargo_run_bin::cargo_config::binstall_alias_exists;

    // Lazy happy path test.
    #[test]
    fn it_should_return_true() {
        let res = binstall_alias_exists().unwrap();
        assert!(res);
    }
}
