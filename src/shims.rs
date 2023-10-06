use std::env;
use std::fs;
use std::io::Write;
use std::os::unix::prelude::OpenOptionsExt;

use anyhow::Result;

use crate::metadata;

#[cfg(test)]
#[path = "shims_test.rs"]
mod shims_test;

fn create_shim(binary: &str) -> Result<String> {
    let shell = env::var("SHELL")
        .unwrap_or("bash".to_string())
        .split('/')
        .last()
        .unwrap()
        .to_string();

    let script = format!(
        r#"#!/usr/bin/env {shell}

if [ ! -t 0 ]; then
    cat - | cargo bin {binary} "$@"
else
    cargo bin {binary} "$@"
fi"#
    );

    return Ok(script);
}

pub fn sync() -> Result<()> {
    let bin_dir = metadata::get_project_root()?.join(".bin/.shims");
    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir)?;
    }

    for pkg in metadata::get_binary_packages()? {
        let mut bin = pkg.package;
        if let Some(bin_target) = pkg.bin_target {
            bin = bin_target;
        }

        if bin.starts_with("cargo-") {
            continue;
        }

        let script = create_shim(&bin)?;
        let bin_path = bin_dir.join(&bin);
        if bin_path.exists() {
            continue;
        }
        let mut f = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o770)
            .open(&bin_path)?;

        write!(f, "{}", script)?;
    }

    return Ok(());
}
