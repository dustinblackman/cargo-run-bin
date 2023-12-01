use std::env;
use std::fs;
use std::io::Write;
use std::path;

use anyhow::Result;
use cfg_if::cfg_if;

use crate::metadata;

#[cfg(test)]
#[path = "shims_test.rs"]
mod shims_test;

#[cfg(target_family = "unix")]
fn create_shim(binary: &str, bin_path: path::PathBuf) -> Result<()> {
    use std::os::unix::prelude::OpenOptionsExt;

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

    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o770)
        .open(bin_path)?;

    write!(f, "{}", script)?;

    return Ok(());
}

#[cfg(not(target_family = "unix"))]
fn create_shim(binary: &str, bin_path: path::PathBuf) -> Result<()> {
    let script = format!(
        r#"@echo off
cargo bin {binary} %*
"#
    );

    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(bin_path)?;

    write!(f, "{}", script)?;

    return Ok(());
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

        let mut bin_path = bin_dir.join(&bin);
        bin_path.set_extension("");
        cfg_if! {
            if #[cfg(not(target_family = "unix"))] {
                bin_path.set_extension("cmd");
            }
        }
        if bin_path.exists() {
            continue;
        }

        create_shim(&bin, bin_path)?;
    }

    return Ok(());
}
