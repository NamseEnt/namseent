use clap::CommandFactory;
use clap_complete::{generate_to, shells::Bash};
use std::env;
use std::fs::create_dir_all;
use std::io::Error;
use std::path::PathBuf;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("CARGO_MANIFEST_DIR") {
        None => return Ok(()),
        Some(outdir) => PathBuf::from(outdir).join("target").join("completions"),
    };

    create_dir_all(&outdir)?;

    let mut cmd = Cli::command();
    generate_to(Bash, &mut cmd, "for-all-projects", outdir)?;

    Ok(())
}
