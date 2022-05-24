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
        Commands::Init(args) => {
            create_project_manifest(&args.project_name, args.path.to_path_buf(), args.force) 
        },
        Commands::Inspect(args)  => {
            query_package_reqistry(REGISTRY_HOST, &args.project_name, &args.version) 
        },
    }
}
