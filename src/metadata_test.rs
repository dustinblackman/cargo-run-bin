use expectest::prelude::*;

use super::*;

mod get_binary_packages {
    use super::*;

    #[test]
    fn return_expected_binary_packages() {
        let binary_packages = get_binary_packages().unwrap();
        let nextest = binary_packages.iter().find(|&e| {
            return e.package == "cargo-nextest";
        });

        expect!(nextest.is_some()).to(be_equal_to(true));
        let res = nextest.unwrap();
        expect!(&res.package).to(be_equal_to("cargo-nextest"));
        expect!(res.locked.unwrap()).to(be_equal_to(true));
    }
}
