#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use std::process;

use owo_colors::OwoColorize;

mod binary;
mod cargo_config;
mod cli;
mod metadata;

fn main() {
    let res = cli::run();

    // Only reached if run-bin code fails, otherwise process exits early from within
    // binary::run.
    if let Err(res) = res {
        eprintln!("{}", format!("run-bin failed: {res}").red());
        process::exit(1);
    }
}
