use anyhow::{anyhow, Result};
use json::object;
use std::fs::OpenOptions;

/// Create a new package.json file at `path` and set the name to `project_name`
pub fn init_project(project_name: &str, path: std::path::PathBuf, force: bool) -> Result<()> {
    println!(
        "Initializing a new project named {:?} in {:?}, force: {:?}",
        project_name, path, force
    );
    let file_name = "package.json";

    let result = match force {
        false => OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.with_file_name(file_name)),
        true => OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.with_file_name(file_name)),
    };

    let mut file = match result {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    return Err(anyhow!(
                        "The file already exists. Use 'init --force' if you want to overwrite it."
                    ));
                }
                _ => {
                    return Err(anyhow!("Unknown error creating project file '{}'. Please file an issue: {}/issues/new", error, env!("CARGO_PKG_REPOSITORY")));
                }
            };
        }
    };

    let mut project_json = object! {};
    let result = project_json.insert("name", project_name);
    match result {
        Err(error) => { return Err(anyhow!(
            "Unknown error truncating adding name to json array on project file: {}",
            error,
        ))},
        _ => {}
    }

    let result = file.set_len(0);
    match result {
        Err(error) => {
            return Err(anyhow!(
                "Unknown error truncating project file '{}'. Please file an issue: {}/issues/new",
                error,
                env!("CARGO_PKG_REPOSITORY")
            ));
        }
        _ => {}
    }

    let result = project_json.write_pretty(&mut file, 2);
    match result {
        Err(error) => {
            return Err(anyhow!(
                "Unknown error writing project file '{}'. Please file an issue: {}/issues/new",
                error,
                env!("CARGO_PKG_REPOSITORY")
            ));
        }
        _ => Ok(()),
    }
}
