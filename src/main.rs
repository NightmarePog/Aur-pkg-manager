mod aur;
mod cli;
mod bwrap;
mod ui;
mod config;
mod build;
mod dependency;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
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
