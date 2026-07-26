mod aur;
mod cli;
mod logging;
mod runtime;
mod store;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    logging::init_logging();

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Install { package } => {
            cli::install::install(package)?;
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
