use clap::{Parser, Subcommand};
use json::object;
use std::fs::OpenOptions;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Required for dangerous operations
    #[clap(long)]
    force: bool,

    // Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project (use "init --force" to overwrite an existing one)
    Init { 
        /// Overwrite an existing package
        #[clap(long)]
        force: bool,

        project_name: String,

        /// defaults to the current directory
        #[clap(default_value = ".", hide_default_value =  true)]
        path: std::path::PathBuf
    }
}

fn create_project (project_name: String, path: std::path::PathBuf, force: bool) {
    println!("Initializing a new project named {:?} in {:?}, force: {:?}", project_name, path, force);
    let file_name = "package.json";

    let result = match force {
        false => OpenOptions::new().write(true).create_new(true).open(path.with_file_name(file_name)),
        true => OpenOptions::new().write(true).create(true).open(path.with_file_name(file_name))
    };

    let mut file = match result {
        Ok(file) => { file },
        Err(error) => { 
            match error.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    println!("The file already exists. Use 'init --force' if you want to overwrite it.");
                    std::process::exit(-1)
                },
                _ => {
                    println!("Unknown error creating project file '{}'. Please file an issue: https://github.com/drazisil/nodepm/issues/new", error);
                    std::process::exit(-1)

                }
            };
        }
    };

    let mut project_json = object!{};
    let result = project_json.insert("name", project_name);
    match result {
        Err(error) => {
            println!("Unknown error truncating adding name to json array on project file '{}'. Please file an issue: https://github.com/drazisil/nodepm/issues/new", error);
            std::process::exit(-1);
        },
        _ => {}
    }

    let result = file.set_len(0);
    match result {
        Err(error) => {
            println!("Unknown error truncating project file '{}'. Please file an issue: https://github.com/drazisil/nodepm/issues/new", error);
            std::process::exit(-1);
        },
        _ => {}
    }

    let result = project_json.write_pretty(&mut file, 2);
    match result {
        Err(error) => {
            println!("Unknown error writing project file '{}'. Please file an issue: https://github.com/drazisil/nodepm/issues/new", error);
            std::process::exit(-1);
        },
        _ => {}
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { project_name, path, force } => {
            create_project(project_name.to_string(), path.to_path_buf(), *force)
        }
    }
}
