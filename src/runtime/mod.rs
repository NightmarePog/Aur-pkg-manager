use crate::ui;

pub fn build(package: &str) -> anyhow::Result<()> {
    ui::step(format!("Building {package}"));

    // TODO: run makepkg

    ui::success(format!("Built {package}"));

    Ok(())
}

pub fn launch(package: &str) -> anyhow::Result<()> {
    ui::step(format!("Launching {package} in sandbox"));

    // TODO: run sandbox

    ui::success(format!("Launched {package}"));

    Ok(())
}
