mod app;
mod cli;
mod config;
mod ffi;
mod render;

use clap::Parser;
use std::process;

fn main() {
    let args = cli::args::CliArgs::parse();
    if let Err(message) = run(args) {
        eprintln!("{message}");
        process::exit(2);
    }
}

fn run(args: cli::args::CliArgs) -> Result<(), String> {
    let options = config::normalize::normalize(args)?;
    app::run_parse::run(options)
}
