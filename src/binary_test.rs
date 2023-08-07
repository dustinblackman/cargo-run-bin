use expectest::prelude::*;

use super::*;

mod build {
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

        let cache_bin_path = build(nextest.clone()).unwrap();
        expect!(cache_bin_path.ends_with("/bin/cargo-nextest")).to(be_equal_to(true));
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
        let cache_bin_path = build(nextest.clone()).unwrap();

        let res = run(cache_bin_path, vec!["--help".to_string()]);
        expect(res.is_ok()).to(be_equal_to(true));
    }
}
