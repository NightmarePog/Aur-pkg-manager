use std::path::Path;

use crate::config;
use crate::ui;
use crate::aur;


pub fn install(package_name: &str) -> anyhow::Result<()> {
    ui::header(&format!("Installing {}", package_name));

    let info = aur::rpc::fetch_package_info(package_name)?;

    let package = aur::Package::new(
        &info.name,
        &info.package_base,
    )?;

    package.clone_repository(Path::new(config::STORE_PATH))?;

    //crate::runtime::build(&package)?;
    //crate::store::save(&package)?;

    Ok(())
}
