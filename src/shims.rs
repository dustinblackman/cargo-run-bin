#[cfg(target_family = "unix")]
use std::env;
use std::fs;
use std::io::Write;
use std::path;

use anyhow::Result;
use cfg_if::cfg_if;

use crate::metadata;

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

/// Creates shims in `.bin/shims` for all non cargo extensions configured in
/// Cargo.toml. This directory is added to PATH For all executes of `cargo bin`.
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

/// Return an array of entries that can be added to PATH to provide shims to
/// other configured run-bin packages. Results be empty if entries already exist
/// in PATH.
pub fn get_shim_paths() -> Result<Vec<String>> {
    let mut shim_paths = vec![];
    let system_shell_paths = env::var("PATH")
        .unwrap_or("".to_string())
        .split(':')
        .map(|e| return e.to_string())
        .collect::<Vec<String>>();

    let project_root = metadata::get_project_root()?;
    let runbin = project_root
        .join(".bin/.shims")
        .to_string_lossy()
        .to_string();

    if !system_shell_paths.contains(&runbin) {
        shim_paths.push(runbin);
    }

    let gha = project_root.join(".gha/.shims");
    if gha.exists() && !system_shell_paths.contains(&gha.to_string_lossy().to_string()) {
        shim_paths.push(gha.to_string_lossy().to_string());
    }

    return Ok(shim_paths);
}
