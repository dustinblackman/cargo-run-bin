![cargo-run-bin](.github/banner.png)

[![Build status](https://github.com/dustinblackman/cargo-run-bin/workflows/ci/badge.svg)](https://github.com/dustinblackman/cargo-run-bin/actions)
[![Coverage Status](https://coveralls.io/repos/github/dustinblackman/cargo-run-bin/badge.svg?branch=master)](https://coveralls.io/github/dustinblackman/cargo-run-bin?branch=master)
[![Crates.io](https://img.shields.io/crates/v/cargo-run-bin.svg)](https://crates.io/crates/cargo-run-bin)

> Build, cache, and run binaries scoped in `Cargo.toml` rather than installing globally.

- [Overview](#Overview)
- [Install](#Install)
- [Usage](#Usage)
  - [cargo bin CRATE](#cargo-bin-crate)
  - [cargo bin --sync-aliases](#cargo-bin---sync-aliases)
  - [cargo bin --build](#cargo-bin---build)
- [License](#License)

## Overview

Installing tooling globally when working in teams or on CI is a silly problem to manage. `cargo-run-bin` builds, caches, and executes binaries from their locked down versions in `Cargo.toml`. This acts similarly to [`npm run`](https://docs.npmjs.com/cli/v7/commands/npm-run-script) and [`gomodrun`](https://github.com/dustinblackman/gomodrun), and allows your teams to always be running the same tooling versions.

For command lines that extend cargo such as `cargo-nextest`, run-bin will create and manage cargo aliases to allow using cargo extensions without any changes to your command line scripts! `cargo-run-bin` gets out of your way, and you'll forget you're even using it!

## Install

Run the following to install `cargo-run-bin`, and ignore the cache directory in your project.

```sh
cargo install cargo-run-bin
cd my/rust/project
echo ".bin/" >> .gitignore
```

## Usage

`cargo-run-bin` keeps track of the binaries and their versions from within `Cargo.toml` under the `package.metadata.bin`
table. A quick example taken from this repo:

```toml
[package.metadata.bin]
cargo-nextest = { version = "0.9.57" }
dprint = { version = "0.30.3", locked = true }
tauri-mobile = { version = "0.5.2", bins = ["cargo-android", "cargo-mobile"], locked = true }
```

- `version` specifies the version of the crate.
- `bins` _(optional)_ is an array of binaries that the crate contains that you wish to build. These can be found in a crates
  `Cargo.toml` file. See [`tauri-mobile`](https://github.com/tauri-apps/tauri-mobile/blob/a5f3783870f48886e3266e43f92a6768fb1eb3d4/Cargo.toml#L18-L28) as an example.
- `locked` _(optional)_ is a parameter when set to `true` runs `cargo install` with [`--locked`](https://doc.rust-lang.org/cargo/commands/cargo-install.html#dealing-with-the-lockfile) parameter.

### cargo bin CRATE

Taking an example of `dprint`, running `cargo bin dprint --help` with build and cache the dprint binary with the
specified version in `Cargo.toml`. All future executions will run instantly without a build step, and dprint can be used
as you wish!

### cargo bin --sync-aliases

With the power of [cargo aliases](https://doc.rust-lang.org/cargo/reference/config.html#alias), `cargo bin --sync-aliases`
will create aliases for any `cargo-*` crate, allowing you to execute commands such `cargo nextest run` that will use
`cargo bin` under the hood. Check out some of the example from [this repo](.cargo/config.toml).

### cargo bin --build

When pulling down a new repo, or adding a step to CI, `cargo bin --build` will build all binaries that have not been
cached which are configured in `Cargo.toml`.

## [License](./LICENSE)

MIT.
