//! Installing tooling globally when working in teams or on CI is a silly
//! problem to manage. `cargo-run-bin` builds, caches, and executes binaries
//! from their locked down versions in `Cargo.toml`, and allows your teams to
//! always be running the same tooling versions.
//!
//! For command lines that extend cargo such as `cargo-nextest`, run-bin will
//! create and manage cargo aliases to allow using cargo extensions without any
//! changes to your command line scripts! `cargo-run-bin` gets out of your way,
//! and you'll forget you're even using it!
//!
//! ## Usage
//!
//! For command line usage, see the [GitHub repo](https://github.com/dustinblackman/cargo-run-bin).
//!
//! `run-bin` can also be used as a library and paired nicely with your
//! `build.rs` or any other scripts. The following example demos having `dprint`
//! configured within `[package.metadata.bin]`, and executing `dprint --help`.
//!
//! ```toml
//! [package.metadata.bin]
//! dprint = { version = "0.40.2" }
//! ```
//!
//! ```rust
//! use anyhow::Result;
//! use cargo_run_bin::{binary, metadata};
//!
//! fn main() -> Result<()> {
//!     let binary_package = metadata::get_binary_packages()?
//!         .iter()
//!         .find(|e| e.package == "dprint")
//!         .unwrap()
//!         .to_owned();
//!     let bin_path = binary::install(binary_package)?;
//!     binary::run(bin_path, vec!["--help".to_string()])?;
//!
//!     return Ok(());
//! }
//! ```
//!
//! Using `binary::run` is optional. You can recreate it and make changes to
//! your liking using `std::process`, with shims included!
//!
//! ```rust
//! use std::process;
//!
//! use anyhow::Result;
//! use cargo_run_bin::{binary, metadata, shims};
//!
//! fn main() -> Result<()> {
//!     let binary_package = metadata::get_binary_packages()?
//!         .iter()
//!         .find(|e| e.package == "dprint")
//!         .unwrap()
//!         .to_owned();
//!     let bin_path = binary::install(binary_package)?;
//!
//!     let mut shell_paths = shims::get_shim_paths()?;
//!     shell_paths.push(env::var("PATH").unwrap_or("".to_string()));
//!
//!     process::Command::new(bin_path)
//!         .args(["--help"])
//!         .env("PATH", shell_paths.join(":"))
//!         .spawn();
//!
//!     return Ok(());
//! }
//! ```

#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

pub mod binary;
pub mod cargo_config;
#[cfg(not(doc))]
#[cfg(feature = "cli")]
pub mod cli;
pub mod metadata;
pub mod shims;
