mod cli;
mod find_cargo_project_dirs;
mod run_commands;

use anyhow::Result;
use clap::Parser;
use cli::*;
use find_cargo_project_dirs::*;
use run_commands::*;
use std::env::current_dir;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let cargo_project_dirs = find_cargo_project_dirs(current_dir().unwrap()).await?;
    println!("cargo projects: {:#?}", cargo_project_dirs);

    run_commands_in_parallel(cli, cargo_project_dirs).await?;

    Ok(())
}
