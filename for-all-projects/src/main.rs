mod cli;
mod find_cargo_project_dirs;
mod run_commands;

use anyhow::Result;
use clap::Parser;
use cli::*;
use find_cargo_project_dirs::*;
use run_commands::*;
use std::{env::current_dir, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await
}

/// For macro fmt issue
async fn real_main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Run(run) => {
            let cargo_project_dirs = if let Some(only) = &run.only {
                vec![PathBuf::from(only)]
            } else {
                find_cargo_project_dirs(current_dir().unwrap()).await?
            };

            run_commands_in_parallel(run, cargo_project_dirs).await?;
        }
        Cli::List => {
            let cargo_project_dirs = find_cargo_project_dirs(current_dir().unwrap()).await?;

            println!(
                "{}",
                cargo_project_dirs
                    .into_iter()
                    .map(|x| x.to_str().unwrap().to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }

    Ok(())
}
