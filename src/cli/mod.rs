use std::path::PathBuf;

use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::{
    aur,
    build::{BuildError, SandboxError},
    dependency::{PacmanError, ResolveError},
    ui,
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

    #[error(transparent)]
    Sandbox(#[from] SandboxError),

    #[error(transparent)]
    Build(#[from] BuildError),

    #[error(transparent)]
    Ui(#[from] ui::UiError),

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

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
    Install {
        #[arg(value_name = "PACKAGE", required = true)]
        packages: Vec<String>,
    },
    Run {
        #[arg(value_name = "PACKAGE")]
        package: String,
    },
    Remove {
        #[arg(value_name = "PACKAGE")]
        package: String,
    },
}

pub struct InstalledPackages(pub Vec<PathBuf>);
