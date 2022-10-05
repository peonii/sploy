use std::io;
use crate::error::Error;
use crate::project::{Manifest, PackageMeta};

pub async fn init() -> Result<(), Error> {
    let name;
    let version;
    let description;
    let repository;

    let mut input = String::new();

    println!("Enter the name of the project:");
    io::stdin().read_line(&mut input)?;
    name = input.trim().to_string();

    input.clear();

    println!("Enter the version of the project:");
    io::stdin().read_line(&mut input)?;
    version = input.trim().to_string();

    input.clear();

    println!("Enter the description of the project:");
    io::stdin().read_line(&mut input)?;
    description = input.trim().to_string();

    input.clear();

    println!("Enter the repository of the project:");
    io::stdin().read_line(&mut input)?;
    repository = input.trim().to_string();

    input.clear();

    let manifest = Manifest::new(PackageMeta {
        name: name,
        version: version,
        description: description,
        repository: repository,
    });

    manifest.save()?;

    std::fs::create_dir("sploy_deps")?;

    Ok(())
}