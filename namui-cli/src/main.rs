mod build;

use clap::{App, Arg};
use futures::{self, executor::block_on};

// use cargo::core::compiler::;
#[tokio::main]
async fn main() {
    let matches = App::new("Namui")
        .arg(
            Arg::with_name("target_dir")
                .help("Target directory to run 'cargo build'. Mostly, root dir of crate")
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

    let target_dir = matches
        .value_of("target_dir")
        .unwrap()
        .to_string();
    let watch = matches.occurrences_of("watch") != 0;
    block_on(build::build(target_dir, watch));
}
