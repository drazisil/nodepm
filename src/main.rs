//! Test?

mod nodepm;
use nodepm::package::inspect_package;
use nodepm::project::init_project;
use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// <https://registry.npmjs.com>
const REGISTRY_HOST: &str = "https://registry.npmjs.com";

#[derive(Parser)]
#[structopt(name = "nodepm")]
struct Cli {
    // Subcommands
    #[structopt(subcommand)]
    command: Commands,

    #[structopt(flatten)]
    verbose: Verbosity
}

#[derive(Debug, Parser)]
enum Commands {
    /// Initialize a new project (use "init --force" to overwrite an existing one)
    #[structopt(name = "init")]
    Init {
        /// Overwrite an existing project
        #[structopt(long = "force")]
        force: bool,

        /// The name to add to the package.json file
        project_name: String,

        /// The `path` to create a the project in. Defaults to the current directory (`.`)
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


fn main() -> Result<()> {
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
        } => inspect_package(REGISTRY_HOST, project_name, version),
    }
}
