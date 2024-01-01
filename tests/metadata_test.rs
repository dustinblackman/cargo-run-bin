mod get_binary_packages {
    use cargo_run_bin::metadata::get_binary_packages;

    #[test]
    fn it_returns_locked_packages() {
        let binary_packages = get_binary_packages().unwrap();
        let nextest = binary_packages
            .iter()
            .find(|&e| e.package == "cargo-nextest");

        assert!(nextest.is_some());
        let res = nextest.unwrap();
        assert_eq!(&res.package, "cargo-nextest");
        assert_ne!(&res.version, "");
        assert!(res.locked.unwrap());
    }

    #[test]
    fn it_returns_bin_target_packages() {
        let binary_packages = get_binary_packages().unwrap();
        let android = binary_packages.iter().find(|&e| {
            e.bin_target.is_some() && e.bin_target.clone().unwrap() == "hello-world-first"
        });

        assert!(android.is_some());
        let res = android.unwrap();
        assert_eq!(&res.package, "dustinblackman-hello-world");
        assert_ne!(&res.version, "");
        assert_eq!(
            &res.git,
            &Some("https://github.com/dustinblackman/rust-hello-world".to_string())
        );
        assert_eq!(res.bin_target.clone().unwrap(), "hello-world-first");
    }
}
