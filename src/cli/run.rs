pub fn run(package: String) -> anyhow::Result<()> {
    tracing::info!("Running {package}");

    crate::runtime::launch(&package)?;

    Ok(())
}
