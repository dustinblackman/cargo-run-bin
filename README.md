![cargo-run-bin](https://i.imgur.com/0XAUMLa.jpg)

[![Build status](https://github.com/dustinblackman/cargo-run-bin/workflows/ci/badge.svg)](https://github.com/dustinblackman/cargo-run-bin/actions)
[![Coverage Status](https://coveralls.io/repos/github/dustinblackman/cargo-run-bin/badge.svg?branch=master)](https://coveralls.io/github/dustinblackman/cargo-run-bin?branch=master)
[![Crates.io](https://img.shields.io/crates/v/cargo-run-bin.svg)](https://crates.io/crates/cargo-run-bin)

A simple tool to build, cache, and run binaries scoped in `Cargo.toml` rather than installing globally. This acts similarly to [`npm run`](https://docs.npmjs.com/cli/v7/commands/npm-run-script) and [`gomodrun`](https://github.com/dustinblackman/gomodrun).

**Disclaimer:** This tool was built out of wanting a missing piece in the Rust development experience when initially learning the language, and is not likely to be optimal or feature complete just yet.

## Installation

```sh
cargo install cargo-run-bin
```

## Usage

1. Add your binary dependency to your `Cargo.toml` like any dependency

```sh
cargo add --dev cross
```

If you don't wish to include `cross` in your `Cargo.lock`, you can also do the following in `Cargo.toml`:

```toml
[package.metadata.bin]
cross = "0.2.4"
```

2. Run your desired command prefixed with `cargo bin`. Note first runs will build your binary silently first before executing, all future runs will be instant.

```sh
cargo bin cross --help
```

3. Update your .gitignore to exclude the cached binaries.

```sh
echo ".bin/" >> .gitignore
```

## [License](./LICENSE)

MIT.
