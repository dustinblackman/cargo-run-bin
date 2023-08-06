use std::collections::HashMap;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use cargo_toml::Manifest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MetadataValue {
    version: String,
    bins: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    bin: HashMap<String, MetadataValue>,
}

#[derive(Clone)]
pub struct BinaryPackage {
    pub bin_target: Option<String>,
    pub package: String,
    pub version: String,
}

fn get_project_root() -> Result<PathBuf> {
    let path = env::current_dir()?;
    let path_ancestors = path.as_path().ancestors();

    for p in path_ancestors {
        let has_cargo = read_dir(p)?.any(|p| return p.unwrap().file_name() == *"Cargo.lock");

        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }

    return Err(anyhow!("Root directory for rust project not found."));
}

fn load_cargo_manifest() -> Result<Manifest> {
    let project_root = get_project_root()?;
    let toml_manifest = Manifest::from_path(project_root.join("Cargo.toml"))?;
    return Ok(toml_manifest);
}

fn get_metadata_binaries(toml_manifest: Manifest) -> Result<Metadata> {
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

    if metadata_binaries_str.is_empty() {
        let metadata_res: Result<Metadata, toml::de::Error> =
            toml::from_str(&metadata_binaries_str);

        if let Ok(metadata) = metadata_res {
            return Ok(metadata);
        }
    }

    bail!("No binaries configured in Cargo.toml");
}

pub fn get_binary_packages() -> Result<Vec<BinaryPackage>> {
    let manifest = load_cargo_manifest()?;
    let metadata = get_metadata_binaries(manifest)?;

    let mut binary_details: Vec<BinaryPackage> = Vec::new();

    for (pkg_name, pkg_details) in metadata.bin.into_iter() {
        if let Some(pkg_bins) = pkg_details.bins {
            for bin_target in pkg_bins.iter() {
                binary_details.push(BinaryPackage {
                    bin_target: Some(bin_target.to_string()),
                    package: pkg_name.clone(),
                    version: pkg_details.version.clone(),
                });
            }
        } else {
            binary_details.push(BinaryPackage {
                bin_target: None,
                package: pkg_name,
                version: pkg_details.version,
            });
        }
    }

    return Ok(binary_details);
}
