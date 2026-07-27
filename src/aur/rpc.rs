use anyhow::{Context, Result};
use crate::config;
use super::package::{RpcPackage, RpcResponse};


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
