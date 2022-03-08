mod procedures;
mod services;
mod util;
use procedures::dev_wasm_web;
use std::env::current_dir;
mod types;

#[tokio::main]
async fn main() {
    let manifest_path = current_dir()
        .expect("No current dir found")
        .join("Cargo.toml");

    let result = dev_wasm_web(&manifest_path);

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
