use super::*;

mod sync_shell_aliases {
    use super::super::metadata;
    use super::*;

    #[test]
    fn it_creates_shell_aliases() {
        let res = sync_aliases();
        let exists = metadata::get_project_root()
            .unwrap()
            .join(".bin/.bin/hello-world-first")
            .exists();

        assert!(res.is_ok());
        assert!(exists);
    }
}
