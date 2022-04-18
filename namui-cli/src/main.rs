mod cli;
mod procedures;
mod services;
mod types;
mod util;
use clap::StructOpt;
use cli::{Cli, Commands};
use std::env::current_dir;
use util::{print_namui_cfg, set_namui_user_config};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let manifest_path = current_dir()
        .expect("No current dir found")
        .join("Cargo.toml");

    let result = match &cli.command {
        Commands::Test {
            target,
            manifest_path: option_manifest_path,
        } => {
            let manifest_path = option_manifest_path.as_ref().unwrap_or(&manifest_path);
            procedures::test(target, &manifest_path)
        }
        Commands::Target { target } => set_namui_user_config(target),
        Commands::Print { printable_object } => match printable_object {
            cli::PrintableObject::Cfg => print_namui_cfg(),
        },
        Commands::Start {
            target,
            manifest_path: option_manifest_path,
        } => {
            let manifest_path = option_manifest_path.as_ref().unwrap_or(&manifest_path);
            procedures::start(target, &manifest_path)
        }
        Commands::Build {
            target,
            manifest_path: option_manifest_path,
            arch,
        } => {
            let manifest_path = option_manifest_path.as_ref().unwrap_or(&manifest_path);
            procedures::build(target, &manifest_path, arch.into())
        }
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
