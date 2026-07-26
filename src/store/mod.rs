pub fn save(package: &str) -> anyhow::Result<()> {
    tracing::info!("Saving {package} to store");

    Ok(())
}

pub fn remove(package: &str) -> anyhow::Result<()> {
    tracing::info!("Removing {package} from store");

    Ok(())
}
