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

#[derive(Parser)]
pub struct Project {
         #[structopt(subcommand)]
        pub project_commands: ProjectCommands,
}

#[derive(Parser)]
pub struct Package {
         #[structopt(subcommand)]
        pub package_commands: PackageCommands,
}

#[derive(Args)]
pub struct InspectArgs {
    pub project_name: String,

    /// Package version [default: 'latest']
    pub version: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Project commands
    #[clap(name = "project")]
    Project(Project),
    /// Package commands
    #[clap(name = "package")]
    Package(Package),
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Initialize a new project (use "init --force" to overwrite an existing one)
    Init(ProjectInitArgs)
}

#[derive(Subcommand)]
pub enum PackageCommands {
    /// Inspect a package
    Inspect(PackageInspectArgs)
}

#[derive(Args)]
pub struct ProjectInitArgs {
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
pub struct PackageInspectArgs {
    pub project_name: String,

    /// Package version [default: 'latest']
    pub version: String,
}