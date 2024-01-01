use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use serde::Deserialize;
use toml_edit::Document;
use toml_edit::Item;

#[derive(Deserialize, Debug, PartialEq)]
struct MetadataValue {
    version: String,
    git: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
    rev: Option<String>,
    path: Option<String>,
    locked: Option<bool>,
    bins: Option<Vec<String>>,
    #[serde(alias = "default-features")]
    default_features: Option<bool>,
    features: Option<Vec<String>>,
}

type MetadataBins = HashMap<String, MetadataValue>;

#[derive(Clone)]
pub struct BinaryPackage {
    pub bin_target: Option<String>,
    pub package: String,
    pub locked: Option<bool>,
    pub version: String,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    pub path: Option<String>,
    pub default_features: Option<bool>,
    pub features: Option<Vec<String>>,
}

pub fn get_project_root() -> Result<PathBuf> {
    let path = env::current_dir()?;
    let path_ancestors = path.as_path().ancestors();

    for p in path_ancestors {
        let has_cargo = fs::read_dir(p)?.any(|p| return p.unwrap().file_name() == *"Cargo.lock");

        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }

    return Err(anyhow!("Root directory for rust project not found."));
}

fn toml_has_path(doc: &Item, keys: Vec<&str>) -> bool {
    let mut item = doc;
    for key in keys {
        if item.get(key).is_none() {
            return false;
        }
        item = &item[key];
    }

    return true;
}

fn get_metadata_binaries() -> Result<MetadataBins> {
    let toml_str: String = fs::read_to_string(get_project_root()?.join("Cargo.toml"))?.parse()?;
    let doc = toml_str.parse::<Document>()?;

    let mut metadata_str = "".to_string();
    if toml_has_path(doc.as_item(), vec!["package", "metadata", "bin"]) {
        metadata_str = doc["package"]["metadata"]["bin"].to_string();
    } else if toml_has_path(doc.as_item(), vec!["workspace", "metadata", "bin"]) {
        metadata_str = doc["workspace"]["metadata"]["bin"].to_string();
    }

    if metadata_str.is_empty() {
        bail!("No binaries configured in Cargo.toml");
    }

    let metadata_res: Result<MetadataBins, toml::de::Error> = toml::from_str(&metadata_str);
    return Ok(metadata_res?);
}

pub fn get_binary_packages() -> Result<Vec<BinaryPackage>> {
    let metadata = get_metadata_binaries()?;

    let mut binary_details: Vec<BinaryPackage> = Vec::new();

    for (pkg_name, pkg_details) in metadata.into_iter() {
        if let Some(pkg_bins) = pkg_details.bins {
            for bin_target in pkg_bins.iter() {
                binary_details.push(BinaryPackage {
                    bin_target: Some(bin_target.to_string()),
                    package: pkg_name.clone(),
                    locked: pkg_details.locked,
                    version: pkg_details.version.clone(),
                    git: pkg_details.git.clone(),
                    branch: pkg_details.branch.clone(),
                    tag: pkg_details.tag.clone(),
                    rev: pkg_details.rev.clone(),
                    path: pkg_details.path.clone(),
                    default_features: pkg_details.default_features,
                    features: pkg_details.features.clone(),
                });
            }
        } else {
            binary_details.push(BinaryPackage {
                bin_target: None,
                package: pkg_name,
                locked: pkg_details.locked,
                version: pkg_details.version,
                git: pkg_details.git,
                branch: pkg_details.branch,
                tag: pkg_details.tag,
                rev: pkg_details.rev,
                path: pkg_details.path,
                default_features: pkg_details.default_features,
                features: pkg_details.features,
            });
        }
    }

    binary_details.sort_by_key(|e| {
        if e.bin_target.is_some() {
            return e.bin_target.as_ref().unwrap().to_string();
        }
        return e.package.to_string();
    });

    return Ok(binary_details);
}
