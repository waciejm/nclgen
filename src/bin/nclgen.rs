use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    match nclgen::Cli::parse().exec() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
