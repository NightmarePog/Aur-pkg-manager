pub fn install(package: String) -> anyhow::Result<()> {
    tracing::info!("Installing {package}");

    crate::aur::clone(&package)?;
    crate::runtime::build(&package)?;
    crate::store::save(&package)?;

    Ok(())
}
