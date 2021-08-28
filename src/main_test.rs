use super::*;
use expectest::prelude::*;
use speculate::speculate;

speculate! {
    describe "get_pkg_version" {
        it "should return a name and version number" {
            let res = get_pkg_version("cargo-llvm-cov").unwrap();
            expect!(res.name).to(be_equal_to("cargo-llvm-cov"));
            expect!(res.version).to(be_equal_to("0.1.3"));
        }

        it "should return an error when a package is not found" {
            match get_pkg_version("does-not-exist") {
                Ok(_res) => panic!("Function should of not succeeded"),
                Err(err) =>  {
                    expect!(err.to_string()).to(be_equal_to("Package for binary does-not-exist not found"));
                }
            }
        }
    }

    describe "run" {
        context "cargo binary"  {
            it "should execute successfully" {
                let mut args: Vec<String> = vec!["cargo".to_owned(), "bin".to_owned(), "cargo-llvm-cov".to_owned(), "--help".to_owned()];
                run(&mut args).unwrap();
            }
        }

        context "regular binary"  {
            it "should execute successfully" {
                let mut args: Vec<String> = vec!["cargo".to_owned(), "bin".to_owned(), "petname".to_owned(), "--help".to_owned()];
                run(&mut args).unwrap();
            }
        }
    }
}
