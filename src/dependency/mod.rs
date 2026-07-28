pub mod resolver;

pub use resolver::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyKind {
    Runtime,
    Build,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub kind: DependencyKind,
}

#[derive(Debug, Clone)]
pub enum PackageSource {
    Installed,
    Repo,
    Aur,
}

#[derive(Debug, Clone)]
pub struct PackageNode {
    pub name: String,
    pub source: PackageSource,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Default)]
pub struct DependencyGraph {
    pub packages: HashMap<String, PackageNode>,
}
