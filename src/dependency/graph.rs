use std::collections::{HashMap, HashSet};
use super::package::PackageNode;


#[derive(Debug, Default, Clone)]
pub struct DependencyGraph {
    pub packages: HashMap<String, PackageNode>,
}


impl DependencyGraph {
    pub fn insert(&mut self, package: PackageNode) {
        self.packages.insert(
            package.name.clone(),
            package
        );
    }

    pub fn install_order<B: FromIterator<PackageNode>>(&self) -> B {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        self.packages
            .keys()
            .for_each(|name| {
                self.visit(name, &mut visited, &mut result);
            });

        result.into_iter().collect()
    }


    fn visit(
        &self,
        name: &str,
        visited: &mut HashSet<String>,
        result: &mut Vec<PackageNode>,
    ) {
        if visited.contains(name) {
            return;
        }


        visited.insert(name.to_string());


        if let Some(node) = self.packages.get(name) {
            for dep in &node.dependencies {
                if !dep.kind.is_resolvable() {
                    continue;
                }


                self.visit(
                    &dep.name,
                    visited,
                    result
                );
            }


            result.push(node.clone());
        }
    }
}
