use super::*;

mod sync_shims {
    use super::super::metadata;
    use super::*;

    #[test]
    fn it_creates_shims() {
        let res = sync();
        let exists = metadata::get_project_root()
            .unwrap()
            .join(".bin/.shims/hello-world-first")
            .exists();

        assert!(res.is_ok());
        assert!(exists);
    }
}
