use super::*;

mod install {
    use super::*;
    use crate::metadata;

    #[test]
    fn it_returns_bin_path() {
        let binary_packages = metadata::get_binary_packages().unwrap();
        let nextest = binary_packages
            .iter()
            .find(|&e| {
                return e.package == "cargo-nextest";
            })
            .unwrap();

        let cache_bin_path = install(nextest.clone()).unwrap();
        assert!(cache_bin_path.ends_with("/bin/cargo-nextest"));
    }
}

mod cargo_install {
    use super::*;

    #[test]
    fn it_builds_successfully() {
        let res = cargo_install(
            metadata::BinaryPackage {
                bin_target: None,
                package: "dustinblackman-hello-world".to_string(),
                locked: None,
                version: "0.1.0".to_string(),
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }

    #[test]
    fn it_builds_successfully_with_git() {
        let res = cargo_install(
            metadata::BinaryPackage {
                bin_target: None,
                package: "dustinblackman-hello-world".to_string(),
                locked: None,
                version: "0.2.0".to_string(),
                git: Some("https://github.com/dustinblackman/rust-hello-world".to_string()),
                branch: None,
                tag: None,
                rev: None,
                path: None,
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }

    #[test]
    fn it_builds_successfully_with_git_rev() {
        let res = cargo_install(
            metadata::BinaryPackage {
                bin_target: None,
                package: "dustinblackman-hello-world".to_string(),
                locked: None,
                version: "0.2.0".to_string(),
                git: Some("https://github.com/dustinblackman/rust-hello-world".to_string()),
                branch: None,
                tag: None,
                rev: Some("8a1cd3d2538460d1e8920bf86cf6e2aa982eb69d".to_string()),
                path: None,
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }

    #[test]
    fn it_builds_successfully_with_git_branch() {
        let res = cargo_install(
            metadata::BinaryPackage {
                bin_target: None,
                package: "dustinblackman-hello-world".to_string(),
                locked: None,
                version: "0.2.0".to_string(),
                git: Some("https://github.com/dustinblackman/rust-hello-world".to_string()),
                branch: Some("testbranch".to_string()),
                tag: None,
                rev: None,
                path: None,
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }

    #[test]
    fn it_builds_successfully_with_git_tag() {
        let res = cargo_install(
            metadata::BinaryPackage {
                bin_target: None,
                package: "dustinblackman-hello-world".to_string(),
                locked: None,
                version: "0.2.1".to_string(),
                git: Some("https://github.com/dustinblackman/rust-hello-world".to_string()),
                branch: None,
                tag: Some("v0.2.1".to_string()),
                rev: None,
                path: None,
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }

    #[test]
    fn it_builds_successfully_with_path() {
        let res = cargo_install(
            metadata::BinaryPackage {
                bin_target: None,
                package: "cargo-run-bin".to_string(),
                locked: None,
                version: "1.4.1".to_string(),
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: Some(String::from(".")),
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }
}

mod binstall {
    use super::*;

    #[test]
    fn it_builds_successfully() {
        let res = binstall(
            metadata::BinaryPackage {
                bin_target: None,
                package: "dustinblackman-hello-world".to_string(),
                locked: None,
                version: "0.1.0".to_string(),
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }

    #[test]
    fn it_builds_successfully_with_git() {
        let res = binstall(
            metadata::BinaryPackage {
                bin_target: None,
                package: "dustinblackman-hello-world".to_string(),
                locked: None,
                version: "0.1.0".to_string(),
                git: Some("https://github.com/dustinblackman/rust-hello-world".to_string()),
                branch: None,
                tag: None,
                rev: None,
                path: None,
                default_features: None,
                features: None,
            },
            "./target/debug".into(),
        );

        assert!(res.is_ok());
    }
}

mod run {
    use super::*;
    use crate::metadata;

    #[test]
    fn it_runs_help_successfully() {
        let binary_packages = metadata::get_binary_packages().unwrap();
        let nextest = binary_packages
            .iter()
            .find(|&e| {
                return e.package == "cargo-nextest";
            })
            .unwrap();
        let cache_bin_path = install(nextest.clone()).unwrap();

        let res = run(cache_bin_path, vec!["--help".to_string()]);
        assert!(res.is_ok());
    }
}
