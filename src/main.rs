#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use std::env;
use std::fs;
use std::path;
use std::process;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use version_check as rustc;

mod metadata;

fn run_binary(binary_package: metadata::BinaryPackage, args: Vec<&str>) -> Result<()> {
    let mut rust_version = "unknown".to_string();
    if let Some(res) = rustc::triple() {
        if res.1.is_nightly() {
            rust_version = "nightly".to_string();
        } else {
            rust_version = res.0.to_string();
        }
    }

    let mut bin_name = binary_package.package.to_owned();
    if let Some(bin_target) = &binary_package.bin_target {
        bin_name = bin_target.to_string();
    }

    // TODO get project root here, absolute path.
    let cache_path = format!(
        "./.bin/rust-{rust_version}/{package_name}/{version}",
        package_name = binary_package.package,
        version = binary_package.version
    );
    let cache_bin_path = format!("{cache_path}/bin/{bin_name}");
    let env_path = match env::var("PATH") {
        Ok(val) => val,
        Err(_) => "".to_owned(), // TODO throw err;
    };

    if !path::Path::new(&cache_bin_path).exists() {
        fs::create_dir_all(&cache_path)?;
        let mut cmd_prefix = process::Command::new("cargo");

        // TODO Move this to stderr
        cmd_prefix
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .arg("install")
            .arg("--root")
            .arg(&cache_path)
            .arg("--version")
            .arg(binary_package.version);

        if let Some(bin_target) = &binary_package.bin_target {
            cmd_prefix.arg("--bin").arg(bin_target);
        }

        cmd_prefix.arg(binary_package.package).output()?;
    }

    // TODO I don't think this is needed anymore.
    // if bin_name.starts_with("cargo-") {
    //     cache_bin_path = "cargo".to_owned();
    //     env_path = format!("{cache_path}/bin:{env_path}");
    //
    //     let mut new_args = vec![bin_name.replace("cargo-", "")];
    //     new_args.append(&mut args);
    //     args = new_args;
    // }

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

    bail!("Process {bin_name} failed to start");
}

fn run() -> Result<()> {
    let binary_name = "dprint".to_string();
    let args = vec!["fmt"];

    let binary_packages = metadata::get_binary_packages()?;
    let binary_package = binary_packages.iter().find(|&e| {
        return e.package == binary_name
            || (e.bin_target.is_some() && e.bin_target.as_deref().unwrap() == binary_name);
    });
    if binary_package.is_none() {
        bail!("No package found for binary {binary}");
    }

    run_binary(binary_package.unwrap().clone(), args)?;

    return Ok(());
}

fn main() {
    run().unwrap();
}
