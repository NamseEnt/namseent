use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::ThreadPoolBuilder;
use std::path::PathBuf;
use std::sync::Arc;

use td_simulator::config::GameConfig;
use td_simulator::environment::{AgentAction, LegalAction, Observation};
use td_simulator::hp_balance::{self, BalanceOptions};
use td_simulator::ml::MlContract;
use td_simulator::ml::cli::{self, Command as MlCommand};
use td_simulator::ml::features::observation_features;
use td_simulator::ml::model::{
    DeepSetsActorCritic, InferenceBackend, PolicyDevice, default_policy_device,
};
use td_simulator::ml::neural_checkpoint::NeuralCheckpoint;
use td_simulator::ml::rollout::logits_for;
use td_simulator::ml::seed::SeedRange;
use td_simulator::ml::validation::{
    BaselinePolicy, evaluate_baseline, evaluate_checkpoint, paired_baseline_report,
};
use td_simulator::policy_runner::{BatchResult, PolicyRunnerConfig, run_batch};
use td_simulator::recording::{SimRecorder, SimulationProvenance};
use td_simulator::stats::Database;

mod stats_cli;

#[derive(Parser)]
#[command(name = "td-simulator", about = "Tower Defense environment simulator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
enum Command {
    Simulate(SimulateOptions),
    Baseline(BaselineOptions),
    Balance(BalanceOptions),
    #[command(about = "Interactive SQLite statistics explorer for td-simulator")]
    Stats(stats_cli::StatsOptions),
    Ml {
        #[command(subcommand)]
        command: MlCommand,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum BaselinePolicyArg {
    RandomLegal,
    Checkpoint,
}

#[derive(Args)]
struct BaselineOptions {
    #[arg(long, value_enum, default_value_t = BaselinePolicyArg::Checkpoint)]
    policy: BaselinePolicyArg,
    #[arg(long, default_value_t = 0)]
    seed_start: u64,
    #[arg(long, default_value_t = 3)]
    seed_end: u64,
    #[arg(long, default_value_t = 10_000)]
    max_decisions: usize,
    #[arg(long, default_value = "ml_policy_checkpoint.json")]
    checkpoint: PathBuf,
    #[arg(long)]
    config: Option<PathBuf>,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    pair_with: Option<BaselinePolicyArg>,
}

#[derive(Args)]
struct SimulateOptions {
    #[arg(short, long, default_value_t = 1000)]
    samples: usize,
    #[arg(short, long, default_value = "sim_results.db")]
    db: PathBuf,
    #[arg(long)]
    fresh_db: bool,
    #[arg(short, long, default_value_t = 0)]
    threads: usize,
    #[arg(long)]
    config: Option<PathBuf>,
    #[arg(long, default_value = "ml_policy_checkpoint.json")]
    checkpoint: PathBuf,
    #[arg(long)]
    allow_checkpoint_config_change: bool,
    #[arg(long, default_value_t = 10_000)]
    max_decisions: usize,
    #[arg(long)]
    strategy_stats: bool,
    #[arg(long)]
    clear_rate_graph: bool,
    #[arg(long)]
    all_stats: bool,
    #[arg(long)]
    quiet: bool,
    #[arg(long)]
    trace_steps: bool,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Simulate(options) => run_simulate(options),
        Command::Baseline(options) => run_baseline(options),
        Command::Balance(options) => hp_balance::run(options),
        Command::Stats(options) => stats_cli::run(options),
        Command::Ml { command } => cli::run_command(command),
    }
}

fn run_baseline(options: BaselineOptions) -> Result<()> {
    let config = Arc::new(match options.config {
        Some(ref path) => GameConfig::from_toml(path)
            .with_context(|| format!("failed to load config {}", path.display()))?,
        None => GameConfig::default_config(),
    });
    let seed_range = SeedRange::try_new(options.seed_start, options.seed_end)?;
    let pair_config = Arc::clone(&config);
    let report = match options.policy {
        BaselinePolicyArg::Checkpoint => {
            let checkpoint_path = &options.checkpoint;
            let contract = MlContract::from_config(config.as_ref());
            let (_checkpoint, model) =
                NeuralCheckpoint::load_with_inference_model_with_config_change(
                    checkpoint_path,
                    &contract,
                    false,
                )?;
            evaluate_checkpoint(
                Arc::new(model),
                Arc::new(default_policy_device()),
                config,
                seed_range,
                options.max_decisions,
            )?
        }
        BaselinePolicyArg::RandomLegal => evaluate_baseline(
            config,
            seed_range,
            match options.policy {
                BaselinePolicyArg::RandomLegal => BaselinePolicy::RandomLegal,
                BaselinePolicyArg::Checkpoint => unreachable!(),
            },
            options.max_decisions,
        )?,
    };
    let json = if let Some(pair_policy) = options.pair_with {
        let paired = match pair_policy {
            BaselinePolicyArg::RandomLegal => evaluate_baseline(
                pair_config,
                seed_range,
                BaselinePolicy::RandomLegal,
                options.max_decisions,
            )?,
            BaselinePolicyArg::Checkpoint => anyhow::bail!("--pair-with checkpoint is unsupported"),
        };
        serde_json::to_string_pretty(&paired_baseline_report(&report, &paired)?)?
    } else {
        serde_json::to_string_pretty(&report)?
    };
    if let Some(path) = options.output {
        std::fs::write(&path, format!("{json}\n"))?;
        println!("Baseline report saved to: {}", path.display());
    } else {
        println!("{json}");
    }
    Ok(())
}

fn run_simulate(options: SimulateOptions) -> Result<()> {
    let config = Arc::new(match options.config {
        Some(ref path) => GameConfig::from_toml(path)
            .with_context(|| format!("failed to load config {}", path.display()))?,
        None => GameConfig::default_config(),
    });
    let seeds = (0..options.samples as u64).collect::<Vec<_>>();
    let runner_config = PolicyRunnerConfig {
        max_decisions_per_episode: options.max_decisions,
        record_steps: options.trace_steps,
        max_stage: None,
        reward_config: td_simulator::environment::RewardConfig::default(),
    };

    if options.trace_steps && (options.samples != 1 || options.threads != 1) {
        anyhow::bail!("--trace-steps requires --samples 1 --threads 1");
    }

    if options.fresh_db && options.db.exists() {
        std::fs::remove_file(&options.db)?;
    }
    let recorder = SimRecorder::new(&options.db)?;
    let pool = {
        let builder = ThreadPoolBuilder::new().thread_name(|index| format!("sim-{index}"));
        let builder = if options.threads == 0 {
            builder
        } else {
            builder.num_threads(options.threads)
        };
        builder.build()?
    };

    if !options.quiet {
        println!("Running {} simulations...", seeds.len());
        println!(
            "Policy: neural checkpoint ({})",
            options.checkpoint.display()
        );
    }
    let progress = (!options.quiet).then(|| {
        let progress = ProgressBar::new(seeds.len() as u64);
        progress.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}",
            )
            .expect("static progress template should be valid"),
        );
        progress
    });

    let contract = MlContract::from_config(config.as_ref());
    let mut provenance = SimulationProvenance {
        runner_kind: "policy_runner".to_string(),
        policy_kind: "neural_checkpoint".to_string(),
        checkpoint_path: Some(options.checkpoint.display().to_string()),
        checkpoint_iteration: None,
        environment_version: Some(contract.environment_version),
        action_schema_version: Some(contract.action_schema_version),
        config_digest: Some(contract.config_digest.clone()),
        config_override: false,
        seed_schedule: Some(seed_schedule(&seeds)),
    };
    let result = {
        let checkpoint_path = &options.checkpoint;
        provenance.config_override = options.allow_checkpoint_config_change;
        let (checkpoint, model) = NeuralCheckpoint::load_with_inference_model_with_config_change(
            checkpoint_path,
            &contract,
            options.allow_checkpoint_config_change,
        )?;
        provenance.checkpoint_iteration = Some(checkpoint.iteration);
        let model = Arc::new(model);
        let device = Arc::new(default_policy_device());
        pool.install(|| {
            run_batch::<_, _>(Arc::clone(&config), &seeds, &runner_config, {
                let model = Arc::clone(&model);
                let device = Arc::clone(&device);
                move |_seed| {
                    let model = Arc::clone(&model);
                    let device = Arc::clone(&device);
                    move |observation: &Observation, legal_actions: &[LegalAction]| {
                        choose_model_action(
                            model.as_ref(),
                            device.as_ref(),
                            observation,
                            legal_actions,
                        )
                    }
                }
            })
        })
        .with_context(|| {
            format!(
                "failed to simulate checkpoint {} (iteration {})",
                checkpoint_path.display(),
                checkpoint.iteration
            )
        })?
    };

    record_results(&recorder, &result, &provenance)?;
    if options.trace_steps {
        print_step_trace(&result)?;
    }
    if let Some(progress) = progress {
        progress.finish_and_clear();
    }
    print_summary(&result, &options);

    if options.strategy_stats || options.all_stats {
        let database = Database::open(&options.db)?;
        for row in database.list_strategy_win_rates()? {
            println!(
                "{}: {} {:.1}% ({}/{})",
                row.category,
                row.name,
                row.win_rate * 100.0,
                row.win_count,
                row.sample_count
            );
        }
    }
    println!("Results saved to: {}", options.db.display());
    Ok(())
}

