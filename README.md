![cargo-run-bin](https://i.imgur.com/0XAUMLa.jpg)

[![Build status](https://github.com/dustinblackman/cargo-run-bin/workflows/ci/badge.svg)](https://github.com/dustinblackman/cargo-run-bin/actions)
[![Crates.io](https://img.shields.io/crates/v/cargo-run-bin.svg)](https://crates.io/crates/cargo-run-bin)

A simple tool to build, cache, and run binaries scoped in `Cargo.toml` rather than installing globally. This acts similarly to [`npm run`](https://docs.npmjs.com/cli/v7/commands/npm-run-script) and [`gomodrun`](https://github.com/dustinblackman/gomodrun).

__Disclaimer:__ This tool was built out of a desired missing piece in the Rust development experience when initially learning the language, and is not likely to be optimal or feature complete just yet.

## Installation

```sh
  cargo install cargo-run-bin
```

## Usage

1. Add your binary dependency to your `Cargo.toml` like any dependency

```sh
  cargo add --dev flamegraph
```

2. Run your desired command prefixed with `cargo bin`. Note first runs will build your binary silently first before executing, all future runs will be instant.

```sh
  cargo bin flamegraph --help
```

3. Update your .gitignore to exclude the cached binaries.

```sh
  echo ".bin/" >> .gitignore
```

## [License](./LICENSE)

MIT.
