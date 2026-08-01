use super::{DependencyGraph, PackageNode};


pub struct InstallPlan {
    pub packages: Vec<PackageNode>,
}


impl InstallPlan {
    pub fn from_graph(graph: &DependencyGraph) -> Self {
        Self {
            packages: graph.install_order(),
        }
    }
}
