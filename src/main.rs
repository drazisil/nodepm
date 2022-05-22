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
pub use nodepm::{query_package_reqistry, create_project_manifest};
/// Binary CLI
mod cli;
use cli::{Cli, Commands};
use anyhow::Result;
extern crate clap;
use clap::Parser;


/// <https://registry.npmjs.com>
const REGISTRY_HOST: &str = "https://registry.npmjs.com";



#[doc(hidden)]
fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            project_name,
            path,
            force,
        } => create_project_manifest(&project_name, path.to_path_buf(), *force),
        Commands::Inspect {
            project_name,
            version,
        } => query_package_reqistry(REGISTRY_HOST, &project_name, &version),
    }
}
