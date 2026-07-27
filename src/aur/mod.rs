mod git;
mod package;
mod rpc;

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use crate::{config, ui};

pub fn clone(package: &str) -> Result<PathBuf> {
    let info = rpc::fetch_package_info(package)?;

    show_info(&info);

    validate_name(&info.name)?;

    let destination = Path::new(config::STORE_PATH)
        .join(&info.name);

    git::clone_repository(
        &info.package_base,
        &destination,
    )?;

    ui::success(format!("Cloned {}", info.name));

    Ok(destination)
}


fn show_info(package: &package::RpcPackage) {
    let maintainer = package
        .maintainer
        .as_deref()
        .unwrap_or("none");

    ui::info(format!(
        "{} {} (maintainer: {maintainer})",
        package.name,
        package.version
    ));
}


fn validate_name(name: &str) -> Result<()> {
    if name.contains('/') {
        bail!("unsafe package name: {name}");
    }

    Ok(())
}
