#[derive(Debug, Clone)]
pub enum PackageSource {
    Installed,
    Repo,
    Aur,
}
