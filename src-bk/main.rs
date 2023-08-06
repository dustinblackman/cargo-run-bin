#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

#[cfg(test)]
#[path = "main_test.rs"]
mod main_test;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;
use std::process;

use anyhow::anyhow;
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use cargo_toml::Dependency;
use cargo_toml::Manifest;
use owo_colors::OwoColorize;
use serde::Deserialize;
use version_check as rustc;

#[derive(Debug)]
struct PkgVersion {
    name: String,
    version: String,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    bin: HashMap<String, String>,
}

fn get_metadata_binaries(toml_manifest: cargo_toml::Manifest) -> Result<Option<Metadata>> {
    let metadata_binaries_str = toml_manifest
        .package
        .as_ref()
        .and_then(|pkg| {
            return pkg.metadata.as_ref().map(|metadata_val| {
                return metadata_val.to_string();
            });
        })
        .ok_or_else(|| return "".to_string())
        .unwrap_or_else(|_| return "".to_string());

    if !metadata_binaries_str.is_empty() {
        let metadata_res: Result<Metadata, toml::de::Error> =
            toml::from_str(&metadata_binaries_str);
        if let Ok(metadata) = metadata_res {
            return Ok(Some(metadata));
        }
    }

    return Ok(None);
}

fn get_binaries() -> Result<Vec<String>> {
    let home_dir = home::cargo_home()?;
    let cache_folder = fs::read_dir(home_dir.join("registry/src"))?
        .next()
        .unwrap()
        .unwrap()
        .path();

    let mut binaries: Vec<String> = vec![];

    let toml_manifest = Manifest::from_path("./Cargo.toml")?;
    let mut deps = toml_manifest.dependencies.to_owned();
    deps.append(&mut toml_manifest.dev_dependencies.to_owned());
    for (dep_name, dep_details) in deps.iter() {
        let version = match dep_details {
            Dependency::Detailed(e) => e.version.to_owned().unwrap(),
            Dependency::Simple(e) => e.to_owned(),
        };

        let crate_folder =
            path::Path::new(cache_folder.to_str().unwrap()).join(format!("{dep_name}-{version}"));

        let dep_manifest =
            Manifest::from_path(crate_folder.clone().join("Cargo.toml").to_str().unwrap())?;

        if !dep_manifest.bin.is_empty() {
            for bin in dep_manifest.bin {
                binaries.push(bin.name.unwrap());
            }
        } else if crate_folder.clone().join("src/main.rs").exists() {
            binaries.push(dep_name.to_owned());
        }
    }

    let metadata_bin = get_metadata_binaries(toml_manifest)?;
    if let Some(metadata) = metadata_bin {
        for (key, _version) in metadata.bin {
            binaries.push(key.to_owned());
        }
    }

    return Ok(binaries);
}

fn get_pkg_version(bin_name: &str) -> Result<PkgVersion> {
    let toml_manifest = Manifest::from_path("./Cargo.toml")?;

    let mut deps = toml_manifest.dependencies.to_owned();
    deps.append(&mut toml_manifest.dev_dependencies.to_owned());

    let metadata_bin = get_metadata_binaries(toml_manifest)?;
    if let Some(metadata) = metadata_bin {
        if metadata.bin.contains_key(bin_name) {
            return Ok(PkgVersion {
                name: bin_name.to_string(),
                version: metadata.bin.get(bin_name).unwrap().to_string(),
            });
        }
    }

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
        .manifest_path("./Cargo.toml") // TODO Delete this later, and find a way to autodiscover.
        .exec()?;

    let pkg = metadata
        .packages
        .iter()
        .find(|e| {
            return e.targets.iter().any(|t| {
                return t.name == bin_name;
            });
        })
        .ok_or_else(|| return anyhow!(format!("Package for binary {bin_name} not found")))?;

    return Ok(PkgVersion {
        name: pkg.name.to_owned(),
        version: pkg.version.to_string(),
    });
}

fn run_binary(args: &mut Vec<String>) -> Result<()> {
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

    let cache_path = format!("./.bin/rust-{rust_version}/{name}/{version}", name=pkg_version.name, version=pkg_version.version);
    let mut cache_bin_path = format!("{cache_path}/bin/{bin_name}");
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
        env_path = format!("{cache_path}/bin:{env_path}");

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

    return Err(anyhow!(format!("Process {bin_name} failed to start")));
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args[2] == "--list" {
        let res = get_binaries();
        if let Err(res) = res {
            println!("{}", format!("run-bin failed: {res}").red());
            process::exit(1);
        }

        print!("{}", res.unwrap().join("\n"));
        process::exit(0);
    }

    let res = run_binary(&mut args);
    if let Err(res) = res {
        println!("{}", format!("run-bin failed: {res}").red());
        process::exit(1);
    }
}
