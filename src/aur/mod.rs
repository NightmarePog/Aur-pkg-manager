use std::path::PathBuf;
use std::process::Command;

use anyhow::{bail, Context};
use serde::Deserialize;

#[derive(Deserialize)]
struct RpcResponse {
    results: Vec<RpcPackage>,
}

#[derive(Deserialize)]
struct RpcPackage {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Maintainer")]
    maintainer: Option<String>,
    #[serde(rename = "PackageBase")]
    package_base: String,
}

pub fn clone(package: &str) -> anyhow::Result<PathBuf> {
    let info_url = format!("https://aur.archlinux.org/rpc/v5/info/{}", package);
    let response: RpcResponse = reqwest::blocking::get(&info_url)
        .context("failed to reach AUR RPC")?
        .json()
        .context("failed to parse AUR RPC response")?;

    if response.results.is_empty() {
        bail!("package '{package}' not found on AUR");
    }
    let package_info = &response.results[0];

    let maintainer = package_info.maintainer.as_deref().unwrap_or("none");
    tracing::info!(
        "{} {} (maintainer: {maintainer})",
        package_info.name,
        package_info.version,
    );

    // anti escape
    if package_info.name.contains('/') || package_info.name.contains("..") {
        bail!(
            "refusing to clone package with unsafe name: {}",
            package_info.name
        );
    }

    let git_url = format!("https://aur.archlinux.org/{}.git", package_info.package_base);
    let cwd = std::env::current_dir().context("failed to determine current directory")?;
    let dest = cwd.join(&package_info.name);

    tracing::info!("Cloning {git_url} into {}", dest.display());

    let status = Command::new("git")
        .args(["clone", &git_url, dest.to_string_lossy().as_ref()])
        .status()
        .context("failed to run git")?;

    if !status.success() {
        bail!("git clone failed for {package}");
    }

    Ok(dest)
}
