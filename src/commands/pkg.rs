use std::process::Command;
use std::path;

use crate::Error;
use crate::project::{Manifest, Dependency};

pub fn add_package(source: &str) -> Result<(), Error> {
    let mut manifest = Manifest::load()?;

    // check if the package is already installed
    for dependency in manifest.get_dependencies() {
        if dependency.source == source {
            println!("Package is already installed!");
            std::process::exit(1);
        }
    }

    // start timing
    let start = std::time::Instant::now();

    let mut command = Command::new("git");

    command.arg("clone").arg(source).arg("_temp");

    command.output()?;

    let temp_path = path::Path::new("_temp").join("package.json");
    let temp_manifest = Manifest::load_path(temp_path.to_str().unwrap())?;
    let package_name = temp_manifest.get_name().to_string();
    let later = package_name.clone();

    let package_path = path::PathBuf::from("sploy_deps").join(package_name);
    std::fs::create_dir_all(&package_path)?;

    // move the files from _temp to sploy_deps
    for entry in std::fs::read_dir("_temp")? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let mut new_path = package_path.clone();
        new_path.push(file_name);
        std::fs::rename(path, new_path).unwrap_or_else(|_| {
            println!("Package is already installed!");
            std::fs::remove_dir_all("_temp").unwrap();
            std::process::exit(1);
        });
    }

    manifest.add_dependency(Dependency {
        name: later,
        source: source.to_string(),
        version: temp_manifest.get_meta().version.clone(),
    });

    manifest.save()?;

    std::fs::remove_dir("_temp")?;

    let duration = start.elapsed();

    println!("Package {} installed in {:?}", temp_manifest.get_name(), duration);

    Ok(())
}

pub fn remove_package(name: &str) -> Result<(), Error> {
    let mut manifest = Manifest::load()?;

    let mut index = 0;
    let mut found = false;

    for dependency in manifest.get_dependencies() {
        if dependency.name == name {
            found = true;
            break;
        }
        index += 1;
    }

    if !found {
        println!("Package not found!");
        std::process::exit(1);
    }

    let dependency = manifest.get_dependencies().remove(index);

    let package_path = path::PathBuf::from("sploy_deps").join(dependency.name);

    std::fs::remove_dir_all(package_path)?;

    manifest.save()?;

    Ok(())
}