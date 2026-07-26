use clap::{Parser, Subcommand};

pub mod install;
pub mod remove;
pub mod run;

#[derive(Parser)]
#[command(name = "aur-pkg-manager")]
#[command(version)]
#[command(about = "Sandboxed AUR package manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Install { package: String },
    Run { package: String },
    Remove { package: String },
}
