mod cli;
mod find_cargo_project_dirs;

use anyhow::Result;
use clap::Parser;
use cli::*;
use find_cargo_project_dirs::*;
use std::{env::current_dir, path::PathBuf};
use tokio::process;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let cargo_project_dirs = find_cargo_project_dirs(current_dir().unwrap()).await?;

    run_commands_in_parallel(cli, cargo_project_dirs).await?;

    Ok(())
}

async fn run_commands_in_parallel(cli: Cli, cargo_project_dirs: Vec<PathBuf>) -> Result<()> {
    let mut join_set = tokio::task::JoinSet::new();

    for cargo_project_dir in cargo_project_dirs {
        join_set.spawn(async move { run_commands(cli, cargo_project_dir).await });
    }

    while let Some(result) = join_set.join_next().await {
        result??;
    }

    Ok(())
}

async fn run_commands(cli: Cli, cargo_project_dir: PathBuf) -> Result<()> {
    async fn run_cargo(cargo_args: &[&str], cargo_project_dir: &PathBuf) -> Result<()> {
        let mut child = process::Command::new("cargo")
            .args(cargo_args)
            .current_dir(cargo_project_dir)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        println!(
            "Running `cargo {}` in {:?}",
            cargo_args.join(" "),
            cargo_project_dir
        );

        child.wait().await?;

        println!(
            "Finished `cargo {}` in {:?}",
            cargo_args.join(" "),
            cargo_project_dir
        );

        Ok(())
    }

    macro_rules! run_command {
        (
            $(
                ($command:ident, $opts:literal)
            ),*
        ) => {
            $(
                if cli.$command {
                    let args = [stringify!($command)]
                        .into_iter()
                        .chain($opts.split(" "))
                        .collect::<Vec<_>>();
                    run_cargo(&args, &cargo_project_dir).await?;
                }
            )*
        };
    }

    run_command!(
        (clean, ""),
        (update, ""),
        (metadata, ""),
        (check, ""),
        (fmt, "--allow-dirty --allow-staged"),
        (clippy, "--fix --allow-dirty --allow-staged")
    );

    Ok(())
}
