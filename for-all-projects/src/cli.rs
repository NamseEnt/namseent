use clap::Parser;

#[derive(Parser, Clone, Copy)]
#[command(version, name = "for-all-projects")]
pub struct Cli {
    #[arg(long)]
    pub check: bool,

    #[arg(long)]
    pub clean: bool,

    #[arg(long)]
    pub metadata: bool,

    #[arg(long)]
    pub update: bool,

    #[arg(long)]
    pub clippy: bool,

    #[arg(long)]
    pub fmt: bool,
}
