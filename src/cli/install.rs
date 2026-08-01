use std::{io, path::Path};

use crate::{
    aur, cli::CliError, config, dependency::{self, InstallPlan, PackageNode, PackageSource}, ui::{self, prompt},
};


pub fn install<'a, T: IntoIterator<Item = &'a str>>(package_names: T, verbose: bool) -> Result<(), CliError>

{
    let package_names: Vec<&str> = package_names.into_iter().collect();

    ui::step(&format!("Installing {}", package_names.join(", ")));
    ui::step("Dependency resolution");
    let graph = dependency::Resolver::new()?
        .resolve(package_names)?;

    let plan = InstallPlan::from_graph(&graph);
    ui::header("Install plan");
    ui::install_plan(&plan);

    if verbose {
        ui::aur_details(&plan);
    }
    use owo_colors::OwoColorize;

    ui::step(format!(
        "continue? {}",
        "[y/n]".green()
    ));

    if !confirm() {
        Err(CliError::UserCancelled)
    } else {
        fetch_sources(&plan)?;
        Ok(())
    }

}


pub fn confirm() -> bool {
    let mut input = prompt();


    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

fn fetch_sources(plan: &InstallPlan) -> Result<(), CliError> {
    let aur_packages = plan
        .packages
        .iter()
        .filter(|package| {
            matches!(package.source, PackageSource::Aur)
        });


    for package in aur_packages {
        let repository = repository_of(package)?;


        if repository.directory().exists() {
            ui::info(format!("{package_name} already cloned", package_name = repository.name()));

            continue;
        }


        repository.clone_repository()?;

        ui::success(format!("cloned {}", repository.base()));
    }


    Ok(())
}


fn repository_of(
    package: &PackageNode,
) -> Result<aur::Package<'_>, aur::PackageNameParseError> {
    let base = package
        .aur
        .as_ref()
        .map(|aur| aur.base.as_str())
        .unwrap_or(&package.name);


    aur::Package::new(
        &package.name,
        base,
        Path::new(config::BUILD_PATH).join(&package.name),
    )
}
