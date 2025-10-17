#![allow(dead_code)]
#![allow(unused_variables)]
// temporary allow dead code for cross platform development. it will be removed when the project is stable.

mod cli;
mod procedures;
mod services;
mod start;
mod types;
mod util;

use anyhow::{Result, anyhow, bail};
use clap::Parser;
use cli::{Cli, Commands, StartOption};
use namui_user_config::set_user_config;
use std::env::current_dir;
use util::{get_current_target, print_namui_cfg, print_namui_target};

#[tokio::main]
async fn main() -> Result<()> {
    let project_path: std::path::PathBuf = current_dir().expect("No current dir found");
    start::start(&project_path)?;
    // let cli = Cli::parse();
    // let current_target = get_current_target()?;

    // match cli.command {
    //     Commands::Test {
    //         target: option_target,
    //         manifest_path: option_manifest_path,
    //     } => {
    //         let target = option_target.unwrap_or(current_target);
    //         let manifest_path = option_manifest_path.unwrap_or(manifest_path);
    //         procedures::test(target, manifest_path)?;
    //     }
    //     Commands::Target { target } => set_user_config(target.into())?,
    //     Commands::Print { printable_object } => match printable_object {
    //         cli::PrintableObject::Cfg => print_namui_cfg()?,
    //         cli::PrintableObject::Target => print_namui_target()?,
    //     },
    //     Commands::Start {
    //         target: option_target,
    //         manifest_path: option_manifest_path,
    //         release,
    //         host,
    //         strip_debug_info,
    //     } => {
    //         let target = option_target.unwrap_or(current_target);
    //         let manifest_path = option_manifest_path.unwrap_or(manifest_path);
    //         procedures::start(
    //             target,
    //             manifest_path,
    //             StartOption {
    //                 release,
    //                 host,
    //                 strip_debug_info,
    //             },
    //         )
    //         .await?;
    //     }
    //     Commands::Build {
    //         target: option_target,
    //         manifest_path: option_manifest_path,
    //         release,
    //     } => {
    //         let target = option_target.unwrap_or(current_target);
    //         let manifest_path = option_manifest_path.unwrap_or(manifest_path);
    //         procedures::build(target, manifest_path, release).await?;
    //     }
    //     Commands::Clippy {
    //         target: option_target,
    //         manifest_path: option_manifest_path,
    //     } => {
    //         let target = option_target.unwrap_or(current_target);
    //         let manifest_path = option_manifest_path.unwrap_or(manifest_path);
    //         procedures::clippy(target, manifest_path).await?;
    //     }
    //     Commands::Check {
    //         target: option_target,
    //         manifest_path: option_manifest_path,
    //     } => {
    //         let target = option_target.unwrap_or(current_target);
    //         let manifest_path = option_manifest_path.unwrap_or(manifest_path);
    //         procedures::check(target, manifest_path).await?;
    //     }
    // };

    Ok(())
}
