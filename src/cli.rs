use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser)]
#[structopt(name = "nodepm")]
pub struct Cli {
    // Subcommands
    #[structopt(subcommand)]
    pub command: Commands,

    #[structopt(flatten)]
    verbose: Verbosity
}

#[derive(Debug, Parser)]
pub enum Commands {
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