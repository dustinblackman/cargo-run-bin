use std::fs;

use anyhow::Result;
use cargo_run_bin::metadata;

fn clean_shims() -> Result<()> {
    let bin_dir = metadata::get_project_root()?.join(".bin/.shims");
    if bin_dir.exists() {
        fs::remove_dir_all(&bin_dir)?;
    }

    Ok(())
}

mod sync_shims {
    use std::fs;

    use cargo_run_bin::metadata;
    use cargo_run_bin::shims::sync;

    use super::clean_shims;

    #[test]
    #[cfg(target_family = "unix")]
    fn it_creates_shims() {
        clean_shims().unwrap();
        let res = sync();
        let exists = metadata::get_project_root()
            .unwrap()
            .join(".bin/.shims/hello-world-first")
            .exists();

        clean_shims().unwrap();
        assert!(res.is_ok());
        assert!(exists);
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn it_creates_shims_for_zsh() {
        std::env::set_var("SHELL", "zsh");
        clean_shims().unwrap();
        let res = sync();
        assert!(res.is_ok());

        let shim_path = metadata::get_project_root()
            .unwrap()
            .join(".bin/.shims/hello-world-first");

        let content = fs::read_to_string(shim_path).unwrap();

        clean_shims().unwrap();
        assert!(content.starts_with("#!/usr/bin/env zsh"));
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn it_creates_shims_falling_back_to_sh() {
        std::env::set_var("SHELL", "fish");
        clean_shims().unwrap();
        let res = sync();
        assert!(res.is_ok());

        let shim_path = metadata::get_project_root()
            .unwrap()
            .join(".bin/.shims/hello-world-first");

        let content = fs::read_to_string(shim_path).unwrap();

        clean_shims().unwrap();
        assert!(content.starts_with("#!/usr/bin/env sh"));
    }

    #[test]
    #[cfg(not(target_family = "unix"))]
    fn it_creates_shims() {
        let res = sync();
        let exists = metadata::get_project_root()
            .unwrap()
            .join(".bin/.shims/hello-world-first.cmd")
            .exists();

        assert!(res.is_ok());
        assert!(exists);
    }
}
