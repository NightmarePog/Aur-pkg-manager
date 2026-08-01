use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::{
    aur,
    dependency::{PacmanError, ResolveError},
};

pub mod install;
pub mod remove;
pub mod run;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Resolve(#[from] ResolveError),

    #[error(transparent)]
    Pacman(#[from] PacmanError),

    #[error(transparent)]
    PackageName(#[from] aur::PackageNameParseError),

    #[error(transparent)]
    Clone(#[from] aur::CloneError),

    #[error("user cancelled")]
    UserCancelled,
}

#[derive(Parser)]
#[command(name = "aur-pkg-manager")]
#[command(version)]
#[command(about = "Sandboxed AUR package manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Print full AUR metadata for every package in the plan
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
    Install { packages: Vec<String> },
    Run { package: String },
    Remove { package: String },
}
