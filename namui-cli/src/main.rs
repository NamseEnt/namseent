mod procedures;
mod services;
mod util;
use procedures::{dev_wasm_electron, dev_wasm_web};
use std::env::current_dir;
mod types;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    DevWasmWeb {},
    DevWasmElectron {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let manifest_path = current_dir()
        .expect("No current dir found")
        .join("Cargo.toml");

    let result = match &cli.command {
        Commands::DevWasmWeb {} => dev_wasm_web(&manifest_path),
        Commands::DevWasmElectron {} => dev_wasm_electron(&manifest_path),
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
