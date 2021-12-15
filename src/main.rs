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

fn get_binaries() -> Result<Vec<String>> {
    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml") // TODO Delete this later, and find a way to autodiscover.
        .exec()?;

    let binaries = metadata
        .packages
        .iter()
        .map(|e| {
            return e
                .targets
                .iter()
                .filter(|target| return target.kind.contains(&"bin".to_owned()))
                .map(|target| return target.name.to_owned());
        })
        .flatten()
        .collect::<Vec<String>>();

    return Ok(binaries);
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
        .exec()?;

    let pkg = metadata
        .packages
        .iter()
        .find(|e| {
            return e.targets.iter().any(|t| {
                return t.name == bin_name;
            });
        })
        .ok_or_else(|| return anyhow!(f!("Package for binary {bin_name} not found")))?;

    return Ok(PkgVersion {
        name: pkg.name.to_owned(),
        version: pkg.version.to_string(),
    });
}

fn run(args: &mut Vec<String>) -> Result<()> {
    let mut args = args.to_owned();

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
    let mut cache_bin_path = f!("{cache_path}/bin/{bin_name}");
    let mut env_path = match env::var("PATH") {
        Ok(val) => val,
        Err(_) => "".to_owned(), // TODO throw err;
    };

    if !path::Path::new(&cache_bin_path).exists() {
        fs::create_dir_all(&cache_path)?;
        process::Command::new("cargo")
            .arg("install")
            .arg("--root")
            .arg(&cache_path)
            .arg("--target-dir")
            .arg("./target") // TODO fix target dir alongside cargo.toml later
            .arg("--version")
            .arg(pkg_version.version)
            .arg(pkg_version.name)
            .output()?;
    }

    args.drain(0..3);

    if bin_name.starts_with("cargo-") {
        cache_bin_path = "cargo".to_owned();
        env_path = f!("{cache_path}/bin:{env_path}");

        let mut new_args = vec![bin_name.replace("cargo-", "")];
        new_args.append(&mut args);
        args = new_args;
    }

    let spawn = process::Command::new(cache_bin_path)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .env("PATH", env_path)
        .args(&args)
        .spawn();

    if let Ok(mut spawn) = spawn {
        let status = spawn
            .wait()?
            .code()
            .ok_or_else(|| return anyhow!("Failed to get spawn exit code"))?;
        process::exit(status);
    }

    return Err(anyhow!(f!("Process {bin_name} failed to start")));
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args[2] == "--list-binaries" {
        let res = helpers::get_binaries();
        if let Err(res) = res {
            println!("{}", f!("run-bin failed: {res}").red());
            process::exit(1);
        }

        print!("{}", res.unwrap().join("\n"));
        process::exit(0);
    }

    let res = helpers::run_binary(&mut args);
    if let Err(res) = res {
        println!("{}", f!("run-bin failed: {res}").red());
        process::exit(1);
    }
}
