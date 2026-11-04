use super::managed::{self, TrainOptions};
use super::model::policy_backend_description;
use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Command {
    /// Resume the active policy, or start a new one when none exists.
    ///
    /// Numeric configuration changes and core-rule source changes warm-start
    /// a new run from the active policy. Structural changes train from scratch.
    Train {
        /// Game configuration to train on (default: the built-in configuration).
        #[arg(long)]
        config: Option<PathBuf>,
        /// PPO iterations to run in this invocation.
        #[arg(long, default_value_t = 20)]
        iterations: usize,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
    /// Show the active policy and the recorded training runs.
    Status,
}

pub fn configure_threads(threads: usize) -> Result<()> {
    if threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .map_err(|error| anyhow::anyhow!("failed to configure rayon: {error}"))?;
    }
    Ok(())
}

pub fn run_command(command: Command) -> Result<()> {
    println!("ML inference backend: {}", policy_backend_description());
    match command {
        Command::Train {
            config,
            iterations,
            threads,
        } => {
            configure_threads(threads)?;
            managed::train(TrainOptions { config, iterations })
        }
        Command::Status => managed::status(),
    }
}
