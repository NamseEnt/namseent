use clap::{Args, Parser};

#[derive(Parser, Clone)]
#[command(version, name = "for-all-projects")]
/// You can set CARGO_TARGET_DIR to speed up the process.
/// Order of command is preserved, please check the source code.
pub enum Cli {
    Run(Run),
    List,
}

#[derive(Parser, Clone)]
pub struct Run {
    #[arg(long)]
    /// Cargo project directory to run commands. If set, it will ignore all other Cargo project directories.
    pub only: Option<String>,

    #[command(flatten)]
    pub command: Command,
}

#[derive(Args, Clone)]
pub struct Command {
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
    /// Runs `cargo clippy`
    pub clippy: bool,

    #[arg(long)]
    /// Runs `cargo clippy --fix --allow-dirty --allow-staged`
    /// `cargo clippy --fix` won't return exit code 1 even with '-D warnings'.
    /// https://github.com/rust-lang/rust-clippy/issues/1209
    pub clippy_fix: bool,

    #[arg(long)]
    /// Runs custom test script, `namui test` if it is a namui project, or `cargo test`.
    /// To run custom test script, you need to set `test = "YOUR_CUSTOM_SCRIPT"` in [package.metadata] table of Cargo.toml.
    /// For namui project, you need to set `namui = true` in [package.metadata] table of Cargo.toml.
    pub test: bool,

    #[arg(long)]
    pub custom: Option<String>,
}
