use std::fs;

use expectest::prelude::*;

use super::*;

mod sync_aliases {
    use super::*;

    #[test]
    fn it_removes_and_adds_aliases() {
        let res = sync_aliases();
        let toml_str: String = fs::read_to_string(".cargo/config.toml")
            .unwrap()
            .parse()
            .unwrap();

        expect!(res.is_ok()).to(be_equal_to(true));
        expect!(toml_str.contains("nextest = [\"bin\", \"cargo-nextest\"]")).to(be_equal_to(true));
    }
}

mod binstall_alias_exists {
    use super::*;

    // Lazy happy path test.
    #[test]
    fn it_should_return_true() {
        let res = binstall_alias_exists().unwrap();
        expect!(res).to(be_equal_to(true));
    }
}
