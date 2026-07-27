use crate::ui;

pub fn install(package: String) -> anyhow::Result<()> {
    ui::header(&format!("Installing {}", package));

    crate::aur::clone(&package)?;
    crate::runtime::build(&package)?;
    crate::store::save(&package)?;

    Ok(())
}
