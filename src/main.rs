//! Test?

use clap::{Parser, Subcommand};
use json::object;
use std::error::Error;
use std::fs::OpenOptions;



const REGISTRY_HOST: &str = "https://registry.npmjs.com";

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
        /// Overwrite an existing project
        #[clap(long)]
        force: bool,

        project_name: String,

        /// defaults to the current directory
        #[clap(default_value = ".", hide_default_value = true)]
        path: std::path::PathBuf,
    },
    /// Inspect a package
    Inspect {
        project_name: String,

        /// Package version [default: 'latest']
        #[clap(default_value = "latest", hide_default_value = true)]
        version: String,
    },
}

fn error_and_exit(error_message: &str, origional_message: &(dyn Error + 'static)) -> ! {
    println!(
        "{} '{}'. Please file an issue: {}/issues/new",
        error_message,
        origional_message.to_string(),
        env!("CARGO_PKG_REPOSITORY")
    );
    std::process::exit(-1)
}

fn init_project(project_name: &str, path: std::path::PathBuf, force: bool) {
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
        _ => {}
    }
}

fn inspect_project(package_name: &str, version: &str) {
    println!(
        "Inspecting the package named {:?} at version {:?}",
        package_name, version
    );

    let user_agent = format!(
        "{}: {} - {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
    let query = format!("{}/{}", REGISTRY_HOST, package_name);

    println!("{} - {}\n\n", query, user_agent);

    // Requires the `json` feature enabled.
    let result = ureq::get(&query)
        .set("User-Agent", &user_agent)
        .set("Accept", "application/json")
        .call();

    let response_body_result = match result {
        Err(ureq::Error::Status(code, response)) => {
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
            println!("{} - {}", code, response.status_text());
            std::process::exit(-1);
        }
        Err(error) => {
            /* some kind of io/transport error */
            println!(
                "Unknown error querying registry '{}'. Please file an issue: {}/issues/new",
                error,
                env!("CARGO_PKG_REPOSITORY")
            );
            std::process::exit(-1);
        }
        Ok(response) => response.into_string(),
    };

    let response_body = match response_body_result {
        Err(error) => error_and_exit("Unknown error reading response body", &error),
        Ok(response) => response,
    };

    let result = json::parse(&response_body);

    let response_json = match result {
        Err(error) => {
            println!("Unknown error transforming response into JSON '{}'. Please file an issue: {}/issues/new", error, env!("CARGO_PKG_REPOSITORY"));
            std::process::exit(-1);
        }
        Ok(response) => response,
    };

    // Lets get the version blob
    let requested_version = match version {
        "latest" => response_json["dist-tags"]["latest"].as_str().unwrap_or(""),
        _ => version,
    };

    println!(
        "SHA1: {}",
        response_json["versions"][requested_version]["dist"]["shasum"]
    );
    println!(
        "SHA512: {}",
        response_json["versions"][requested_version]["dist"]["integrity"]
    );
    println!(
        "download_url: {}",
        response_json["versions"][requested_version]["dist"]["tarball"]
    )
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            project_name,
            path,
            force,
        } => init_project(project_name, path.to_path_buf(), *force),
        Commands::Inspect {
            project_name,
            version,
        } => inspect_project(project_name, version),
    }
}
