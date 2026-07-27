pub fn remove(package: String) -> anyhow::Result<()> {
    tracing::info!("Removing {package}");

    // TODO add here real implementation lol
    crate::store::remove(&package)?;

    Ok(())
}
