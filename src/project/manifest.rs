use crate::{project::{PackageMeta, Dependency}, error::Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    meta: PackageMeta,
    dependencies: Vec<Dependency>,
}

impl Manifest {
    pub fn new(meta: PackageMeta) -> Manifest {
        Manifest {
            meta: meta,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }

    pub fn get_dependencies(&mut self) -> &mut Vec<Dependency> {
        &mut self.dependencies
    }

    pub fn get_meta(&self) -> &PackageMeta {
        &self.meta
    }

    pub fn get_meta_mut(&mut self) -> &mut PackageMeta {
        &mut self.meta
    }

    pub fn get_name(&self) -> &str {
        &self.meta.name
    }

    pub fn save(&self) -> Result<(), Error> {
        let toml = serde_json::to_string_pretty(&self)?;
        std::fs::write("package.json", toml)?;
        Ok(())
    }

    pub fn load() -> Result<Manifest, Error> {
        let toml = std::fs::read_to_string("package.json")?;
        let manifest: Manifest = serde_json::from_str(&toml)?;
        Ok(manifest)
    }

    pub fn load_path(path: &str) -> Result<Manifest, Error> {
        let toml = std::fs::read_to_string(path)?;
        let manifest: Manifest = serde_json::from_str(&toml)?;
        Ok(manifest)
    }
}