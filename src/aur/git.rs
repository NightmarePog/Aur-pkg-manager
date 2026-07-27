use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

use crate::{config, ui};


pub fn clone_repository(
    package_base: &str,
    destination: &Path,
) -> Result<()> {

    let url = format!(
        "{}/{}.git",
        config::AUR_URL,
        package_base
    );

    ui::step(format!("Cloning {package_base} repository"));

    let output = Command::new("git")
        .args([
            "clone",
            &url,
            destination.to_string_lossy().as_ref(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("failed to run git")?;

    if !output.status.success() {
        ui::error("Failed to clone repository");

        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            ui::debug(stderr);
        }

        bail!("git clone failed");
    }

    Ok(())
}
