use std::fs;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use toml_edit::table;
use toml_edit::value;
use toml_edit::Array;
use toml_edit::Document;
use toml_edit::Value;

use crate::metadata;

/// Updates alias' in .cargo/config.toml with all configured cargo extensions.
pub fn sync_aliases() -> Result<()> {
    let mut toml_str = "".to_string();
    let config_path = metadata::get_project_root()?.join(".cargo/config.toml");
    if config_path.exists() {
        toml_str = fs::read_to_string(&config_path)?.parse()?;
    }

    let binary_packages = metadata::get_binary_packages()?;
    let new_config = update_aliases_toml(&toml_str, binary_packages)
        .context("failed to update aliases in .cargo/config.toml")?;

    if !config_path.parent().unwrap().exists() {
        fs::create_dir(config_path.parent().unwrap())?;
    }

    fs::write(config_path, new_config)?;

    return Ok(());
}

fn update_aliases_toml(
    toml_str: &str,
    binary_packages: Vec<metadata::BinaryPackage>,
) -> Result<String> {
    let mut doc = toml_str.parse::<Document>()?;
    if doc.get("alias").is_none() {
        doc["alias"] = table();
    }

    // If the TOML structure is not as and we panic because of that, that makes for poor user
    // experience, so try to report errors for everything that could go wrong.
    let aliases = doc["alias"]
        .as_table_mut()
        .context("alias key should be a table")?;
    let mut remove_keys: Vec<String> = vec![];
    for (key, value) in aliases.get_values() {
        let [name] = key.as_slice() else {
            bail!("unexpected nested table: {key:?}")
        };
        // The value can be either a single string (implicitly split on spaces) or an array of
        // strings. We always create an array, but a user might use a single string for other
        // aliases, so we have to at least not crash on such values.
        if let Value::Array(parts) = value {
            let first_part = parts
                .get(0)
                .with_context(|| format!("alias {name:?} is empty array"))?
                .as_str()
                .with_context(|| format!("alias {name:?} should be array of strings"))?;
            if first_part == "bin" {
                remove_keys.push(name.get().to_owned());
            }
        }
    }
    for key in remove_keys {
        aliases.remove(&key);
    }

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
    return Ok(doc.to_string());
}

/// Verifies in cargo-binstall is available in alias'.
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

#[cfg(test)]
mod tests {
    use super::update_aliases_toml;

    #[test]
    fn skips_bare_string_alias() {
        let original_toml = r#"
[alias]
xtask = "run --package xtask --"
"#;
        let new_toml = update_aliases_toml(original_toml, Vec::new()).unwrap();
        assert_eq!(original_toml, new_toml);
    }

    #[test]
    fn doesnt_panic_on_empty_array() {
        let result = update_aliases_toml("alias.mistake = []", Vec::new());
        // Currently an error, could be skipped instead, in that case the Ok(..) result should be
        // tested a bit.
        assert!(result.is_err());
    }

    #[test]
    fn doesnt_panic_on_nested_keys() {
        let result = update_aliases_toml("alias.alias.alias = 'test'", Vec::new());
        // Currently an error, could be skipped instead, in that case the Ok(..) result should be
        // tested a bit.
        assert!(result.is_err());
    }
}
