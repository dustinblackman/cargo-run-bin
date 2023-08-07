use std::fs;
use std::io;
use std::os::fd::AsFd;
use std::path;
use std::process;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use version_check as rustc;

use crate::metadata;

pub fn build(binary_package: metadata::BinaryPackage) -> Result<String> {
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

    let cache_path = metadata::get_project_root()?
        .join(".bin")
        .join(format!("rust-{rust_version}"))
        .join(binary_package.package.clone())
        .join(binary_package.version.clone());
    let cache_bin_path = cache_path.join("bin").join(bin_name);

    if !path::Path::new(&cache_bin_path).exists() {
        fs::create_dir_all(&cache_path)?;
        let stderr = io::stderr().as_fd().try_clone_to_owned()?;
        let mut cmd_prefix = process::Command::new("cargo");

        cmd_prefix
            .stdout::<std::process::Stdio>(stderr.into())
            .stderr(process::Stdio::inherit())
            .arg("install")
            .arg("--root")
            .arg(&cache_path)
            .arg("--version")
            .arg(binary_package.version);

        if let Some(bin_target) = &binary_package.bin_target {
            cmd_prefix.arg("--bin").arg(bin_target);
        }

        if let Some(locked) = &binary_package.locked {
            if *locked {
                cmd_prefix.arg("--locked");
            }
        }

        cmd_prefix.arg(binary_package.package).output()?;
    }

    return Ok(cache_bin_path.to_str().unwrap().to_string());
}

pub fn run(bin_path: String, args:Vec<String>) -> Result<()> {
    // Silly hack to make cargo commands parse arguments correctly.
    let mut final_args = args.clone();
    let bin_name = path::Path::new(&bin_path).file_name().unwrap().to_str().unwrap();
    if bin_name.starts_with("cargo-") {
        final_args = vec![bin_name.to_string().replace("cargo-", "")];
        final_args.append(&mut args.clone());
    }

    let spawn = process::Command::new(bin_path.clone())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .args(&final_args)
        .spawn();

    if let Ok(mut spawn) = spawn {
        let status = spawn
            .wait()?
            .code()
            .ok_or_else(|| return anyhow!("Failed to get spawn exit code"))?;

        process::exit(status);
    }

    bail!(format!("Process failed to start: {bin_path}"));
}
