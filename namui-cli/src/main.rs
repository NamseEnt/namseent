mod build;
mod procedures;
mod services;
mod util;
use clap::{App, Arg};
use procedures::dev_wasm_web;
use std::{env::current_dir, path::PathBuf};

#[tokio::main]
async fn main() {
    let matches = App::new("Namui")
        .arg(
            Arg::with_name("manifest_path")
                .help("Target directory to run 'cargo build'. Mostly, root dir of crate")
                .index(1)
                .required(false),
        )
        .get_matches();

    let manifest_path = matches
        .value_of("manifest_path")
        .map(|manifest_path| PathBuf::from(manifest_path))
        .unwrap_or(
            current_dir()
                .expect("No current dir found")
                .join("Cargo.toml"),
        );

    let result = dev_wasm_web(&manifest_path);

    match result {
        Ok(_) => todo!(),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}
