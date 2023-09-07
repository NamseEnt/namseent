use clap::Parser;

#[derive(Parser, Clone, Copy)]
#[command(version, name = "for-all-projects")]
/// You can set CARGO_TARGET_DIR to speed up the process.
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
    /// Runs `cargo fmt --allow-dirty --allow-staged`
    pub fmt: bool,

    #[arg(long)]
    /// Runs `cargo clippy --fix --allow-dirty --allow-staged`
    pub clippy: bool,
}
