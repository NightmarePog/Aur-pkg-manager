use crate::{
    aur::rpc::RpcPackage,
    dependency::{Dependency, source::PackageSource},
};
use std::{collections::HashMap, process::Command, string::FromUtf8Error};
use thiserror::Error;

fn parse_size(value: &str) -> Option<u64> {
    let mut parts = value.split_whitespace();
    let number: f64 = parts.next()?.parse().ok()?;

    match parts.next()? {
        "KiB" => Some((number * 1024.0) as u64),
        "MiB" => Some((number * 1024.0 * 1024.0) as u64),
        "GiB" => Some((number * 1024.0 * 1024.0 * 1024.0) as u64),
        _ => None,
    }
}

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

    #[error("failed to compare package versions")]
    VersionCompare(#[source] std::io::Error),

    #[error("vercmp returned invalid output")]
    InvalidVersionCompare,

    #[error("provider '{0}' not found")]
    ProviderNotFound(String),
}

impl PacmanError {
    pub fn spawn(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => Self::Missing,
            _ => Self::Pacman(error),
        }
    }

    pub fn version_compare(error: std::io::Error) -> Self {
        Self::VersionCompare(error)
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
    pub dependencies: Vec<Dependency>,
    pub size: Option<u64>,
    pub download_size: Option<u64>,
    pub provides: Vec<String>,
    pub packager: Option<String>,
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
            provides: info.provides.clone(),
            packager: None,
            aur: Some(AurMeta::from_rpc(info)),
        }
    }

    pub fn from_pacman(target: &str) -> Result<Self, PacmanError> {
        let name = crate::dependency::normalize_name(target).to_owned();

        let output = Command::new("pacman")
            .env("LC_ALL", "C")
            .args(["-Si", target])
            .output()
            .map_err(PacmanError::spawn)?;

        if !output.status.success() {
            Err(PacmanError::NotFound(name))
        } else {
            Ok(Self::parse_pacman(
                &name,
                &String::from_utf8(output.stdout)?,
                PackageSource::Repo,
            ))
        }
    }

    pub fn from_installed(name: &str) -> Result<Self, PacmanError> {
        let output = Command::new("pacman")
            .args(["-Qi", name])
            .output()
            .map_err(PacmanError::spawn)?;

        if !output.status.success() {
            Err(PacmanError::NotFound(name.to_owned()))
        } else {
            Ok(Self::parse_pacman(
                name,
                &String::from_utf8(output.stdout)?,
                PackageSource::Installed,
            ))
        }
    }

    fn parse_pacman(name: &str, text: &str, source: PackageSource) -> Self {
        text.lines()
            .filter_map(|line| line.split_once(':'))
            .map(|(k, v)| (k.trim(), v.trim()))
            .fold(
                Self {
                    name: name.into(),
                    version: None,
                    source,
                    dependencies: Vec::new(),
                    size: None,
                    download_size: None,
                    provides: Vec::new(),
                    packager: None,
                    aur: None,
                },
                |r, (key, value)| match key {
                    "Version" => Self {
                        version: Some(value.into()),
                        ..r
                    },
                    "Installed Size" => Self {
                        size: parse_size(value),
                        ..r
                    },
                    "Download Size" => Self {
                        download_size: parse_size(value),
                        ..r
                    },
                    "Provides" => Self {
                        provides: value.split_whitespace().map(str::to_owned).collect(),
                        ..r
                    },
                    "Packager" => Self {
                        packager: Some(value.into()),
                        ..r
                    },
                    _ => r,
                },
            )
    }
}

impl AurMeta {
    fn from_rpc(info: &RpcPackage) -> Self {
        Self {
            base: info.package_base.clone(),
            maintainer: info.maintainer.clone(),
            submitter: info.submitter.clone(),
            description: info.description.clone(),
            url: info.url.clone(),
            votes: info.votes,
            popularity: info.popularity,
            out_of_date: info.out_of_date,
            last_modified: info.last_modified,
        }
    }
}
