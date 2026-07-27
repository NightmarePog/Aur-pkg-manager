use anyhow::{Context, Result};
use crate::config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcResponse {
    pub results: Vec<RpcPackage>,
}

#[derive(Debug, Deserialize)]
pub struct RpcPackage {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Maintainer")]
    pub maintainer: Option<String>,

    #[serde(rename = "PackageBase")]
    pub package_base: String,
}



pub fn fetch_package_info(package: &str) -> Result<RpcPackage> {
    let url = format!(
        "{}/rpc/v5/info/{package}",
        config::AUR_URL
    );

    let response: RpcResponse = reqwest::blocking::get(url)
        .context("failed to reach AUR RPC")?
        .json()
        .context("failed to parse AUR response")?;

    response
        .results
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!(
            "package '{package}' not found"
        ))
}
