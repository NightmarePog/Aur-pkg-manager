use std::{
    collections::HashMap,
    process::{Command, Stdio},
};
use thiserror::Error;
use crate::{
    aur::{self, rpc::RpcError},
    dependency::{self, DependencyGraph, PackageNode, PacmanError},
    ui,
};


#[derive(Debug, Error)]
pub enum ResolveError {
    #[error(transparent)]
    Rpc(#[from] RpcError),

    #[error(transparent)]
    Pacman(#[from] PacmanError),

    #[error("provider not found: {0}")]
    NotFound(String),
}


pub struct Resolver {
    graph: DependencyGraph,
    installed: HashMap<String, String>,
    providers: HashMap<String, String>,
}


impl Resolver {
    pub fn new() -> Result<Self, PacmanError> {
        Ok(Self {
            graph: DependencyGraph::default(),
            installed: dependency::installed_packages()?,
            providers: HashMap::new(),
        })
    }

    pub fn resolve<'a, T: IntoIterator<Item = &'a str>>(mut self, packages: T) -> Result<DependencyGraph, ResolveError> {
        packages.into_iter().try_for_each(|p| self.package(p).map(|_| ())).map(|_| self.graph)
    }


    fn package(&mut self, package: &str) -> Result<String, ResolveError> {
        let name = dependency::normalize_name(package);

        self.cached_provider(&name)
            .or_else(|_| self.resolve_repo(&name))
            .or_else(|_| self.resolve_provider(&name))
            .or_else(|_| self.resolve_aur(&name))
    }


    fn cached_provider(&self, name: &str) -> Result<String, ResolveError> {
        if let Some(provider) = self.providers.get(name) {
            Ok(provider.clone())
        } else {
            Err(ResolveError::NotFound(name.into()))
        }
    }

    fn resolve_installed(
        &mut self,
        name: &str,
    ) -> Option<String> {
        if let Some(version) = self.installed.get(name) {
            let version = version.clone();
            self.graph.insert(PackageNode::installed(name, version));
            self.remember_provider(name, name);
            Some(name.into())
        } else {
            None
        }
    }


    fn resolve_repo(
        &mut self,
        name: &str,
    ) -> Result<String, ResolveError> {
        let result = PackageNode::from_pacman(name)?;

        self.graph.insert(result);
        self.remember_provider(name, name);
        Ok(name.into())
    }


    fn resolve_provider(
        &mut self,
        name: &str,
    ) -> Result<String, ResolveError> {
        let provider = Self::provider_of(name)?;

        let provider = self.package(&provider)?;
        self.remember_provider(name, provider.clone());
        Ok(provider)
    }


    fn resolve_aur(
        &mut self,
        name: &str,
    ) -> Result<String, ResolveError> {

        let aur_info = aur::rpc::fetch_package_info(name)?;
        self.warn_about_aur(&aur_info);
        let mut node = PackageNode::from_rpc(&aur_info);
        self.remember_provider(name, aur_info.name.clone());
        self.resolve_dependencies(&mut node)?;
        self.graph.insert(node);
        Ok(aur_info.name)
    }


    fn warn_about_aur(
        &self,
        package: &aur::rpc::RpcPackage,
    ) {
        if package.orphan() {
            ui::warn(format!(
                "{} has no maintainer",
                package.name
            ));
        }

        if let Some(flagged) = package.out_of_date {
            ui::warn(format!(
                "{} was flagged out of date {}",
                package.name,
                ui::relative_time(flagged),
            ));
        }
    }


    fn resolve_dependencies(
        &mut self,
        node: &mut PackageNode,
    ) -> Result<(), ResolveError> {
        node.dependencies
            .iter_mut()
            .filter(|dependency| dependency.kind.is_resolvable())
            .try_for_each(|dependency| {
                dependency.name = self.package(&dependency.name)?;
                Ok(())
            })
    }


    fn remember_provider(
        &mut self,
        requested: impl Into<String>,
        provider: impl Into<String>,
    ) {
        self.providers.insert(
            requested.into(),
            provider.into(),
        );
    }

    pub fn provider_of(name: &str) -> Result<String, PacmanError> {
        let output = Command::new("pacman")
            .args([
                "-Sp",
                "--nodeps",
                "--nodeps",
                "--noconfirm",
                "--print-format=%n",
                name,
            ])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(PacmanError::spawn)?;

        if !output.status.success() {
            Err(PacmanError::ProviderNotFound(name.to_string()))
        } else {
            let stdout = String::from_utf8(output.stdout)
                .map_err(|_| PacmanError::InvalidUtf8(name.to_string()))?;

            let provider = stdout
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty() && *line != name)
                .ok_or_else(|| PacmanError::ProviderNotFound(name.to_string()))?;

            Ok(provider.to_string())
        }
    }
}
