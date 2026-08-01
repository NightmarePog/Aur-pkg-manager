pub mod rpc;

use std::{
    fmt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use thiserror::Error;

use crate::config;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Package<'a> {
    name: &'a str,
    base: &'a str,
    directory: PathBuf,
}

#[derive(Debug, Error)]
#[error("unsafe package name: {0}")]
pub struct PackageNameParseError(String);

impl<'a> Package<'a> {
    pub fn new(
        name: &'a str,
        base: &'a str,
        directory: impl Into<PathBuf>,
    ) -> Result<Self, PackageNameParseError> {
        if name.contains('/') {
            Err(PackageNameParseError(name.to_string()))
        } else {
            Ok(Self {
                name,
                base,
                directory: directory.into(),
            })
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn base(&self) -> &str {
        self.base
    }

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    pub fn clone_repository(&self) -> Result<(), CloneError> {
        let url = format!("{}/{}.git", config::AUR_URL, self.base);

        let output = Command::new("git")
            .arg("clone")
            .arg(&url)
            .arg(&self.directory)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(CloneError::spawn)?;

        if !output.status.success() {
            return Err(CloneError::GitCloneFailed {
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        Ok(())
    }

    pub fn clean(&self) -> Result<(), std::io::Error> {
        std::fs::remove_dir_all(&self.directory)
    }
}

impl fmt::Display for Package<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Debug, Error)]
pub enum CloneError {
    #[error("git is not installed")]
    Missing,

    #[error("failed to execute git")]
    Git(#[source] std::io::Error),

    #[error("git clone failed: {stderr}")]
    GitCloneFailed {
        stderr: String,
    },
}

impl CloneError {
    fn spawn(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => Self::Missing,
            _ => Self::Git(error),
        }
    }
}
