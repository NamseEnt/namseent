use super::phase4_dataset::{
    Phase4Split, SourcePolicy, collect_into_directory, load_episodes, merge_directories,
    reward_invariant_report,
};
use crate::config::GameConfig;
use crate::teacher_selection::TeacherSelectionPools;
use anyhow::{Result, bail};
use clap::Subcommand;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Subcommand)]
pub enum Phase4Command {
    /// Generate canonical or teacher episodes for a frozen split.
    Collect {
        #[arg(long, value_enum)]
        split: Phase4Split,
        #[arg(long, value_enum)]
        source: SourceArg,
        /// Use the first `count` seeds of the split (default: whole split).
        #[arg(long)]
        count: Option<usize>,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 0)]
        shard_index: usize,
        #[arg(long, default_value_t = 1)]
        shard_count: usize,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
    /// Merge dataset shard directories into one directory.
    Merge {
        #[arg(long, required = true, num_args = 1..)]
        inputs: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },
    /// Dataset statistics and the clear_rate progress-reward invariant.
    DatasetReport {
        #[arg(long)]
        dataset: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum SourceArg {
    Canonical,
    Teacher,
}

pub(crate) fn configure_threads(threads: usize) -> Result<()> {
    if threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .map_err(|error| anyhow::anyhow!("failed to configure rayon: {error}"))?;
    }
    Ok(())
}

pub(crate) fn write_json<T: Serialize>(path: Option<&Path>, value: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    match path {
        Some(path) => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, json)?;
            println!("wrote {}", path.display());
        }
        None => println!("{json}"),
    }
    Ok(())
}

#[derive(Serialize)]
struct DatasetReport {
    episodes: usize,
    decisions: usize,
    seeds: (u64, u64),
    mean_decisions_per_episode: f64,
    mean_terminal_clear_rate: f64,
    victories: usize,
    action_kind_counts: std::collections::BTreeMap<String, usize>,
    decision_point_counts: std::collections::BTreeMap<String, usize>,
    mean_candidates_per_decision: f64,
    max_candidates_per_decision: usize,
    reward_invariant: super::phase4_dataset::RewardInvariantReport,
}

pub fn run(command: Phase4Command) -> Result<()> {
    let config = Arc::new(GameConfig::default_config());
    match command {
        Phase4Command::Collect {
            split,
            source,
            count,
            output,
            shard_index,
            shard_count,
            threads,
        } => {
            configure_threads(threads)?;
            let (source, pools) = match source {
                SourceArg::Canonical => (SourcePolicy::Canonical, None),
                SourceArg::Teacher => {
                    if split != Phase4Split::TeacherTrain {
                        bail!("teacher labels may only be collected on the teacher_train split");
                    }
                    (
                        SourcePolicy::Teacher,
                        Some(TeacherSelectionPools::production()),
                    )
                }
            };
            if source == SourcePolicy::Canonical
                && !matches!(
                    split,
                    Phase4Split::CanonicalTrain | Phase4Split::CanonicalValidation
                )
            {
                bail!("canonical training data may only come from canonical_train/validation");
            }
            let seeds = split.seeds(count)?;
            let summary = collect_into_directory(
                config,
                &output,
                split,
                &seeds,
                source,
                (shard_index, shard_count),
                pools,
            )?;
            write_json(None, &summary)
        }
        Phase4Command::Merge { inputs, output } => {
            let merged = merge_directories(&inputs, &output)?;
            println!("merged {merged} episodes into {}", output.display());
            Ok(())
        }
        Phase4Command::DatasetReport { dataset, output } => {
            let episodes = load_episodes(&dataset, &config)?;
            if episodes.is_empty() {
                bail!("{} contains no episodes", dataset.display());
            }
            let mut action_kind_counts = std::collections::BTreeMap::new();
            let mut decision_point_counts = std::collections::BTreeMap::new();
            let mut candidates = 0usize;
            let mut max_candidates = 0usize;
            let mut decisions = 0usize;
            for sample in episodes.iter().flat_map(|episode| &episode.samples) {
                decisions += 1;
                *action_kind_counts
                    .entry(sample.action_kind.clone())
                    .or_insert(0) += 1;
                *decision_point_counts
                    .entry(format!("{:?}", sample.decision_point))
                    .or_insert(0) += 1;
                candidates += sample.candidates.len();
                max_candidates = max_candidates.max(sample.candidates.len());
            }
            let report = DatasetReport {
                episodes: episodes.len(),
                decisions,
                seeds: (
                    episodes.first().map_or(0, |episode| episode.game_seed),
                    episodes.last().map_or(0, |episode| episode.game_seed),
                ),
                mean_decisions_per_episode: decisions as f64 / episodes.len() as f64,
                mean_terminal_clear_rate: episodes
                    .iter()
                    .map(|episode| episode.final_terminal_clear_rate as f64)
                    .sum::<f64>()
                    / episodes.len() as f64,
                victories: episodes.iter().filter(|episode| episode.victory).count(),
                action_kind_counts,
                decision_point_counts,
                mean_candidates_per_decision: candidates as f64 / decisions.max(1) as f64,
                max_candidates_per_decision: max_candidates,
                reward_invariant: reward_invariant_report(&episodes),
            };
            write_json(output.as_deref(), &report)
        }
    }
}
