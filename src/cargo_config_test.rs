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
