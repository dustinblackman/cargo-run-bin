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
