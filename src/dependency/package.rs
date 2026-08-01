use std::{
    collections::HashMap,
    process::Command,
    string::FromUtf8Error,
};
use thiserror::Error;
use crate::aur::rpc::RpcPackage;
use super::source::PackageSource;


#[derive(Debug, Error)]
pub enum PacmanError {
    #[error("pacman is not installed")]
    Missing,

    #[error("failed to execute pacman")]
    Pacman(#[source] std::io::Error),

    #[error("package '{0}' not found in the official repositories")]
    NotFound(String),

    #[error("pacman failed to list installed packages")]
    Query,

    #[error("pacman returned invalid UTF-8")]
    Encoding(#[from] FromUtf8Error),

    #[error("provider '{0}' not found")]
    ProviderNotFound(String),

    #[error("Pacman output contains invalid UTF-8 for package: {0}")]
    InvalidUtf8(String),
}


impl PacmanError {
    pub fn spawn(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => Self::Missing,
            _ => Self::Pacman(error),
        }
    }
}

pub fn installed_packages() -> Result<HashMap<String, String>, PacmanError> {
    let output = Command::new("pacman")
        .arg("-Q")
        .output()
        .map_err(PacmanError::spawn)?;


    if !output.status.success() {
        Err(PacmanError::Query)
    } else {
        Ok(String::from_utf8(output.stdout)?
            .lines()
            .filter_map(|l| l.split_once(' '))
            .map(|(n, v)| (n.into(), v.trim().into()))
            .collect())
    }
}


#[derive(Debug, Clone)]
pub struct PackageNode {
    pub name: String,
    pub version: Option<String>,
    pub source: PackageSource,
    pub dependencies: Vec<super::Dependency>,
    pub size: Option<u64>,
    pub download_size: Option<u64>,
    pub aur: Option<AurMeta>,
}


#[derive(Debug, Clone)]
pub struct AurMeta {
    pub base: String,
    pub maintainer: Option<String>,
    pub submitter: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub votes: u32,
    pub popularity: f64,
    pub out_of_date: Option<i64>,
    pub last_modified: i64,
}


impl PackageNode {
    pub fn from_rpc(info: &RpcPackage) -> Self {
        Self {
            name: info.name.clone(),
            version: Some(info.version.clone()),
            source: PackageSource::Aur,
            dependencies: info.dependencies(),
            size: None,
            download_size: None,
            aur: Some(AurMeta {
                base: info.package_base.clone(),
                maintainer: info.maintainer.clone(),
                submitter: info.submitter.clone(),
                description: info.description.clone(),
                url: info.url.clone(),
                votes: info.votes,
                popularity: info.popularity,
                out_of_date: info.out_of_date,
                last_modified: info.last_modified,
            }),
        }
    }


    pub fn installed(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: Some(version.into()),
            source: PackageSource::Installed,
            dependencies: Vec::new(),
            size: None,
            download_size: None,
            aur: None,
        }
    }

    pub fn from_pacman(name: impl Into<String>) -> Result<Self, PacmanError> {
        let name = name.into();

        let output = Command::new("pacman")
            .env("LC_ALL", "C")
            .args(["-Si", &name])
            .output()
            .map_err(PacmanError::spawn)?;

        if !output.status.success() {
            Err(PacmanError::NotFound(name))
        } else {
            Ok(Self::parse_pacman(
                &name,
                &String::from_utf8(output.stdout)?,
            ))
        }
    }

    fn parse_pacman(name: &str, text: &str) -> Self {
        text.lines()
            .filter_map(|line| line.split_once(':'))
            .map(|(k, v)| (k.trim(), v.trim()))
            .fold(
                Self {
                    name: name.into(),
                    version: None,
                    source: PackageSource::Repo,
                    dependencies: Vec::new(),
                    size: None,
                    download_size: None,
                    aur: None,
                },
                |r, (key, value)| match key {
                    "Version" => Self {
                        version: Some(value.into()),
                        ..r
                    },
                    "Installed Size" => Self {
                        size: super::parse_size(value),
                        ..r
                    },
                    "Download Size" => Self {
                        download_size: super::parse_size(value),
                        ..r
                    },
                    _ => r,
                },
            )
    }
}
