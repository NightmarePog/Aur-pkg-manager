mod graph;
mod package;
mod source;
mod resolver;
mod plan;

pub use graph::DependencyGraph;
pub use package::{
    installed_packages,
    AurMeta,
    PacmanError,
    PackageNode,
};
pub use source::PackageSource;
pub use resolver::{ResolveError, Resolver};
pub use plan::InstallPlan;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DependencyKind {
    Runtime,
    Build,
    Check,
    Optional,
    Provides,
    Conflicts,
}


impl DependencyKind {
    pub fn is_resolvable(self) -> bool {
        matches!(self, Self::Runtime | Self::Build | Self::Check)
    }
}


#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub kind: DependencyKind,
}


impl Dependency {
    pub fn new(raw: &str, kind: DependencyKind) -> Self {
        Self {
            name: normalize_name(raw).to_string(),
            kind,
        }
    }
}


pub fn normalize_name(dependency: &str) -> &str {
    dependency
        .split(['>', '<', '=', ':'])
        .next()
        .unwrap_or(dependency)
        .trim()
}

pub fn parse_size(value: &str) -> Option<u64> {
    let parts: Vec<&str> =
        value.split_whitespace().collect();


    let number: f64 =
        parts.first()?.parse().ok()?;


    match *parts.get(1)? {
        "KiB" => Some((number * 1024.0) as u64),
        "MiB" => Some((number * 1024.0 * 1024.0) as u64),
        "GiB" => Some((number * 1024.0 * 1024.0 * 1024.0) as u64),
        _ => None,
    }
}
