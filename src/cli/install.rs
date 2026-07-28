use std::path::Path;

use srcinfo::Srcinfo;

use crate::build::{Environment, SandboxFiles};
use crate::config;
use crate::dependency::DependencyResolver;
use crate::ui;
use crate::aur;


pub fn install(package_name: &str) -> anyhow::Result<()> {
    ui::header(&format!("Installing {}", package_name));

    let info = aur::rpc::fetch_package_info(package_name)?;
    ui::step(format!("Cloning {} repository", info.name));
    let package = aur::Package::new(
        &info.name,
        &info.package_base,
        config::BUILD_PATH,
    )?;

    match package.clone_repository(Path::new(config::BUILD_PATH)) {
        Ok(directory) => {
            ui::header("Dependency resolution");
            let srcinfo = Srcinfo::from_path(&Path::new(config::BUILD_PATH).join(".SRCINFO"))?;
            let dependencies = DependencyResolver::from_srcinfo::<Vec<_>>(srcinfo)?;

            ui::header(&format!("Building {}", package_name));
            let sandbox = SandboxFiles::initialize()?;
            let env = Environment::new(&sandbox)?;


            Ok(())
        }
        Err(err) => {
            ui::error("failed to clone a repository");
            ui::error(err.to_string());
            ui::info("cleaning...");

            if let Err(err) = package.clean() {
                ui::error("failed to clean up");
                ui::error(err.to_string());
            }

            return Err(err.into());
        }
    }
}
