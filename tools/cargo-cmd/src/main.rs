use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use toml::Value;

fn find_cargo_toml() -> Result<PathBuf, String> {
    let mut current_dir =
        env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Ok(cargo_toml);
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => {
                return Err("Could not find Cargo.toml in current directory or parents".to_string());
            }
        }
    }
}

fn parse_commands(cargo_toml_path: &PathBuf) -> Result<HashMap<String, String>, String> {
    let content = std::fs::read_to_string(cargo_toml_path)
        .map_err(|e| format!("Failed to read {}: {}", cargo_toml_path.display(), e))?;

    let toml: Value = content
        .parse()
        .map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

    let commands = toml
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("commands"))
        .and_then(|c| c.as_table())
        .map(|table| {
            table
                .iter()
                .filter_map(|(key, value)| {
                    value
                        .as_str()
                        .map(|cmd| (key.clone(), cmd.trim().to_string()))
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(commands)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo cmd <command> [additional args...]");
        std::process::exit(1);
    }

    let command_name = &args[1];
    let additional_args = &args[2..];

    let cargo_toml = match find_cargo_toml() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let commands = match parse_commands(&cargo_toml) {
        Ok(cmds) => cmds,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let command_script = match commands.get(command_name) {
        Some(cmd) => cmd,
        None => {
            eprintln!("Error: Command '{}' not found in Cargo.toml", command_name);
            std::process::exit(1);
        }
    };

    // Build the full command: the script from TOML + any additional arguments
    let full_command = if additional_args.is_empty() {
        command_script.clone()
    } else {
        format!("{} {}", command_script.trim(), additional_args.join(" "))
    };

    // Execute the command using bash
    let mut cmd = Command::new("bash");
    cmd.arg("-c")
        .arg(&full_command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Failed to execute command: {}", e);
            std::process::exit(1);
        }
    };

    std::process::exit(status.code().unwrap_or(1));
}
