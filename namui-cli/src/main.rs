mod cli;
mod procedures;
mod services;
mod types;
mod util;
use clap::StructOpt;
use cli::{Cli, Commands};
use procedures::{dev_wasm_electron, dev_wasm_web, release_wasm_electron, release_wasm_web};
use std::env::current_dir;
use util::{print_namui_cfg, set_namui_user_config};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let manifest_path = current_dir()
        .expect("No current dir found")
        .join("Cargo.toml");

    let result = match &cli.command {
        Commands::DevWasmWeb {} => dev_wasm_web(&manifest_path),
        Commands::DevWasmElectron {} => dev_wasm_electron(&manifest_path),
        Commands::ReleaseWasmWeb {} => release_wasm_web(&manifest_path),
        Commands::ReleaseWasmElectron { platform, arch } => {
            release_wasm_electron(&manifest_path, platform.into(), arch.into())
        }
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
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
