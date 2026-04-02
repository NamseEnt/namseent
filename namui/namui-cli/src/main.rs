#![allow(dead_code)]
#![allow(unused_variables)]
// temporary allow dead code for cross platform development. it will be removed when the project is stable.

mod cli;
mod start;
mod types;
mod util;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, Target};
use std::env::current_dir;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let project_path = current_dir().expect("No current dir found");

    match cli.command {
        Commands::Start { target, .. } => {
            let target = target.unwrap_or(Target::Wasm32WasiWeb);
            match target {
                Target::Wasm32WasiWeb => start::start_web(&project_path)?,
                Target::Aarch64AppleDarwin => start::mac::start(&project_path)?,
                other => {
                    eprintln!("Target {other} is not yet supported");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Only 'start' command is currently supported");
            std::process::exit(1);
        }
    }

    Ok(())
}
