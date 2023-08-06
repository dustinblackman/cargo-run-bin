use expectest::prelude::*;

use super::*;

mod get_pkg_version {
    use super::*;

    #[test]
    fn return_a_name_and_version_number() {
        let res = get_pkg_version("cargo-llvm-cov").unwrap();
        expect!(res.name).to(be_equal_to("cargo-llvm-cov"));
        expect!(res.version).to(be_equal_to("0.1.3"));
    }

    #[test]
    fn return_an_error_when_a_package_is_not_found() {
        match get_pkg_version("does-not-exist") {
            Ok(_res) => panic!("Function should of not succeeded"),
            Err(err) => {
                expect!(err.to_string())
                    .to(be_equal_to("Package for binary does-not-exist not found"));
            }
        }
    }
}

mod get_binaries {
    use super::*;

    #[test]
    fn should_execute_successfully() {
        let mut res = get_binaries().unwrap();
        res.sort();
        expect!(res).to(be_equal_to(vec![
            "cargo-deny",
            "cargo-llvm-cov",
            "dprint",
            "petname",
        ]));
    }
}

mod run_binary {
    use super::*;

    #[test]
    fn cargo_binary_execute_successfuly() {
        let mut args: Vec<String> = vec![
            "cargo".to_owned(),
            "bin".to_owned(),
            "cargo-llvm-cov".to_owned(),
            "--help".to_owned(),
        ];
        run_binary(&mut args).unwrap();
    }

    #[test]
    fn regular_binary_execute_successfuly() {
        let mut args: Vec<String> = vec![
            "cargo".to_owned(),
            "bin".to_owned(),
            "petname".to_owned(),
            "--help".to_owned(),
        ];
        run_binary(&mut args).unwrap();
    }
}
