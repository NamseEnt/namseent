use clap::Parser;

#[derive(Parser, Clone, Copy)]
#[command(version, name = "for-all-projects")]
/// You can set CARGO_TARGET_DIR to speed up the process.
/// Order of command is preserved, please check the source code.
pub struct Cli {
    #[arg(long)]
    pub clean: bool,

    #[arg(long)]
    pub update: bool,

    #[arg(long)]
    pub metadata: bool,

    #[arg(long)]
    pub check: bool,

    #[arg(long)]
    pub fmt: bool,

    #[arg(long)]
    /// Runs `cargo fix --allow-dirty --allow-staged`
    pub fix: bool,

    #[arg(long)]
    /// Runs `cargo clippy --fix --allow-dirty --allow-staged`
    pub clippy: bool,

    #[arg(long)]
    /// Runs `cargo test` or `namui test` if it is a namui project.
    /// for namui project, you need to set `namui = true` in [package] table of Cargo.toml.
    pub test: bool,
}