fn print_step_trace(result: &BatchResult) -> Result<()> {
    let episode = result
        .episodes
        .first()
        .context("step trace requires one episode")?;
    let steps = episode
        .steps
        .as_ref()
        .context("step trace was not recorded")?;
    for (index, step) in steps.iter().enumerate() {
        let selected_index = step
            .legal_actions
            .iter()
            .position(|legal| legal.action == step.action)
            .context("selected action is absent from legal actions")?;
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "event": "policy_step",
                "step": index,
                "observation": {
                    "decision_point": step.observation.decision_point,
                    "stage": step.observation.stage,
                    "sim_tick": step.observation.sim_tick,
                    "hp_raw": step.observation.hp_raw,
                    "gold": step.observation.gold,
                    "left_dice": step.observation.left_dice,
                    "selected_hand_slot_indices": step.observation.selected_hand_slot_indices,
                    "card_selection_confirmable": step.observation.card_selection_confirmable,
                    "hand_count": step.observation.hand.len(),
                    "tower_count": step.observation.towers.len(),
                    "active_monster_count": step.observation.active_monster_count,
                    "queued_monster_count": step.observation.queued_monster_count,
                },
                "legal_actions": step.legal_actions,
                "selected_index": selected_index,
                "selected_action": step.action,
                "outcome": {
                    "reason": step.outcome.info.reason,
                    "ticks_advanced": step.outcome.info.ticks_advanced,
                    "reward": step.outcome.reward,
                    "terminated": step.outcome.terminated,
                    "truncated": step.outcome.truncated,
                    "next_decision_point": step.outcome.observation.decision_point,
                    "next_stage": step.outcome.observation.stage,
                    "next_sim_tick": step.outcome.observation.sim_tick,
                    "state_hash": step.outcome.state_hash,
                }
            }))?
        );
    }
    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({
            "event": "episode_end",
            "seed": episode.seed,
            "decision_count": episode.decision_count,
            "terminated": episode.terminated,
            "truncated": episode.truncated,
            "termination_reason": episode.termination_reason,
            "victory": episode.victory,
            "clear_rate": episode.clear_rate,
            "final_stage": episode.final_observation.stage,
            "final_state_hash": episode.final_state_hash,
        }))?
    );
    Ok(())
}

