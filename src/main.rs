//! Test?

mod nodepm;
use nodepm::package::inspect_package;
use nodepm::project::init_project;
use quicli::prelude::*;
use structopt::StructOpt;

const REGISTRY_HOST: &str = "https://registry.npmjs.com";

#[derive(StructOpt)]
#[structopt(name = "nodepm")]
struct Cli {
    // Subcommands
    #[structopt(subcommand)]
    command: Commands,
}

#[derive(Debug, StructOpt)]
enum Commands {
    /// Initialize a new project (use "init --force" to overwrite an existing one)
    #[structopt(name = "init")]
    Init {
        /// Overwrite an existing project
        #[structopt(long = "force")]
        force: bool,

        project_name: String,

        /// defaults to the current directory
        #[structopt(default_value = ".")]
        path: std::path::PathBuf,
    },
    /// Inspect a package
    #[structopt(name = "inspect")]
    Inspect {
        project_name: String,

        /// Package version [default: 'latest']
        version: String,
    },
}


fn main() -> CliResult {
    let cli = Cli::from_args();

    match &cli.command {
        Commands::Init {
            project_name,
            path,
            force,
        } => init_project(project_name, path.to_path_buf(), *force),
        Commands::Inspect {
            project_name,
            version,
        } => inspect_package(REGISTRY_HOST, project_name, version),
    }
}
