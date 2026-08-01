mod aur;
mod cli;
mod bwrap;
mod ui;
mod config;
mod build;
mod dependency;

use std::{error::Error, process::ExitCode};

use clap::Parser;
use cli::Cli;

fn main() -> ExitCode {
    match dispatch() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            report(&error);

            ExitCode::FAILURE
        }
    }
}

fn dispatch() -> Result<(), cli::CliError> {
    let cli = Cli::parse();

    match cli.command {
        cli::Command::Install { packages } => {
            cli::install::install(packages.iter().map(String::as_str), cli.verbose)
        }
        cli::Command::Run { package } => {
            cli::run::run(package)
        }
        cli::Command::Remove { package } => {
            cli::remove::remove(package)
        }
    }
}

fn report(error: &dyn Error) {
    ui::error(error.to_string());

    let mut source = error.source();

    while let Some(cause) = source {
        ui::error(format!("caused by: {cause}"));

        source = cause.source();
    }
}
