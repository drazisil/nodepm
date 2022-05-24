use clap::{ Args, Parser, Subcommand };
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

#[derive(Args)]
pub struct InitArgs {
        /// Overwrite an existing project
        #[clap(long = "force")]
        pub force: bool,

        /// The name to add to the package.json file
        pub project_name: String,

        /// The `path` to create a the project in. Defaults to the current directory (`.`)
        #[clap(default_value = ".")]
        pub path: std::path::PathBuf,

}

#[derive(Args)]
pub struct InspectArgs {
    pub project_name: String,

    /// Package version [default: 'latest']
    pub version: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project (use "init --force" to overwrite an existing one)
    #[clap(name = "init")]
    Init(InitArgs),
    /// Inspect a package
    #[clap(name = "inspect")]
    Inspect(InspectArgs),
}