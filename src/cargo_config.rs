use std::fs;

use anyhow::Result;
use toml_edit::table;
use toml_edit::value;
use toml_edit::Array;
use toml_edit::Document;

use crate::metadata;

pub fn sync_aliases() -> Result<()> {
    let mut toml_str = "".to_string();
    let config_path = metadata::get_project_root()?.join(".cargo/config.toml");
    if config_path.exists() {
        toml_str = fs::read_to_string(&config_path)?.parse()?;
    }

    let mut doc = toml_str.parse::<Document>()?;
    if doc.get("alias").is_none() {
        doc["alias"] = table();
    }

    let aliases = doc["alias"].as_table_mut().unwrap();
    let mut remove_keys: Vec<String> = vec![];
    for (key, value) in aliases.get_values() {
        if value.as_array().unwrap().get(0).unwrap().as_str().unwrap() == "bin" {
            remove_keys.push(key[0].get().to_owned());
        }
    }
    for key in remove_keys {
        aliases.remove(&key);
    }

    let binary_packages = metadata::get_binary_packages()?;
    for binary_package in binary_packages {
        let mut bin = binary_package.package;
        if let Some(bin_target) = binary_package.bin_target {
            bin = bin_target;
        }

        if !bin.starts_with("cargo-") {
            continue;
        }

        let mut arr = Array::new();
        arr.push("bin");
        arr.push(bin.clone());
        doc["alias"][bin.replace("cargo-", "")] = value(arr);
    }

    if !config_path.parent().unwrap().exists() {
        fs::create_dir(config_path.parent().unwrap())?;
    }

    fs::write(config_path, doc.to_string())?;

    return Ok(());
}

pub fn binstall_alias_exists() -> Result<bool> {
    let config_path = metadata::get_project_root()?.join(".cargo/config.toml");
    if !config_path.exists() {
        return Ok(false);
    }

    let toml_str: String = fs::read_to_string(&config_path)?.parse()?;
    let mut doc = toml_str.parse::<Document>()?;
    if doc.get("alias").is_none() {
        return Ok(false);
    }

    let aliases = doc["alias"].as_table_mut().unwrap();
    return Ok(aliases.contains_key("binstall"));
}
