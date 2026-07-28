pub mod rpc;

use std::{
    fmt,
    path::Path,
    process::{Command, Stdio},
};

use thiserror::Error;

use crate::{config};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Package<'a> {
    name: &'a str,
    base: &'a str,
    directory: &'a str,
}

#[derive(Debug, Error)]
#[error("unsafe package name: {0}")]
pub struct PackageNameParseError(String);

impl<'a> Package<'a> {
    pub fn new(
        name: &'a str,
        base: &'a str,
        directory: &'a str,
    ) -> Result<Self, PackageNameParseError> {
        if name.contains('/') {
            Err(PackageNameParseError(name.to_string()))
        } else {
            Ok(Self {
                name,
                base,
                directory,
            })
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn base(&self) -> &str {
        self.base
    }

    pub fn clone_repository(
        self,
        destination: &Path,
    ) -> Result<&Path, CloneError> {
        let url = format!(
            "{}/{}.git",
            config::AUR_URL,
            self.base,
        );


        let output = Command::new("git")
            .args([
                "clone",
                &url,
                self.directory,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            return Err(CloneError::GitCloneFailed {
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        Ok(destination)
    }

    pub fn clean(&self) -> Result<(), std::io::Error> {
        std::fs::remove_dir_all(self.directory)
    }
}

impl fmt::Display for Package<'_> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Debug, Error)]
pub enum CloneError {
    #[error("failed to execute git")]
    Git(#[from] std::io::Error),

    #[error("git clone failed: {stderr}")]
    GitCloneFailed {
        stderr: String,
    },
}