fn choose_model_action(
    model: &DeepSetsActorCritic<InferenceBackend>,
    device: &PolicyDevice,
    observation: &Observation,
    legal_actions: &[LegalAction],
) -> Result<AgentAction> {
    let state = observation_features(observation);
    let candidate_rows = legal_actions
        .iter()
        .map(|legal| {
            td_simulator::ml::encoding::candidate_entity_rows(observation, &legal.action)
                .pop()
                .expect("candidate entity row must exist")
        })
        .collect::<Vec<_>>();
    let logits = logits_for(model, device, observation, &state, &candidate_rows);
    let action_index = logits
        .iter()
        .enumerate()
        .max_by(|left, right| left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0)))
        .map(|(index, _)| index)
        .context("checkpoint produced no action logits")?;
    Ok(legal_actions[action_index].action.clone())
}

fn seed_schedule(seeds: &[u64]) -> String {
    match (seeds.first(), seeds.last()) {
        (Some(first), Some(last)) => format!("{first}..={last}"),
        _ => "empty".to_string(),
    }
}

fn record_results(
    recorder: &SimRecorder,
    result: &BatchResult,
    provenance: &SimulationProvenance,
) -> Result<()> {
    for episode in &result.episodes {
        let sim_id = format!("sim_{:016x}", episode.seed);
        recorder.record_simulation_start_with_provenance(
            &sim_id,
            "environment",
            "environment",
            "environment",
            "environment",
            "environment",
            episode.seed,
            provenance,
        )?;
        recorder.record_simulation_end(
            &sim_id,
            episode.victory,
            episode.final_observation.stage,
            episode.clear_rate,
            episode.final_observation.hp_raw as f32 / 1000.0,
            episode.final_observation.gold,
            episode.metrics.total_towers_placed,
            episode.metrics.total_items_used,
            episode.metrics.total_player_damage,
            episode.metrics.total_gold_earned,
        )?;
    }
    Ok(())
}

fn print_summary(result: &BatchResult, options: &SimulateOptions) {
    let victories = result
        .episodes
        .iter()
        .filter(|episode| episode.victory)
        .count();
    println!("=== Simulation Complete ===");
    println!("Samples: {}", result.episodes.len());
    println!(
        "Win rate: {:.1}% ({}/{})",
        victories as f64 / result.episodes.len().max(1) as f64 * 100.0,
        victories,
        result.episodes.len()
    );
    if options.clear_rate_graph || options.all_stats {
        let clear_rates = result
            .episodes
            .iter()
            .map(|episode| episode.clear_rate)
            .collect::<Vec<_>>();
        print_clear_rate_histogram(&clear_rates);
    }
}

fn print_clear_rate_histogram(clear_rates: &[f32]) {
    let mut bins = vec![0usize; 51];
    for &rate in clear_rates {
        let index = ((rate.clamp(0.0, 100.0) as usize) / 2).min(50);
        bins[index] += 1;
    }
    let max_count = bins.iter().copied().max().unwrap_or(1).max(1);
    for (index, count) in bins.into_iter().enumerate() {
        let bar_length = (count * 20 + max_count / 2) / max_count;
        println!(
            "{:02} | {:<20} {}",
            index + 1,
            "#".repeat(bar_length),
            count
        );
    }
}
