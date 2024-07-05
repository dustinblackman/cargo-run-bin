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