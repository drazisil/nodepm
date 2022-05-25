#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts,
    clippy::single_char_pattern
)]
#![forbid(unsafe_code)]

//! Test?

/// Core functionality
mod nodepm;
pub use nodepm::{create_project_manifest, query_package_reqistry};
/// Binary CLI
mod cli;
use anyhow::Result;
use cli::{Cli, Commands, PackageCommands, ProjectCommands};
extern crate clap;
use clap::Parser;

/// <https://registry.npmjs.com>
const REGISTRY_HOST: &str = "https://registry.npmjs.com";

#[doc(hidden)]
fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Project(args) => match &args.project_commands {
            ProjectCommands::Init(args) => {
                create_project_manifest(
                    &args.project_name,
                    args.path.clone(),
                    args.force,
                )
            }
        },
        Commands::Package(args) => match &args.package_commands {
            PackageCommands::Inspect(args) => {
                query_package_reqistry(REGISTRY_HOST, &args.project_name, &args.version)
            }
        },
    }
}
