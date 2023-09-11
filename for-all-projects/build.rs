use clap::CommandFactory;
use clap_complete::{generate_to, shells::Bash};
use std::fs::create_dir_all;
use std::io::Error;
use std::path::PathBuf;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    if !cfg!(linux) {
        return Ok(());
    }

    let Some(home) = std::env::var_os("HOME") else {
        return Ok(());
    };
    let outdir = PathBuf::from(home).join(".local/share/bash-completion/completions");
    let bin_name = "for-all-projects";

    create_dir_all(&outdir)?;

    let mut cmd = Cli::command();
    generate_to(Bash, &mut cmd, bin_name, &outdir)?;

    Ok(())
}
