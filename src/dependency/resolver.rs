use anyhow::Result;
use srcinfo::Srcinfo;
use super::{Dependency, DependencyKind};
pub struct DependencyResolver;


impl DependencyResolver {

    pub fn from_srcinfo<B: FromIterator<Dependency>>(srcinfo: Srcinfo) -> Result<B> {

        let runtime = srcinfo
            .depends()
            .iter()
            .flat_map(|arch| arch.iter())
            .map(|dep| Dependency {
                name: dep.to_string(),
                kind: DependencyKind::Runtime,
            });

        let build = srcinfo
            .makedepends()
            .iter()
            .flat_map(|arch| arch.iter())
            .map(|dep| Dependency {
                name: dep.to_string(),
                kind: DependencyKind::Build,
            });

        Ok(runtime.chain(build).collect())
    }
}
