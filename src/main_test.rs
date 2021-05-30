use super::*;
use expectest::prelude::*;
use speculate::speculate;

speculate! {
    describe "get_pkg_version" {
        it "should return a name and version number" {
            let res = get_pkg_version("cargo-watch").unwrap();
            expect!(res.name).to(be_equal_to("cargo-watch"));
            expect!(res.version).to(be_equal_to("7.8.0"));
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
}
