mod build;

use clap::{App, Arg};
use futures::{self, executor::block_on};

// use cargo::core::compiler::;
#[tokio::main]
async fn main() {
    let matches = App::new("Namui")
        .arg(
            Arg::with_name("manifest_path")
                .help("Target Cargo.toml file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("watch")
                .help("Build in watch mode")
                .required(false)
                .long("watch")
                .short("w")
                .takes_value(false),
        )
        .get_matches();

    let manifest_path = matches
        .value_of("manifest_path")
        .unwrap()
        .to_string();
    let watch = matches.occurrences_of("watch") != 0;
    block_on(build::build(manifest_path, watch));
}
