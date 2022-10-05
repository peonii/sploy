use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageMeta {
    pub name: String,
    pub description: String,
    pub version: String,
    pub repository: String,
}

