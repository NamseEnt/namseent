use clap::{Parser, Subcommand};

mod commands;
mod services;

#[derive(Parser)]
#[command(name = "cli")]
#[command(about = "Rust WASM development CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start development server
    Start,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            let current_dir = std::env::current_dir()?;
            commands::start::start(&current_dir).await?;
        }
    }

    Ok(())
}
