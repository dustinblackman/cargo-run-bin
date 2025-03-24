<h1 align=center>cargo-run-bin</h1>

![cargo-run-bin](.github/banner.png)

[![Build status](https://github.com/dustinblackman/cargo-run-bin/workflows/ci/badge.svg)](https://github.com/dustinblackman/cargo-run-bin/actions)
[![Coverage Status](https://coveralls.io/repos/github/dustinblackman/cargo-run-bin/badge.svg?branch=master)](https://coveralls.io/github/dustinblackman/cargo-run-bin?branch=master)
[![Crates.io](https://img.shields.io/crates/v/cargo-run-bin.svg)](https://crates.io/crates/cargo-run-bin)

> Build, cache, and run CLI tools scoped in `Cargo.toml` rather than installing globally. Stop the version drifts across your team, keep it all in sync within your project!

- [Overview](#overview)
- [Install](#install)
- [Usage](#usage)
  - [cargo bin CRATE](#cargo-bin-crate)
  - [cargo bin --sync-aliases](#cargo-bin---sync-aliases)
  - [cargo bin --install](#cargo-bin---install)
  - [Library](#library)
- [License](#license)

## Overview

Installing tooling globally when working in teams or on CI is a silly problem to manage. `cargo-run-bin` builds, caches, and executes binaries from their locked down versions in `Cargo.toml`. This acts similarly to [`npm run`](https://docs.npmjs.com/cli/v7/commands/npm-run-script) and [`gomodrun`](https://github.com/dustinblackman/gomodrun), and allows your teams to always be running the same tooling versions.

For command lines that extend cargo such as `cargo-nextest`, run-bin will create and manage cargo aliases to allow using cargo extensions without any changes to your command line scripts! `cargo-run-bin` gets out of your way, and you'll forget you're even using it!

## Install

Minimum Rust Version: 1.70.0

There are a few different ways you can install `cargo-run-bin`. I'd recommend giving each a read, and picking the best that suits you!

### Global Install

Run the following to install `cargo-run-bin`, and ignore the cache directory in your project.

```sh
cargo install cargo-run-bin
cd my/rust/project
echo ".bin/" >> .gitignore
```

### Using wrapper

Installing with a minimal wrapper and an alias in your project. This gives you the best onboarding experiance in to your project for you and your team as it does not require any global installs! Git pull, and you're ready to consume all your CLI tools!

These instructions assume you'd like to place the wrapper in `tools/cargo-bin` within your project. Change it to fit your needs!

```sh
cd my/rust/project
echo ".bin/" >> .gitignore
cargo new --vcs none --bin tools/cargo-bin
curl --output tools/cargo-bin/src/main.rs https://raw.githubusercontent.com/dustinblackman/cargo-run-bin/refs/tags/v1.7.4/src/main.rs
cd tools/cargo-bin
cargo add --features cli cargo-run-bin
```

Ensure the tool binary is added to the workspace within `Cargo.toml`.

```toml
[workspace]
members = ["tools/cargo-bin"]
```

Now add an alias in `.cargo/config.toml`. If the file doesn't exist yet, then this will be the first entry!

```toml
[alias]
bin = ["run", "--package", "cargo-bin", "--"]
```

Done! Now it can be used as if installed globally.

```sh
cargo bin --version
```

### As a library

You can also use it as a library within your existing logic.

```toml
[dependencies]
cargo-run-bin = { version = "1.7.4", default-features = false }
```

### Distro packages

<details>
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/cargo-run-bin.svg)](https://repology.org/project/cargo-run-bin/versions)

</details>

If your distribution has packaged `cargo-run-bin`, you can use that package for the installation.

### Arch Linux

You can use [pacman](https://wiki.archlinux.org/title/Pacman) to install from the [extra repository](https://archlinux.org/packages/extra/x86_64/cargo-run-bin/):

```sh
pacman -S cargo-run-bin
```

### Alpine Linux

`cargo-run-bin` is available for [Alpine Edge](https://pkgs.alpinelinux.org/packages?name=cargo-run-bin&branch=edge). It can be installed via [apk](https://wiki.alpinelinux.org/wiki/Alpine_Package_Keeper) after enabling the [testing repository](https://wiki.alpinelinux.org/wiki/Repositories).

```sh
apk add cargo-run-bin
```

## Usage

`cargo-run-bin` keeps track of the binaries and their versions from within `Cargo.toml` under the `[package.metadata.bin]`.
table. A quick example taken from this repo:

```toml
[package.metadata.bin]
cargo-binstall = { version = "1.1.2" }
cargo-nextest = { version = "0.9.57", locked = true }
dprint = { version = "0.30.3" }
cargo-mobile2 = { version = "0.5.2", bins = ["cargo-android", "cargo-mobile"], locked = true }
```

Or if you're setting up in a workspace:

```toml
[workspace.metadata.bin]
cargo-binstall = { version = "1.1.2" }
cargo-nextest = { version = "0.9.57", locked = true }
```

| Parameter        | Type          | Required | Description                                                                                                                                                                                                                                                               |
| ---------------- | ------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version          | `String`      | true     | Specifies the version of the crate. It is not treated as a caret requirement automatically. Corresponds to the `cargo install` [`--version`](https://doc.rust-lang.org/cargo/commands/cargo-install.html#install-options) argument.                                       |
| bins             | `Vec<String>` | false    | An array of binaries that the crate contains that you wish to build. These can be found in a crates `Cargo.toml` file. See [`cargo-mobile2`](https://github.com/tauri-apps/cargo-mobile2/blob/a5f3783870f48886e3266e43f92a6768fb1eb3d4/Cargo.toml#L18-L28) as an example. |
| locked           | `Boolean`     | false    | A parameter when set to `true` runs `cargo install` with the [`--locked`](https://doc.rust-lang.org/cargo/commands/cargo-install.html#dealing-with-the-lockfile) parameter.                                                                                               |
| features         | `Vec<String>` | false    | An array of crate features to enable.                                                                                                                                                                                                                                     |
| default-features | `Boolean`     | false    | When set to `false`, disables all default features.                                                                                                                                                                                                                       |
| git              | String        | false    | A git URL to install from rather than from crates.io. This will also be used by Binstall to look up Cargo manifist if Binstall is available.                                                                                                                              |
| branch           | String        | false    | A git branch to install from when `git` is set. This takes priority over `tag` and `rev`                                                                                                                                                                                  |
| tag              | String        | false    | A git tag to install from when `git` is set. `branch` will take priority if set, and takes priority over `rev`.                                                                                                                                                           |
| rev              | String        | false    | A git revision to install from when `git` is set. `branch` and `tag` will take priority if set.                                                                                                                                                                           |
| path             | String        | false    | The path to a local crate to install.                                                                                                                                                                                                                                     |

If you're a fan of prebuilt binaries and fast downloads, run-bin will use [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) if it's installed globally, or configured within `[package.metadata.bin]`, rather than building tools from source.

### `cargo bin CRATE`

Taking an example of `dprint`, running `cargo bin dprint --help` with install/build and cache the dprint binary with the
specified version in `Cargo.toml`. All future executions will run instantly without an install step, and dprint can be used
as you wish!

### `cargo bin --sync-aliases`

With the power of [cargo aliases](https://doc.rust-lang.org/cargo/reference/config.html#alias), `cargo bin --sync-aliases`
will create aliases for any `cargo-*` crate, allowing you to execute commands such `cargo nextest run` that will use
`cargo bin` under the hood. Check out some of the example from [this repo](.cargo/config.toml).

### `cargo bin --install`

When pulling down a new repo, or adding a step to CI, `cargo bin --install` will install or build all binaries that have not been
cached which are configured in `Cargo.toml`.

### Library

`run-bin` can also be used as a library and paired nicely with your `build.rs` or any other scripts. The following
example demos having `dprint` configured within `[package.metadata.bin]`, and executing `dprint --help`.

```toml
[package.metadata.bin]
dprint = { version = "0.40.2" }
```

```rust
use anyhow::Result;
use cargo_run_bin::{binary, metadata};

fn main() -> Result<()> {
    let binary_package = metadata::get_binary_packages()?
        .iter()
        .find(|e| e.package == "dprint")
        .unwrap()
        .to_owned();
    let bin_path = binary::install(binary_package)?;
    binary::run(bin_path, vec!["--help".to_string()])?;

    return Ok(());
}
```

Using `binary::run` is optional. You can recreate it and make changes to your liking using `std::process`, with shims included!

```rust
use std::process;

use anyhow::Result;
use cargo_run_bin::{binary, metadata, shims};

fn main() -> Result<()> {
    let binary_package = metadata::get_binary_packages()?
        .iter()
        .find(|e| e.package == "dprint")
        .unwrap()
        .to_owned();
    let bin_path = binary::install(binary_package)?;

    let mut shell_paths = shims::get_shim_paths()?;
    shell_paths.push(env::var("PATH").unwrap_or("".to_string()));

    process::Command::new(bin_path)
        .args(["--help"])
        .env("PATH", shell_paths.join(":"))
        .spawn();

    return Ok(());
}
```

## [License](./LICENSE)

MIT.
