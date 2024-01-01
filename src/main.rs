use std::process;

use owo_colors::OwoColorize;

fn main() {
    let res = cargo_run_bin::cli::run();

    // Only reached if run-bin code fails, otherwise process exits early from within
    // binary::run.
    if let Err(res) = res {
        eprintln!("{}", format!("run-bin failed: {res}").red());
        process::exit(1);
    }
}
