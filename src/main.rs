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

mod nodepm;
use nodepm::package::inspect_package;
use nodepm::project::init_project;
mod cli;
use cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;


/// <https://registry.npmjs.com>
const REGISTRY_HOST: &str = "https://registry.npmjs.com";




fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            project_name,
            path,
            force,
        } => init_project(&project_name, path.to_path_buf(), *force),
        Commands::Inspect {
            project_name,
            version,
        } => inspect_package(REGISTRY_HOST, &project_name, &version),
    }
}
