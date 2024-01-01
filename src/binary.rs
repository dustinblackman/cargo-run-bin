use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use cfg_if::cfg_if;
use version_check as rustc;
use which::which;

use crate::cargo_config;
use crate::metadata;

/// INTERNAL: Install binary with cargo install.
pub fn cargo_install(
    binary_package: metadata::BinaryPackage,
    cache_path: path::PathBuf,
) -> Result<()> {
    let mut cmd_prefix = process::Command::new("cargo");

    cmd_prefix
        .stdout(io::stderr())
        .stderr(process::Stdio::inherit())
        .arg("install")
        .arg("--root")
        .arg(&cache_path)
        .arg("--version")
        .arg(binary_package.version);

    if let Some(git) = &binary_package.git {
        cmd_prefix.arg("--git").arg(git);
        if let Some(branch) = &binary_package.branch {
            cmd_prefix.arg("--branch").arg(branch);
        } else if let Some(tag) = &binary_package.tag {
            cmd_prefix.arg("--tag").arg(tag);
        } else if let Some(rev) = &binary_package.rev {
            cmd_prefix.arg("--rev").arg(rev);
        }
    } else if let Some(path) = &binary_package.path {
        cmd_prefix.arg("--path").arg(path);
    }

    if let Some(bin_target) = &binary_package.bin_target {
        cmd_prefix.arg("--bin").arg(bin_target);
    }

    if let Some(locked) = &binary_package.locked {
        if *locked {
            cmd_prefix.arg("--locked");
        }
    }

    if let Some(features) = &binary_package.features {
        cmd_prefix.arg("--features");
        cmd_prefix.arg(features.join(","));
    }

    if let Some(default_features) = &binary_package.default_features {
        if !*default_features {
            cmd_prefix.arg("--no-default-features");
        }
    }

    cmd_prefix.arg(binary_package.package).output()?;

    return Ok(());
}

/// INTERNAL: Install binary with binstall
pub fn binstall(binary_package: metadata::BinaryPackage, cache_path: path::PathBuf) -> Result<()> {
    let mut cmd_prefix = process::Command::new("cargo");

    cmd_prefix
        .stdout(io::stderr())
        .stderr(process::Stdio::inherit())
        .arg("binstall")
        .arg("--no-confirm")
        .arg("--no-symlinks")
        .arg("--root")
        .arg(&cache_path)
        .arg("--install-path")
        .arg(cache_path.join("bin"));

    if let Some(git) = &binary_package.git {
        cmd_prefix.arg("--git").arg(git);
    }

    if let Some(locked) = &binary_package.locked {
        if *locked {
            cmd_prefix.arg("--locked");
        }
    }

    cmd_prefix
        .arg(format!(
            "{package}@{version}",
            package = binary_package.package,
            version = binary_package.version,
        ))
        .output()?;

    return Ok(());
}

/// Install the provided binary package if it has not been built already.
pub fn install(binary_package: metadata::BinaryPackage) -> Result<String> {
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

    let mut cache_bin_path = cache_path.join("bin").join(bin_name);
    cache_bin_path = cache_bin_path.clone();

    cfg_if! {
        if #[cfg(not(target_family = "unix"))] {
            cache_bin_path.set_extension("exe");
        }
    }

    if !path::Path::new(&cache_bin_path).exists() {
        fs::create_dir_all(&cache_path)?;
        if binary_package.bin_target.is_none()
            && binary_package.features.is_none()
            && binary_package.default_features.is_none()
            && binary_package.branch.is_none()
            && binary_package.tag.is_none()
            && binary_package.rev.is_none()
            && binary_package.package != "cargo-binstall"
            && (cargo_config::binstall_alias_exists()? || which("cargo-binstall").is_ok())
        {
            binstall(binary_package, cache_path)?;
        } else {
            cargo_install(binary_package, cache_path)?;
        }
    }

    return Ok(cache_bin_path.to_str().unwrap().to_string());
}

/// Executes provided binary and arguments, adding shims to PATH so any
/// other run-bin configured binaries are available.
pub fn run(bin_path: String, args: Vec<String>) -> Result<()> {
    // Silly hack to make cargo commands parse arguments correctly.
    let mut final_args = args.clone();
    let bin_name = path::Path::new(&bin_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    if bin_name.starts_with("cargo-") {
        final_args = vec![bin_name
            .to_string()
            .replace("cargo-", "")
            .replace(".exe", "")];
        final_args.append(&mut args.clone());
    }

    let mut system_shell_paths = env::var("PATH")
        .unwrap_or("".to_string())
        .split(':')
        .map(|e| return e.to_string())
        .collect::<Vec<String>>();

    let project_root = metadata::get_project_root()?;
    let mut shell_paths = vec![];

    let runbin = project_root
        .join(".bin/.shims")
        .to_string_lossy()
        .to_string();
    if !system_shell_paths.contains(&runbin) {
        shell_paths.push(runbin);
    }

    // https://github.com/dustinblackman/cargo-gha
    let gha = project_root.join(".gha/.shims");
    if gha.exists() && !system_shell_paths.contains(&gha.to_string_lossy().to_string()) {
        shell_paths.push(gha.to_string_lossy().to_string());
    }

    shell_paths.append(&mut system_shell_paths);

    let spawn = process::Command::new(bin_path.clone())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .args(&final_args)
        .env("PATH", shell_paths.join(":"))
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
