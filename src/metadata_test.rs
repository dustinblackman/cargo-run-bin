use expectest::prelude::*;

use super::*;

mod get_binary_packages {
    use super::*;

    #[test]
    fn it_returns_locked_packages() {
        let binary_packages = get_binary_packages().unwrap();
        let nextest = binary_packages.iter().find(|&e| {
            return e.package == "cargo-nextest";
        });

        expect!(nextest.is_some()).to(be_equal_to(true));
        let res = nextest.unwrap();
        expect!(&res.package).to(be_equal_to("cargo-nextest"));
        expect!(&res.version).to_not(be_equal_to(""));
        expect!(res.locked.unwrap()).to(be_equal_to(true));
    }

    #[test]
    fn it_returns_bin_target_packages() {
        let binary_packages = get_binary_packages().unwrap();
        let android = binary_packages.iter().find(|&e| {
            return e.bin_target.is_some() && e.bin_target.clone().unwrap() == "cargo-android";
        });

        expect!(android.is_some()).to(be_equal_to(true));
        let res = android.unwrap();
        expect!(&res.package).to(be_equal_to("tauri-mobile"));
        expect!(&res.version).to_not(be_equal_to(""));
        expect!(res.bin_target.clone().unwrap()).to(be_equal_to("cargo-android"));
    }
}
