mod aur;
mod cli;
mod logging;
mod runtime;
mod store;
mod bwrap;
mod ui;
mod config;

use clap::Parser;
use cli::Cli;
use tracing_subscriber::fmt;

fn main() -> anyhow::Result<()> {
    logging::init_logging();

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Install { package } => {
            cli::install::install(&package)?;
        }
        cli::Command::Run { package } => {
            cli::run::run(package)?;
        }
        cli::Command::Remove { package } => {
            cli::remove::remove(package)?;
        }
    }

    Ok(())
}

pub fn init_logging() {
    fmt()
        .without_time()
        .with_target(false)
        .with_level(true)
        .init();
}
