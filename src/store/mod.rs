use crate::ui;

pub fn save(package: &str) -> anyhow::Result<()> {
    ui::step(format!("Saving {package} to store"));

    // TODO: save package

    ui::success(format!("Saved {package}"));

    Ok(())
}

pub fn remove(package: &str) -> anyhow::Result<()> {
    ui::step(format!("Removing {package} from store"));

    // TODO: remove package

    ui::success(format!("Removed {package}"));

    Ok(())
}
