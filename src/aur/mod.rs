pub mod rpc;

use std::{
    fmt,
    path::Path,
    process::{Command, Stdio},
};

use thiserror::Error;

use crate::{config, ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Package<'a> {
    name: &'a str,
    base: &'a str,
}

#[derive(Debug, Error)]
#[error("unsafe package name: {0}")]
pub struct PackageNameParseError(String);

impl<'a> Package<'a> {
    pub fn new(
        name: &'a str,
        base: &'a str,
    ) -> Result<Self, PackageNameParseError> {
        if name.contains('/') {
            return Err(PackageNameParseError(name.to_string()));
        }

        Ok(Self {
            name,
            base,
        })
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn base(&self) -> &str {
        self.base
    }

    pub fn clone_repository(
        &self,
        destination: &Path,
    ) -> Result<(), CloneError> {
        let url = format!(
            "{}/{}.git",
            config::AUR_URL,
            self.base,
        );

        ui::step(format!("Cloning {} repository", self.name));

        let output = Command::new("git")
            .args([
                "clone",
                &url,
                destination.to_string_lossy().as_ref(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            return Err(CloneError::GitCloneFailed {
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        Ok(())
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
