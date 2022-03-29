mod cli;
mod procedures;
mod services;
mod types;
mod util;
use clap::StructOpt;
use cli::{Cli, Commands};
use procedures::{dev_wasm_electron, dev_wasm_web, release_wasm_electron, release_wasm_web};
use std::env::current_dir;

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
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
