use crate::dependency::PackageNode;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct DependencyGraph {
    pub packages: HashMap<String, PackageNode>,
}

impl DependencyGraph {
    pub fn insert(&mut self, package: PackageNode) {
        self.packages.insert(package.name.clone(), package);
    }

    pub fn install_order<B: FromIterator<PackageNode>>(&self) -> B {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        self.packages.keys().for_each(|name| {
            self.visit(name, &mut visited, &mut result);
        });

        result.into_iter().collect()
    }

    fn visit(&self, name: &str, visited: &mut HashSet<String>, result: &mut Vec<PackageNode>) {
        if !visited.insert(name.into()) {
            return;
        }

        if let Some(node) = self.packages.get(name) {
            self.visit_dependencies(node, visited, result);
            result.push(node.clone());
        }
    }

    fn visit_dependencies(
        &self,
        node: &PackageNode,
        visited: &mut HashSet<String>,
        result: &mut Vec<PackageNode>,
    ) {
        node.dependencies
            .iter()
            .filter(|dep| dep.kind.is_resolvable())
            .for_each(|dep| self.visit(&dep.name, visited, result));
    }
}
