use json::object;
use quicli::prelude::*;
use std::fs::OpenOptions;
use super::error::error_and_exit;


pub fn init_project(project_name: &str, path: std::path::PathBuf, force: bool) -> CliResult {
    info!(
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
                    println!(
                        "The file already exists. Use 'init --force' if you want to overwrite it."
                    );
                    std::process::exit(-1)
                }
                _ => {
                    println!("Unknown error creating project file '{}'. Please file an issue: {}/issues/new", error, env!("CARGO_PKG_REPOSITORY"));
                    std::process::exit(-1)
                }
            };
        }
    };

    let mut project_json = object! {};
    let result = project_json.insert("name", project_name);
    match result {
        Err(error) => error_and_exit(
            "Unknown error truncating adding name to json array on project file",
            &error,
        ),
        _ => {}
    }

    let result = file.set_len(0);
    match result {
        Err(error) => {
            println!(
                "Unknown error truncating project file '{}'. Please file an issue: {}/issues/new",
                error,
                env!("CARGO_PKG_REPOSITORY")
            );
            std::process::exit(-1);
        }
        _ => {}
    }

    let result = project_json.write_pretty(&mut file, 2);
    match result {
        Err(error) => {
            println!(
                "Unknown error writing project file '{}'. Please file an issue: {}/issues/new",
                error,
                env!("CARGO_PKG_REPOSITORY")
            );
            std::process::exit(-1);
        }
        _ => {Ok(())}
    }
}