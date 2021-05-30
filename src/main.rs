#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

#[cfg(test)]
#[path = "main_test.rs"]
mod main_test;

use anyhow::{anyhow, Result};
use cargo_metadata::MetadataCommand;
use cargo_toml::{Dependency, Manifest};
use colored::*;
use fstrings::*;
use std::env;
use std::fs;
use std::path;
use std::process;
use version_check as rustc;

#[derive(Debug)]
struct PkgVersion {
    name: String,
    version: String,
}

fn get_pkg_version(bin_name: &str) -> Result<PkgVersion> {
    let toml = Manifest::from_path("./Cargo.toml")?;
    let mut deps = toml.dependencies.to_owned();
    deps.append(&mut toml.dev_dependencies.to_owned());
    for (key, value) in deps.iter() {
        if key != bin_name {
            continue;
        }

        let version = match value {
            Dependency::Detailed(e) => e.version.to_owned().unwrap(),
            Dependency::Simple(e) => e.to_owned(),
        };

        return Ok(PkgVersion {
            name: key.to_owned(),
            version,
        });
    }

    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml") // TODO Delete this my later, and find a way to autodiscover.
        .exec()
        .unwrap();

    let pkg = metadata.packages.iter().find(|e| {
        return e.targets.iter().any(|t| {
            return t.name == bin_name;
        });
    });

    if let Some(pkg) = pkg {
        return Ok(PkgVersion {
            name: pkg.name.to_owned(),
            version: pkg.version.to_string(),
        });
    }

    return Err(anyhow!(f!("Package for binary {bin_name} not found")));
}

fn run() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    let mut rust_version = "unknown".to_string();
    if let Some(res) = rustc::triple() {
        if res.1.is_nightly() {
            rust_version = "nightly".to_string();
        } else {
            rust_version = res.0.to_string();
        }
    }

    let bin_name = args[2].to_owned();
    let pkg_version = get_pkg_version(&bin_name)?;

    let cache_path = f!("./.bin/rust-{rust_version}/{pkg_version.name}/{pkg_version.version}");
    let cache_bin_path = f!("{cache_path}/bin/{bin_name}");

    if !path::Path::new(&cache_bin_path).exists() {
        fs::create_dir_all(&cache_path)?;
        process::Command::new("cargo")
            .arg("install")
            .arg("--root")
            .arg(cache_path)
            .arg("--target-dir")
            .arg("./target") // TODO fix target dir alongside cargo.toml later
            .arg("--version")
            .arg(pkg_version.version)
            .arg(pkg_version.name)
            .output()?;
    }

    args.drain(0..3);
    let spawn = process::Command::new(cache_bin_path)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .args(&args)
        .spawn();

    if let Ok(mut spawn) = spawn {
        let status = spawn.wait()?;
        process::exit(status.code().unwrap());
    }

    return Err(anyhow!(f!("Process {bin_name} failed to start")));
}

fn main() {
    let res = run();
    if let Err(res) = res {
        println!("{}", f!("run-bin failed: {res}").red());
        process::exit(1);
    }
}
