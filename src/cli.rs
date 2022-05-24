use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

// Top level clap::Command
#[derive(Parser)]
#[structopt(name = "nodepm")]
pub struct Cli {
    // Subcommands
    // You can only have one subcommand section
    // so we point this to the Commands struct
    #[structopt(subcommand)]
    pub command: Commands,

    #[structopt(flatten)]
    verbose: Verbosity,
}

// Second tier of commands -  clap::Subcommand
// Normally this would point to a clap::Args object,
// but in this case we are passing in another clap::Subcommand
// This enum defines our subcommand groups and passes on down
#[derive(Subcommand)]
pub enum Commands {
    /// Project commands
    #[clap(name = "project")]
    Project(Project),
    /// Package commands
    #[clap(name = "package")]
    Package(Package),
}

// The 'project' command itself
// The only way this differs from the Commands struct
// is that it directes clap::Parser, instead of clap::Subcommand
// This is so it gains the ability to auto-parse into
// the structure tree as a clap::Command with args, instead of as
// a clap::Args object
#[derive(Parser)]
pub struct Project {
    #[structopt(subcommand)]
    pub project_commands: ProjectCommands,
}

// The 'package' command itself
// The only way this differs from the Commands struct
// is that it directes clap::Parser, instead of clap::Subcommand
// This is so it gains the ability to auto-parse into
// the structure tree as a clap::Command with args, instead of as
// a clap::Args object
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

// The command enum for the 'project' command
// Here we pass in the args for the 'project init' command
#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Initialize a new project (use "init --force" to overwrite an existing one)
    Init(ProjectInitArgs),
}

// The command enum for the 'package' command
// Here we pass in the args for the 'package inspect' command
#[derive(Subcommand)]
pub enum PackageCommands {
    /// Inspect a package
    Inspect(PackageInspectArgs),
}

// The final clap::Args struct for the project init command
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
