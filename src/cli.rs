use std::env;

use anyhow::bail;
use anyhow::Result;
use clap::Arg;
use clap::ArgMatches;
use clap::Command;
use owo_colors::OwoColorize;

use crate::binary;
use crate::metadata;
use crate::cargo_config;

fn build_all_binaries() -> Result<()> {
    let binary_packages = metadata::get_binary_packages()?;
    for binary_package in binary_packages {
        binary::build(binary_package)?;
    }

    println!("{}", "Done!".green());
    return Ok(());
}

fn sync_aliases() -> Result<()> {
    cargo_config::sync_aliases()?;
    println!("{}", "Done!".green());
    return Ok(());
}

fn run_binary(binary_name: String, args: Vec<String>) -> Result<()> {
    let binary_packages = metadata::get_binary_packages()?;
    let binary_package = binary_packages.iter().find(|&e| {
        return e.package == binary_name
            || (e.bin_target.is_some() && e.bin_target.as_deref().unwrap() == binary_name);
    });
    if binary_package.is_none() {
        bail!("No package found for binary {binary}");
    }

    let bin_path = binary::build(binary_package.unwrap().clone())?;
    binary::run(bin_path, args)?;

    return Ok(());
}

fn arg_used(matches: &ArgMatches, arg_long: &str) -> bool {
    if let Some(used) = matches.get_one::<bool>(arg_long) {
        if *used {
            return true;
        }
    }

    return false;
}

pub fn run() -> Result<()> {
    let mut app = Command::new("cargo-bin")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(false)
        .ignore_errors(true)
        .arg(
            Arg::new("sync-aliases")
                .short('s')
                .long("sync-aliases")
                .num_args(0)
                .help("Sync aliases for cargo-* commands in .cargo/config"),
        )
        .arg(
            Arg::new("build")
                .short('b')
                .long("build")
                .num_args(0)
                .help("Build all configured binaries, skips entries that are already built."),
        );

    let matches = app.clone().get_matches();

    if arg_used(&matches, "sync-aliases") {
        sync_aliases()?;
    } else if arg_used(&matches, "build") {
        build_all_binaries()?;
    } else {
        let mut args: Vec<_> = env::args().collect();
        let start_index = args.iter().position(|e| return e.ends_with("/cargo-bin"));
        if start_index.is_none() || start_index.unwrap() == (args.len() + 1) {
            app.print_long_help()?;
            return Ok(());
        }

        let bin_index = start_index.unwrap()+1;
        let binary_name = args[bin_index].clone();
        args.drain(0..(bin_index+1));
        run_binary(binary_name, args)?;
    }

    return Ok(());
}
