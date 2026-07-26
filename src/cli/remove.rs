pub fn remove(package: String) -> anyhow::Result<()> {
    tracing::info!("Removing {package}");

    // just drops it from the store for now, doesn't touch anything on disk yet
    crate::store::remove(&package)?;

    Ok(())
}
