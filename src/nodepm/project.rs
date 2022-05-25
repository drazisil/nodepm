use anyhow::{anyhow, Result};
use json::object;
use std::fs::OpenOptions;

/// Create a new package.json file at `path` and set the name to `project_name`
/// 
/// # Errors
/// 
/// Will return `Err` in the following cases:
/// * Attempting to create a project in a directory where one already exists and `--force` is not passed
/// * There is an unknown ios error on file creation
/// * There is an unknown error on adding the name to the JSON object
/// * There is an error truncating the project file
/// * There is an error writing the project file
/// 
/// # TODO: Reduce error cases by bubbling or handling errors <https://github.com/drazisil/nodepm/issues/10>
pub fn create_project_manifest(
    project_name: &str,
    path: &std::path::PathBuf,
    force: bool,
) -> Result<()> {
    println!(
        "Initializing a new project named {:?} in {:?}, force: {:?}",
        project_name, path, force
    );
    let file_name = "package.json";

    let result = if force {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.with_file_name(file_name))
    } else {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.with_file_name(file_name))
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
    if let Err(error) = result {
        return Err(anyhow!(
            "Unknown error truncating adding name to json array on project file: {}",
            error,
        ));
    }

    let result = file.set_len(0);
    if let Err(error) = result {
        return Err(anyhow!(
            "Unknown error truncating project file '{}'. Please file an issue: {}/issues/new",
            error,
            env!("CARGO_PKG_REPOSITORY")
        ));
    };

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
