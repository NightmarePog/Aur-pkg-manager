pub fn run(package: String) -> Result<(), crate::cli::CliError> {
    crate::ui::step(&format!("Preparing to run {package}"));
    crate::ui::info("Run support is not implemented yet");

    Ok(())
}
