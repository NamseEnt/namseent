use anyhow::{Context, Result, bail};
#[cfg(feature = "simulator-wgpu")]
use clap::ValueEnum;
use clap::{Parser, Subcommand};
use rayon::ThreadPoolBuilder;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "simulator-wgpu")]
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use super::MlContract;
use super::bc::{BcConfig, evaluate_bc, train_bc, train_bc_from_jsonl, train_bc_from_model};
use super::dataset::{
    collect_behavior_dataset, collect_item_expert_behavior_dataset,
    collect_monte_carlo_expert_behavior_dataset, collect_scripted_expert_behavior_dataset,
    collect_spiral_expert_behavior_dataset, collect_strict_expert_dataset, write_jsonl,
};
#[cfg(feature = "simulator-wgpu")]
use super::model::gpu_policy_backend_description;
#[cfg(feature = "simulator-wgpu")]
use super::model::inference_model;
use super::model::{
    ENTITY_ENCODER_SCHEMA_VERSION, ModelConfig, default_policy_device, policy_backend_description,
};
use super::neural_checkpoint::{
    NEURAL_CHECKPOINT_SCHEMA_VERSION, NeuralCheckpoint, POLICY_SCHEMA_VERSION, current_git_revision,
};
#[cfg(feature = "simulator-wgpu")]
use super::ppo::{
    MINIMUM_WGPU_VALIDATION_CLEAR_RATE, train_ppo_wgpu_once, trainer_hyperparameters,
};
use super::ppo::{
    OverfitGateThreshold, PpoConfig, evaluate_overfit_gate, train_ppo_from_model_with_progress,
    train_ppo_with_progress,
};
use super::rollout::{RolloutConfig, collect_diagnostic_trace};
use super::seed::{SeedRange, TrainingSeedSchedule};
use super::training_progress::PpoProgress;
use super::validation::evaluate_clear_rate;
use super::validation::{EvaluationProvenance, sha256_hex};
use crate::config::GameConfig;
use crate::events::SimEvent;
use crate::policy_runner::{ScriptedOracleTrace, run_scripted_oracle_with_stage_limit};

pub fn parse_no_progress_cycle_penalty(value: &str) -> Result<f32, String> {
    let value = value
        .parse::<f32>()
        .map_err(|error| format!("invalid cycle penalty {value:?}: {error}"))?;
    if !value.is_finite() {
        return Err(format!("cycle penalty must be finite, got {value}"));
    }
    if value > 0.0 {
        return Err(format!("cycle penalty must be non-positive, got {value}"));
    }
    Ok(value)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ResolvedRunManifest {
    schema_version: u32,
    run_id: String,
    parent_run_id: Option<String>,
    mode: String,
    reward_config: crate::environment::RewardConfig,
    run_directory: Option<String>,
    checkpoint_path: String,
    input_artifact_path: Option<String>,
    input_artifact_sha256: Option<String>,
    git_revision: String,
    contract: MlContract,
    seed_schedule: TrainingSeedSchedule,
}

fn artifact_sha256(path: &std::path::Path) -> Result<String> {
    Ok(sha256_hex(&std::fs::read(path).with_context(|| {
        format!("failed to read input artifact {}", path.display())
    })?))
}

fn emit_training_startup(
    mode: &'static str,
    reward_config: &crate::environment::RewardConfig,
    run_dir: Option<&std::path::Path>,
    checkpoint_path: &std::path::Path,
    input_artifact: Option<&std::path::Path>,
    contract: &MlContract,
    schedule: &TrainingSeedSchedule,
) -> Result<()> {
    reward_config.validate().map_err(anyhow::Error::msg)?;
    let input_artifact_sha256 = input_artifact.map(artifact_sha256).transpose()?;
    let run_id = run_dir
        .and_then(std::path::Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unmanaged".to_string());
    let manifest = ResolvedRunManifest {
        schema_version: 1,
        run_id: run_id.clone(),
        parent_run_id: input_artifact.and_then(|path| {
            path.parent()
                .and_then(std::path::Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        }),
        mode: mode.to_string(),
        reward_config: reward_config.clone(),
        run_directory: run_dir.map(|path| path.display().to_string()),
        checkpoint_path: checkpoint_path.display().to_string(),
        input_artifact_path: input_artifact.map(|path| path.display().to_string()),
        input_artifact_sha256,
        git_revision: current_git_revision()?,
        contract: contract.clone(),
        seed_schedule: schedule.clone(),
    };
    if let Some(run_dir) = run_dir {
        std::fs::create_dir_all(run_dir)?;
        std::fs::write(
            run_dir.join("resolved-manifest.json"),
            format!("{}\n", serde_json::to_string_pretty(&manifest)?),
        )?;
    }
    Ok(())
}

#[derive(Parser)]
#[command(
    name = "td-simulator",
    about = "Fixed-seed PPO policy training and validation"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum Command {
    CollectExpert {
        #[arg(long, default_value = "expert_dataset.jsonl")]
        output: PathBuf,
        #[arg(long, default_value_t = 0)]
        seed_start: u64,
        #[arg(long, default_value_t = 3)]
        seed_end: u64,
        #[arg(long, default_value_t = 4_096)]
        max_decisions: usize,
        #[arg(long, default_value_t = 0)]
        threads: usize,
        #[arg(long)]
        include_truncated: bool,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        strict: bool,
        #[arg(long)]
        scripted_expert_behavior: bool,
        #[arg(long)]
        spiral_expert: bool,
        #[arg(long)]
        monte_carlo_expert: bool,
        #[arg(long)]
        item_expert: bool,
        #[arg(long)]
        all_experts: bool,
    },
    Pretrain {
        #[arg(long)]
        dataset: PathBuf,
        #[arg(long, default_value = "bc_checkpoint.json")]
        checkpoint: PathBuf,
        #[arg(long, default_value_t = 10)]
        epochs: usize,
        #[arg(long, default_value_t = 0.001)]
        learning_rate: f64,
        #[arg(long, default_value_t = 64)]
        batch_size: usize,
        #[arg(long, default_value_t = 0)]
        threads: usize,
        #[arg(long)]
        config: Option<PathBuf>,
    },
    EvaluateBc {
        #[arg(long)]
        checkpoint: PathBuf,
        #[arg(long)]
        dataset: PathBuf,
        #[arg(long)]
        config: Option<PathBuf>,
    },
    Train {
        #[cfg(feature = "simulator-wgpu")]
        #[arg(long, value_enum, default_value_t = LearnerArg::Cpu)]
        learner: LearnerArg,
        #[cfg(feature = "simulator-wgpu")]
        #[arg(long, default_value_t = false)]
        wgpu_stream_rollout: bool,
        #[arg(short, long, default_value = "ml_policy_checkpoint.json")]
        checkpoint: PathBuf,
        #[arg(long, conflicts_with_all = ["train_size"])]
        train_start: Option<u64>,
        #[arg(long, conflicts_with_all = ["train_size"])]
        train_end: Option<u64>,
        #[arg(long, conflicts_with_all = ["validation_size"])]
        validation_start: Option<u64>,
        #[arg(long, conflicts_with_all = ["validation_size"])]
        validation_end: Option<u64>,
        #[arg(long, conflicts_with_all = ["train_start", "train_end"], help = "number of training seeds [default: 1024]")]
        train_size: Option<u64>,
        #[arg(long, conflicts_with_all = ["validation_start", "validation_end"], help = "number of validation seeds [default: 256]")]
        validation_size: Option<u64>,
        #[arg(long)]
        iterations: Option<usize>,
        #[arg(long, default_value_t = 2)]
        ppo_epochs: usize,
        #[arg(long, default_value_t = 512)]
        max_decisions: usize,
        #[arg(long, default_value_t = 0.0001)]
        learning_rate: f64,
        #[arg(
            long,
            allow_hyphen_values = true,
            value_parser = parse_no_progress_cycle_penalty,
            default_value = "0.0"
        )]
        no_progress_cycle_penalty: f32,
        #[arg(long, default_value_t = 0.0)]
        damage_progress_weight: f32,
        #[arg(long, default_value_t = false)]
        adaptive_exploration: bool,
        #[arg(long, default_value_t = 64)]
        hidden_size: usize,
        #[arg(long, default_value_t = 8192)]
        minibatch_size: usize,
        #[arg(long, default_value_t = 0)]
        rollout_step_budget: usize,
        #[arg(long, default_value_t = 0)]
        max_queued_steps: usize,
        #[arg(long, default_value_t = 0)]
        rollout_chunk_size: usize,
        #[arg(long, default_value = "artifacts/ml/auto")]
        run_dir: Option<PathBuf>,
        #[arg(long, default_value_t = 0)]
        threads: usize,
        #[arg(long)]
        resume: Option<PathBuf>,
        #[arg(long, conflicts_with = "resume")]
        init_checkpoint: Option<PathBuf>,
        #[arg(long, conflicts_with = "resume", default_value_t = false)]
        resume_auto: bool,
        #[arg(long)]
        require_improvement: bool,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        curriculum_final_stage: Option<usize>,
        #[arg(long)]
        curriculum_validation_start: Option<u64>,
        #[arg(long)]
        curriculum_validation_end: Option<u64>,
    },
    Validate {
        #[arg(short, long, default_value = "ml_policy_checkpoint.json")]
        checkpoint: PathBuf,
        #[arg(long, default_value_t = 10_000)]
        max_decisions: usize,
        #[arg(long, default_value_t = 0)]
        threads: usize,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "run-1")]
        run_id: String,
    },
    DiagnosticTrace {
        #[arg(short, long, default_value = "ml_policy_checkpoint.json")]
        checkpoint: PathBuf,
        #[arg(long, default_value_t = u64::MAX)]
        seed: u64,
        #[arg(long, default_value_t = 10_000)]
        max_decisions: usize,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    OverfitGate {
        #[arg(short, long, default_value = "ml_policy_checkpoint.json")]
        checkpoint: PathBuf,
        #[arg(long, default_value_t = 1)]
        seed_count: u64,
        #[arg(long, default_value_t = 64)]
        max_decisions: usize,
        #[arg(long, default_value_t = 0.05)]
        minimum_return_improvement: f32,
        #[arg(long, default_value_t = 0.25)]
        maximum_supervised_nll: f32,
        #[arg(long, default_value_t = 1.0)]
        maximum_value_loss: f32,
        #[arg(long)]
        config: Option<PathBuf>,
    },
    PairedBaseline {
        #[arg(short, long, default_value = "ml_policy_checkpoint.json")]
        checkpoint: PathBuf,
        #[arg(long, default_value_t = 0)]
        seed_start: u64,
        #[arg(long, default_value_t = 3)]
        seed_end_inclusive: u64,
        #[arg(long, default_value_t = 256)]
        max_decisions: usize,
        #[arg(long, default_value = "paired_baseline.db")]
        database: PathBuf,
        #[arg(long, default_value = "run-1")]
        run_id: String,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[cfg(feature = "simulator-wgpu")]
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum LearnerArg {
    Cpu,
    Wgpu,
}

#[derive(Clone)]
struct TrainCommandOptions {
    #[cfg(feature = "simulator-wgpu")]
    learner: LearnerArg,
    #[cfg(feature = "simulator-wgpu")]
    wgpu_stream_rollout: bool,
    checkpoint: PathBuf,
    train_start: Option<u64>,
    train_end: Option<u64>,
    validation_start: Option<u64>,
    validation_end: Option<u64>,
    train_size: Option<u64>,
    validation_size: Option<u64>,
    iterations: Option<usize>,
    ppo_epochs: usize,
    max_decisions: usize,
    learning_rate: f64,
    no_progress_cycle_penalty: f32,
    damage_progress_weight: f32,
    adaptive_exploration: bool,
    hidden_size: usize,
    minibatch_size: usize,
    rollout_step_budget: usize,
    max_queued_steps: usize,
    rollout_chunk_size: usize,
    run_dir: Option<PathBuf>,
    threads: usize,
    resume: Option<PathBuf>,
    init_checkpoint: Option<PathBuf>,
    resume_auto: bool,
    require_improvement: bool,
    config: Option<PathBuf>,
    curriculum_final_stage: Option<usize>,
    curriculum_validation_start: Option<u64>,
    curriculum_validation_end: Option<u64>,
}

pub fn run() -> Result<()> {
    run_command(Cli::parse().command)
}

pub fn run_command(command: Command) -> Result<()> {
    println!("ML inference backend: {}", policy_backend_description());
    match command {
        Command::CollectExpert {
            output,
            seed_start,
            seed_end,
            max_decisions,
            threads,
            include_truncated,
            config,
            strict,
            scripted_expert_behavior,
            spiral_expert,
            monte_carlo_expert,
            item_expert,
            all_experts,
        } => collect_expert_command(
            output,
            seed_start,
            seed_end,
            max_decisions,
            threads,
            include_truncated,
            config,
            strict,
            scripted_expert_behavior,
            spiral_expert,
            monte_carlo_expert,
            item_expert,
            all_experts,
        ),
        Command::Pretrain {
            dataset,
            checkpoint,
            epochs,
            learning_rate,
            batch_size,
            threads,
            config,
        } => pretrain_command(
            dataset,
            checkpoint,
            epochs,
            learning_rate,
            batch_size,
            threads,
            config,
        ),
        Command::EvaluateBc {
            checkpoint,
            dataset,
            config,
        } => evaluate_bc_command(checkpoint, dataset, config),
        Command::Train {
            #[cfg(feature = "simulator-wgpu")]
            learner,
            #[cfg(feature = "simulator-wgpu")]
            wgpu_stream_rollout,
            checkpoint,
            train_start,
            train_end,
            validation_start,
            validation_end,
            train_size,
            validation_size,
            iterations,
            ppo_epochs,
            max_decisions,
            learning_rate,
            no_progress_cycle_penalty,
            damage_progress_weight,
            adaptive_exploration,
            hidden_size,
            minibatch_size,
            rollout_step_budget,
            max_queued_steps,
            rollout_chunk_size,
            run_dir,
            threads,
            resume,
            init_checkpoint,
            resume_auto,
            require_improvement,
            config,
            curriculum_final_stage,
            curriculum_validation_start,
            curriculum_validation_end,
        } => train_command(TrainCommandOptions {
            #[cfg(feature = "simulator-wgpu")]
            learner,
            #[cfg(feature = "simulator-wgpu")]
            wgpu_stream_rollout,
            checkpoint,
            train_start,
            train_end,
            validation_start,
            validation_end,
            train_size,
            validation_size,
            iterations,
            ppo_epochs,
            max_decisions,
            learning_rate,
            no_progress_cycle_penalty,
            damage_progress_weight,
            adaptive_exploration,
            hidden_size,
            threads,
            minibatch_size,
            rollout_step_budget,
            max_queued_steps,
            rollout_chunk_size,
            run_dir,
            resume,
            init_checkpoint,
            resume_auto,
            require_improvement,
            config,
            curriculum_final_stage,
            curriculum_validation_start,
            curriculum_validation_end,
        }),
        Command::Validate {
            checkpoint,
            max_decisions,
            threads,
            config,
            output,
            run_id,
        } => validate_command(checkpoint, max_decisions, threads, config, output, run_id),
        Command::DiagnosticTrace {
            checkpoint,
            seed,
            max_decisions,
            config,
            output,
        } => diagnostic_trace_command(checkpoint, seed, max_decisions, config, output),
        Command::OverfitGate {
            checkpoint,
            seed_count,
            max_decisions,
            minimum_return_improvement,
            maximum_supervised_nll,
            maximum_value_loss,
            config,
        } => overfit_gate_command(
            checkpoint,
            seed_count,
            max_decisions,
            OverfitGateThreshold {
                minimum_return_improvement,
                maximum_supervised_nll,
                maximum_value_loss,
            },
            config,
        ),
        Command::PairedBaseline {
            checkpoint,
            seed_start,
            seed_end_inclusive,
            max_decisions,
            database,
            run_id,
            config,
            output,
        } => paired_baseline_command(
            checkpoint,
            seed_start,
            seed_end_inclusive,
            max_decisions,
            database,
            run_id,
            config,
            output,
        ),
    }
}

fn evaluate_bc_command(
    checkpoint_path: PathBuf,
    dataset_path: PathBuf,
    config_path: Option<PathBuf>,
) -> Result<()> {
    let game_config = load_config(config_path)?;
    let contract = MlContract::from_config(&game_config);
    let (_checkpoint, model) =
        NeuralCheckpoint::load_for_initialization(&checkpoint_path, &contract)?;
    let dataset = super::dataset::read_jsonl(&dataset_path)?;
    let report = evaluate_bc(&model, &dataset, &game_config)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "sample_count": report.sample_count,
            "nll": report.nll,
            "top1_accuracy": report.top1_accuracy,
            "action_kind_buckets": report.action_kind_buckets,
        }))?
    );
    Ok(())
}

fn pretrain_command(
    dataset_path: PathBuf,
    checkpoint_path: PathBuf,
    epochs: usize,
    learning_rate: f64,
    batch_size: usize,
    threads: usize,
    config_path: Option<PathBuf>,
) -> Result<()> {
    run_with_threads(threads, || {
        let game_config = load_config(config_path)?;
        let dataset = super::dataset::read_jsonl(&dataset_path)?;
        let model_config = ModelConfig::default();
        let (model, report) = train_bc_from_jsonl(
            &dataset_path,
            &game_config,
            model_config,
            &BcConfig {
                epochs,
                learning_rate,
                batch_size,
                priority_action_kinds: true,
            },
        )?;
        let train = SeedRange::try_new(dataset.metadata.seed_start, dataset.metadata.seed_end)?;
        let validation = SeedRange::try_new(u64::MAX, u64::MAX)?;
        let mut hyperparameters = BTreeMap::new();
        hyperparameters.insert("bc_epochs".to_string(), epochs.to_string());
        hyperparameters.insert("bc_learning_rate".to_string(), learning_rate.to_string());
        hyperparameters.insert("bc_batch_size".to_string(), batch_size.to_string());
        let checkpoint = NeuralCheckpoint {
            checkpoint_schema_version: NEURAL_CHECKPOINT_SCHEMA_VERSION,
            policy_schema_version: POLICY_SCHEMA_VERSION,
            entity_encoder_schema_version: ENTITY_ENCODER_SCHEMA_VERSION,
            contract: MlContract::from_config(&game_config),
            seed_schedule: TrainingSeedSchedule::try_new(train, validation)?,
            iteration: 0,
            best_iteration: 0,
            git_revision: current_git_revision()?,
            model_config,
            model_file: String::new(),
            train_clear_rate: 0.0,
            validation_clear_rate: 0.0,
            best_validation_clear_rate: 0.0,
            best_validation_full_clear_count: 0,
            best_validation_truncated_count: 0,
            reward_config: crate::environment::RewardConfig::default(),
            hyperparameters,
        };
        checkpoint.save_with_model(&model, &checkpoint_path)?;
        println!(
            "BC pretrain complete: samples={} initial_nll={:.6} final_nll={:.6} top1={:.3} checkpoint={}",
            report.sample_count,
            report.initial_nll,
            report.final_nll,
            report.top1_accuracy,
            checkpoint_path.display()
        );
        Ok(())
    })
}

#[allow(clippy::too_many_arguments)]
fn collect_expert_command(
    output: PathBuf,
    seed_start: u64,
    seed_end: u64,
    max_decisions: usize,
    threads: usize,
    _include_truncated: bool,
    config_path: Option<PathBuf>,
    strict: bool,
    scripted_expert_behavior: bool,
    spiral_expert: bool,
    monte_carlo_expert: bool,
    item_expert: bool,
    all_experts: bool,
) -> Result<()> {
    if all_experts
        && (strict
            || scripted_expert_behavior
            || spiral_expert
            || monte_carlo_expert
            || item_expert)
    {
        bail!("--all-experts cannot be combined with a specific expert policy");
    }
    if [
        scripted_expert_behavior,
        spiral_expert,
        monte_carlo_expert,
        item_expert,
    ]
    .into_iter()
    .filter(|enabled| *enabled)
    .count()
        > 1
    {
        bail!("expert policy flags are mutually exclusive");
    }
    let config = Arc::new(load_config(config_path)?);
    let seed_range = SeedRange::try_new(seed_start, seed_end)?;
    if all_experts {
        let (left, right) = run_with_threads(threads, || {
            Ok(rayon::join(
                || {
                    rayon::join(
                        || {
                            collect_scripted_expert_behavior_dataset(
                                Arc::clone(&config),
                                seed_range,
                                max_decisions,
                            )
                        },
                        || {
                            collect_spiral_expert_behavior_dataset(
                                Arc::clone(&config),
                                seed_range,
                                max_decisions,
                            )
                        },
                    )
                },
                || {
                    rayon::join(
                        || {
                            collect_monte_carlo_expert_behavior_dataset(
                                Arc::clone(&config),
                                seed_range,
                                max_decisions,
                            )
                        },
                        || {
                            collect_item_expert_behavior_dataset(
                                Arc::clone(&config),
                                seed_range,
                                max_decisions,
                            )
                        },
                    )
                },
            ))
        })?;
        let datasets = [
            ("scripted", left.0?),
            ("spiral", left.1?),
            ("monte-carlo", right.0?),
            ("item", right.1?),
        ];
        let parent = output.parent().unwrap_or_else(|| std::path::Path::new("."));
        let stem = output
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("expert_dataset");
        for (name, dataset) in datasets {
            let path = parent.join(format!("{stem}-{name}.jsonl"));
            write_jsonl(&dataset, &path)?;
            println!(
                "Expert dataset saved: {} episodes, {} steps, {}",
                dataset.metadata.episode_count,
                dataset.metadata.step_count,
                path.display()
            );
        }
        return Ok(());
    }
    let dataset = run_with_threads(threads, || {
        if strict {
            collect_strict_expert_dataset(Arc::clone(&config), seed_range, max_decisions)
        } else if scripted_expert_behavior {
            collect_scripted_expert_behavior_dataset(Arc::clone(&config), seed_range, max_decisions)
        } else if spiral_expert {
            collect_spiral_expert_behavior_dataset(Arc::clone(&config), seed_range, max_decisions)
        } else if monte_carlo_expert {
            collect_monte_carlo_expert_behavior_dataset(
                Arc::clone(&config),
                seed_range,
                max_decisions,
            )
        } else if item_expert {
            collect_item_expert_behavior_dataset(Arc::clone(&config), seed_range, max_decisions)
        } else {
            collect_behavior_dataset(Arc::clone(&config), seed_range, max_decisions)
        }
    })?;
    write_jsonl(&dataset, &output)?;
    println!(
        "Expert dataset saved: {} episodes, {} steps, {}",
        dataset.metadata.episode_count,
        dataset.metadata.step_count,
        output.display()
    );
    Ok(())
}

fn train_combined_expert_bootstrap(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions: usize,
    model_config: ModelConfig,
    bc_config: &BcConfig,
) -> Result<(
    super::model::DeepSetsActorCritic<super::model::TrainBackend>,
    super::bc::BcReport,
)> {
    type Collector = fn(Arc<GameConfig>, SeedRange, usize) -> Result<super::dataset::ExpertDataset>;
    let collectors: [(&str, Collector); 4] = [
        ("scripted", collect_scripted_expert_behavior_dataset),
        ("spiral", collect_spiral_expert_behavior_dataset),
        ("monte_carlo", collect_monte_carlo_expert_behavior_dataset),
        ("item", collect_item_expert_behavior_dataset),
    ];
    let mut model = None;
    let mut total_samples = 0;
    let mut total_updates = 0;
    let mut initial_nll = None;
    let mut final_nll = 0.0;
    let mut top1_accuracy = 0.0;
    let mut epochs = 0;
    for (expert_name, collector) in collectors {
        let collect_started = Instant::now();
        let dataset = collector(Arc::clone(&config), seed_range, max_decisions)?;
        eprintln!(
            "expert.bootstrap expert={} phase=collect seconds={:.3} episodes={} steps={}",
            expert_name,
            collect_started.elapsed().as_secs_f64(),
            dataset.metadata.episode_count,
            dataset.metadata.step_count,
        );
        let bc_started = Instant::now();
        let (next_model, report) = match model.take() {
            Some(model) => train_bc_from_model(model, &dataset, config.as_ref(), bc_config)?,
            None => train_bc(&dataset, config.as_ref(), model_config, bc_config)?,
        };
        eprintln!(
            "expert.bootstrap expert={} phase=bc seconds={:.3} samples={} updates={}",
            expert_name,
            bc_started.elapsed().as_secs_f64(),
            report.sample_count,
            report.updates,
        );
        model = Some(next_model);
        total_samples += report.sample_count;
        total_updates += report.updates;
        initial_nll.get_or_insert(report.initial_nll);
        final_nll = report.final_nll;
        top1_accuracy = report.top1_accuracy;
        epochs += report.epochs;
    }
    let model = model.ok_or_else(|| anyhow::anyhow!("combined expert bootstrap has no model"))?;
    Ok((
        model,
        super::bc::BcReport {
            epochs,
            sample_count: total_samples,
            updates: total_updates,
            initial_nll: initial_nll.unwrap_or(0.0),
            final_nll,
            top1_accuracy,
        },
    ))
}

fn train_command(options: TrainCommandOptions) -> Result<()> {
    if options.iterations.is_some() {
        return train_command_once(options);
    }
    let mut options = options;
    eprintln!(
        "ML train auto-resume enabled: checkpoint={}",
        options.checkpoint.display()
    );
    if options.resume_auto && options.init_checkpoint.is_none() && options.checkpoint.exists() {
        options.init_checkpoint = Some(options.checkpoint.clone());
    }
    let mut retry_count = 0usize;
    loop {
        let mut iteration_options = options.clone();
        iteration_options.iterations = Some(1);
        iteration_options.resume_auto = true;
        match train_command_once(iteration_options) {
            Ok(()) => {
                retry_count = 0;
                #[cfg(feature = "simulator-wgpu")]
                if options.learner == LearnerArg::Wgpu {
                    write_auto_resume_manifest(&options)?;
                }
                options.resume = None;
                options.init_checkpoint = Some(options.checkpoint.clone());
            }
            Err(error) => {
                let message = error.to_string();
                if message.contains("does not match") || message.contains("unsupported") {
                    return Err(error);
                }
                retry_count += 1;
                if retry_count > 8 {
                    return Err(error).context("adaptive ml train retries exhausted");
                }
                options.learning_rate *= 0.5;
                options.ppo_epochs = options.ppo_epochs.max(1).saturating_sub(1).max(1);
                options.max_decisions = options.max_decisions.min(4096);
                options.resume_auto = true;
                options.resume = None;
                options.init_checkpoint = None;
                eprintln!(
                    "adaptive retry {retry_count}/8 after error: {error:#}; lr={:.3e} ppo_epochs={} minibatch_size={} max_decisions={}",
                    options.learning_rate,
                    options.ppo_epochs,
                    options.minibatch_size,
                    options.max_decisions,
                );
            }
        }
    }
}

#[cfg(feature = "simulator-wgpu")]
fn write_auto_resume_manifest(options: &TrainCommandOptions) -> Result<()> {
    let Some(run_dir) = &options.run_dir else {
        return Ok(());
    };
    fs::create_dir_all(run_dir)?;
    let manifest = serde_json::json!({
        "checkpoint": options.checkpoint.display().to_string(),
        "mode": "auto-resume",
    });
    fs::write(
        run_dir.join("latest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    Ok(())
}

#[cfg(feature = "simulator-wgpu")]
fn encode_elite_train_seeds(seeds: &[u64]) -> String {
    seeds
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(feature = "simulator-wgpu")]
fn decode_elite_train_seeds(value: Option<&String>) -> Vec<u64> {
    value
        .into_iter()
        .flat_map(|value| value.split(','))
        .filter_map(|seed| seed.parse().ok())
        .take(32)
        .collect()
}

fn train_command_once(options: TrainCommandOptions) -> Result<()> {
    let TrainCommandOptions {
        checkpoint: checkpoint_path,
        train_start,
        train_end,
        validation_start,
        validation_end,
        train_size,
        validation_size,
        iterations,
        ppo_epochs,
        max_decisions,
        learning_rate,
        no_progress_cycle_penalty,
        damage_progress_weight,
        adaptive_exploration,
        hidden_size,
        minibatch_size,
        rollout_step_budget,
        max_queued_steps,
        rollout_chunk_size,
        run_dir,
        threads,
        resume: mut resume_path,
        init_checkpoint,
        resume_auto,
        require_improvement,
        config: config_path,
        curriculum_final_stage,
        curriculum_validation_start,
        curriculum_validation_end,
        #[cfg(feature = "simulator-wgpu")]
        learner,
        #[cfg(feature = "simulator-wgpu")]
        wgpu_stream_rollout,
    } = options;
    #[cfg(feature = "simulator-wgpu")]
    println!(
        "ML learner backend: {}",
        match learner {
            LearnerArg::Cpu => policy_backend_description(),
            LearnerArg::Wgpu => gpu_policy_backend_description().to_string(),
        }
    );
    #[cfg(not(feature = "simulator-wgpu"))]
    println!("ML learner backend: {}", policy_backend_description());
    let iterations = iterations.context("iterations must be provided for finite training")?;
    let game_config = Arc::new(load_config(config_path)?);
    let schedule = TrainingSeedSchedule::try_new(
        resolve_seed_range(
            "train",
            train_size,
            train_start,
            train_end,
            SeedRange::try_new(0, 1023)?,
        )?,
        resolve_seed_range(
            "validation",
            validation_size,
            validation_start,
            validation_end,
            SeedRange::try_new(u64::MAX - 255, u64::MAX)?,
        )?,
    )?;
    let contract = MlContract::from_config(game_config.as_ref());
    let reward_config = crate::environment::RewardConfig {
        no_progress_cycle_penalty,
        damage_progress_weight,
        ..crate::environment::RewardConfig::default()
    };
    reward_config.validate().map_err(anyhow::Error::msg)?;
    let mut ppo_config = PpoConfig {
        iterations,
        ppo_epochs,
        learning_rate,
        minibatch_size,
        rollout_step_budget,
        max_queued_steps,
        rollout_chunk_size,
        trainer_checkpoint_dir: run_dir,
        resume_trainer: resume_auto,
        rollout: RolloutConfig {
            max_decisions_per_episode: max_decisions,
            adaptive_exploration,
            ..RolloutConfig::default()
        },
        model: ModelConfig { hidden_size },
        reward_config: reward_config.clone(),
        #[cfg(feature = "simulator-wgpu")]
        wgpu_stream_rollout,
        ..PpoConfig::default()
    };
    ppo_config.rollout.reward_config = reward_config;
    ppo_config.init_checkpoint = init_checkpoint;
    ppo_config.curriculum =
        curriculum_final_stage.map(|final_max_stage| super::curriculum::CurriculumConfig {
            final_max_stage,
            ..Default::default()
        });
    ppo_config.curriculum_validation =
        match (curriculum_validation_start, curriculum_validation_end) {
            (Some(start), Some(end)) => Some(SeedRange::try_new(start, end)?),
            (None, None) => None,
            _ => bail!("curriculum validation start and end must be provided together"),
        };
    let no_explicit_artifact = resume_path.is_none() && ppo_config.init_checkpoint.is_none();
    let trainer_latest_exists = no_explicit_artifact
        && ppo_config
            .trainer_checkpoint_dir
            .as_ref()
            .is_some_and(|directory| directory.join("latest.json").exists());
    #[cfg(feature = "simulator-wgpu")]
    let bootstrap_required =
        no_explicit_artifact && !trainer_latest_exists && !checkpoint_path.exists();
    #[cfg(feature = "simulator-wgpu")]
    if learner == LearnerArg::Wgpu {
        if wgpu_stream_rollout && rollout_chunk_size != 0 {
            bail!("--wgpu-stream-rollout cannot be combined with --rollout-chunk-size");
        }
        ppo_config.trainer_checkpoint_dir = None;
        ppo_config.resume_trainer = false;
        if resume_path.is_some() {
            bail!("WGPU learner does not support --resume; use --resume-auto");
        }
        let checkpoint_model = if no_explicit_artifact && checkpoint_path.exists() {
            match NeuralCheckpoint::load_with_inference_model(&checkpoint_path, &contract) {
                Ok((checkpoint, model))
                    if checkpoint.seed_schedule == schedule
                        && checkpoint.reward_config == ppo_config.reward_config
                        && checkpoint.model_config.hidden_size == hidden_size
                        && checkpoint.best_validation_clear_rate
                            >= MINIMUM_WGPU_VALIDATION_CLEAR_RATE =>
                {
                    Some((checkpoint, model))
                }
                Ok(_) => {
                    eprintln!(
                        "existing WGPU checkpoint is incompatible with the requested seed, reward, or model configuration; bootstrapping"
                    );
                    None
                }
                Err(error) => {
                    eprintln!(
                        "existing WGPU checkpoint cannot be resumed ({error:#}); bootstrapping"
                    );
                    None
                }
            }
        } else {
            None
        };
        let bootstrap_required = bootstrap_required
            || (no_explicit_artifact && checkpoint_path.exists() && checkpoint_model.is_none());
        let (
            cpu_model,
            initial_best_model,
            initial_best,
            initial_elite_train_seeds,
            starting_iteration,
        ) = if let Some(path) = &ppo_config.init_checkpoint {
            let (checkpoint, model) = NeuralCheckpoint::load_with_inference_model(path, &contract)?;
            (
                Some(model.clone()),
                Some(model),
                super::ppo::InitialBestState {
                    validation_clear_rate: checkpoint.best_validation_clear_rate,
                    iteration: checkpoint.best_iteration,
                    full_clear_count: checkpoint.best_validation_full_clear_count,
                    truncated_count: checkpoint.best_validation_truncated_count,
                },
                decode_elite_train_seeds(checkpoint.hyperparameters.get("elite_train_seeds")),
                checkpoint.iteration.saturating_add(1),
            )
        } else if let Some((checkpoint, model)) = checkpoint_model {
            (
                Some(model.clone()),
                Some(model),
                super::ppo::InitialBestState {
                    validation_clear_rate: checkpoint.best_validation_clear_rate,
                    iteration: checkpoint.best_iteration,
                    full_clear_count: checkpoint.best_validation_full_clear_count,
                    truncated_count: checkpoint.best_validation_truncated_count,
                },
                decode_elite_train_seeds(checkpoint.hyperparameters.get("elite_train_seeds")),
                checkpoint.iteration.saturating_add(1),
            )
        } else if bootstrap_required {
            let bootstrap_end = schedule
                .train
                .start
                .saturating_add(20)
                .min(schedule.train.end_inclusive);
            let (model, report) = train_combined_expert_bootstrap(
                Arc::clone(&game_config),
                SeedRange::try_new(schedule.train.start, bootstrap_end)?,
                max_decisions,
                ppo_config.model,
                &BcConfig {
                    epochs: 1,
                    learning_rate: 0.001,
                    batch_size: 64,
                    priority_action_kinds: true,
                },
            )?;
            eprintln!(
                "combined expert bootstrap complete: samples={} initial_nll={:.6} final_nll={:.6} top1={:.3}",
                report.sample_count, report.initial_nll, report.final_nll, report.top1_accuracy,
            );
            (
                Some(inference_model(&model)),
                None,
                super::ppo::InitialBestState {
                    validation_clear_rate: f64::NEG_INFINITY,
                    iteration: 0,
                    full_clear_count: 0,
                    truncated_count: usize::MAX,
                },
                Vec::new(),
                0,
            )
        } else {
            (
                None,
                None,
                super::ppo::InitialBestState {
                    validation_clear_rate: f64::NEG_INFINITY,
                    iteration: 0,
                    full_clear_count: 0,
                    truncated_count: usize::MAX,
                },
                Vec::new(),
                0,
            )
        };
        let progress = PpoProgress::new(ppo_config.iterations);
        let run = train_ppo_wgpu_once(
            Arc::clone(&game_config),
            &schedule,
            &ppo_config,
            cpu_model,
            initial_best_model,
            initial_best,
            initial_elite_train_seeds,
            starting_iteration,
            Some(&progress),
        )?;
        progress.finish();
        let last = run
            .history
            .last()
            .context("WGPU PPO produced no iterations")?;
        let mut hyperparameters = trainer_hyperparameters(&ppo_config);
        hyperparameters.insert(
            "elite_train_seeds".to_string(),
            encode_elite_train_seeds(&run.elite_train_seeds),
        );
        let checkpoint = NeuralCheckpoint {
            checkpoint_schema_version: NEURAL_CHECKPOINT_SCHEMA_VERSION,
            policy_schema_version: POLICY_SCHEMA_VERSION,
            entity_encoder_schema_version: ENTITY_ENCODER_SCHEMA_VERSION,
            contract,
            seed_schedule: schedule,
            iteration: last.iteration,
            best_iteration: run.best_iteration,
            git_revision: current_git_revision()?,
            model_config: ppo_config.model,
            model_file: String::new(),
            train_clear_rate: last.train_clear_rate,
            validation_clear_rate: last.validation_clear_rate,
            best_validation_clear_rate: run.best_validation_clear_rate,
            best_validation_full_clear_count: run.best_validation_full_clear_count,
            best_validation_truncated_count: run.best_validation_truncated_count,
            reward_config: ppo_config.reward_config.clone(),
            hyperparameters,
        };
        checkpoint.save_with_model(&run.best_model, &checkpoint_path)?;
        eprintln!("✓ WGPU checkpoint path={}", checkpoint_path.display());
        return Ok(());
    }
    if no_explicit_artifact && !trainer_latest_exists && checkpoint_path.exists() {
        match NeuralCheckpoint::load_metadata(&checkpoint_path, &contract, &schedule) {
            Ok(metadata)
                if metadata.reward_config == ppo_config.reward_config
                    && metadata.model_config.hidden_size == hidden_size =>
            {
                resume_path = Some(checkpoint_path.clone());
            }
            Ok(_) => eprintln!(
                "latest checkpoint is incompatible with requested reward/model config; bootstrapping from scripted behavior"
            ),
            Err(error) => eprintln!(
                "latest checkpoint cannot be resumed ({error:#}); bootstrapping from scripted behavior"
            ),
        }
    }
    let bootstrap_required =
        no_explicit_artifact && !trainer_latest_exists && resume_path.is_none();
    ppo_config.resume_trainer = trainer_latest_exists;
    let mode = if resume_path.is_some() || trainer_latest_exists {
        "resume"
    } else if ppo_config.init_checkpoint.is_some() {
        "init"
    } else if bootstrap_required {
        "bootstrap"
    } else {
        "fresh"
    };
    let input_artifact = resume_path
        .as_deref()
        .or(ppo_config.init_checkpoint.as_deref());
    emit_training_startup(
        mode,
        &ppo_config.reward_config,
        ppo_config.trainer_checkpoint_dir.as_deref(),
        &checkpoint_path,
        input_artifact,
        &contract,
        &schedule,
    )?;
    let progress = PpoProgress::new(iterations);
    let training = run_with_threads(threads, || {
        if let Some(resume_path) = resume_path {
            let metadata = NeuralCheckpoint::load_metadata(&resume_path, &contract, &schedule)?;
            if metadata.reward_config != ppo_config.reward_config {
                bail!("resume reward configuration does not match requested configuration");
            }
            if metadata.model_config.hidden_size != hidden_size {
                bail!("resume model hidden size does not match --hidden-size");
            }
            let (previous, model) =
                NeuralCheckpoint::load_with_model(&resume_path, &contract, &schedule)?;
            if previous.model_config.hidden_size != hidden_size {
                bail!("resume model hidden size does not match --hidden-size");
            }
            ppo_config.model = previous.model_config;
            let starting_iteration = previous.iteration.saturating_add(1);
            Ok((
                train_ppo_from_model_with_progress(
                    Arc::clone(&game_config),
                    &schedule,
                    &ppo_config,
                    model,
                    starting_iteration,
                    super::ppo::InitialBestState {
                        validation_clear_rate: previous.best_validation_clear_rate,
                        iteration: previous.best_iteration,
                        full_clear_count: previous.best_validation_full_clear_count,
                        truncated_count: previous.best_validation_truncated_count,
                    },
                    Some(&progress),
                )?,
                Some(super::ppo::best_score_from_checkpoint(&previous)),
            ))
        } else if bootstrap_required {
            let bootstrap_end = schedule
                .train
                .start
                .saturating_add(20)
                .min(schedule.train.end_inclusive);
            let (model, report) = train_combined_expert_bootstrap(
                Arc::clone(&game_config),
                SeedRange::try_new(schedule.train.start, bootstrap_end)?,
                max_decisions,
                ppo_config.model,
                &BcConfig {
                    epochs: 1,
                    learning_rate: 0.001,
                    batch_size: 64,
                    priority_action_kinds: true,
                },
            )?;
            eprintln!(
                "combined expert bootstrap complete: samples={} initial_nll={:.6} final_nll={:.6} top1={:.3}",
                report.sample_count, report.initial_nll, report.final_nll, report.top1_accuracy,
            );
            Ok((
                train_ppo_from_model_with_progress(
                    Arc::clone(&game_config),
                    &schedule,
                    &ppo_config,
                    model,
                    0,
                    super::ppo::InitialBestState {
                        validation_clear_rate: f64::NEG_INFINITY,
                        iteration: 0,
                        full_clear_count: 0,
                        truncated_count: usize::MAX,
                    },
                    Some(&progress),
                )?,
                None,
            ))
        } else {
            Ok((
                train_ppo_with_progress(
                    Arc::clone(&game_config),
                    &schedule,
                    &ppo_config,
                    Some(&progress),
                )?,
                None,
            ))
        }
    });
    progress.finish();
    let (run, previous_best) = training?;
    let run_score = super::ppo::best_score_from_run(&run);
    if require_improvement
        && previous_best.is_some_and(|previous| !run_score.is_better_than(previous))
    {
        bail!(
            "validation clear rate did not improve: previous {:.6}, new {:.6}",
            previous_best.map_or(0.0, |score| score.clear_rate),
            run.best_validation_clear_rate
        );
    }
    let last = run.history.last().context("PPO produced no iterations")?;
    let mut hyperparameters = BTreeMap::new();
    hyperparameters.insert("iterations".to_string(), iterations.to_string());
    hyperparameters.insert("ppo_epochs".to_string(), ppo_epochs.to_string());
    hyperparameters.insert("max_decisions".to_string(), max_decisions.to_string());
    hyperparameters.insert("learning_rate".to_string(), learning_rate.to_string());
    hyperparameters.insert("hidden_size".to_string(), hidden_size.to_string());
    hyperparameters.insert(
        "no_progress_cycle_penalty".to_string(),
        no_progress_cycle_penalty.to_string(),
    );
    hyperparameters.insert("minibatch_size".to_string(), minibatch_size.to_string());
    hyperparameters.insert(
        "rollout_step_budget".to_string(),
        rollout_step_budget.to_string(),
    );
    hyperparameters.insert("max_queued_steps".to_string(), max_queued_steps.to_string());
    hyperparameters.insert(
        "rollout_chunk_size".to_string(),
        rollout_chunk_size.to_string(),
    );
    let mut checkpoint = NeuralCheckpoint {
        checkpoint_schema_version: NEURAL_CHECKPOINT_SCHEMA_VERSION,
        policy_schema_version: POLICY_SCHEMA_VERSION,
        entity_encoder_schema_version: ENTITY_ENCODER_SCHEMA_VERSION,
        contract,
        seed_schedule: schedule,
        iteration: last.iteration,
        best_iteration: run.best_iteration,
        git_revision: current_git_revision()?,
        model_config: ppo_config.model,
        model_file: String::new(),
        train_clear_rate: last.train_clear_rate,
        validation_clear_rate: last.validation_clear_rate,
        best_validation_clear_rate: run.best_validation_clear_rate,
        best_validation_full_clear_count: run.best_validation_full_clear_count,
        best_validation_truncated_count: run.best_validation_truncated_count,
        reward_config: ppo_config.reward_config.clone(),
        hyperparameters,
    };
    checkpoint.model_file = checkpoint_path
        .with_extension("mpk")
        .file_name()
        .context("checkpoint path has no file name")?
        .to_string_lossy()
        .into_owned();
    let checkpoint_started = Instant::now();
    checkpoint.save_with_model(&run.best_model, &checkpoint_path)?;
    eprintln!(
        "ppo.timing phase=final_checkpoint seconds={:.3}",
        checkpoint_started.elapsed().as_secs_f64()
    );
    let promotion_started = Instant::now();
    promote_canonical_best(&run, &checkpoint)?;
    eprintln!(
        "ppo.timing phase=canonical_promotion seconds={:.3}",
        promotion_started.elapsed().as_secs_f64()
    );
    eprintln!("✓ checkpoint path={}", checkpoint_path.display());
    Ok(())
}

fn promote_canonical_best(
    run: &super::ppo::PpoTrainingRun,
    checkpoint: &NeuralCheckpoint,
) -> Result<()> {
    let canonical_path = PathBuf::from("ml_policy_checkpoint.json");
    let should_promote = match std::fs::read(&canonical_path) {
        Ok(bytes) => serde_json::from_slice::<NeuralCheckpoint>(&bytes)
            .map(|existing| {
                should_promote_canonical_best(
                    &existing,
                    checkpoint,
                    super::ppo::best_score_from_run(run),
                )
            })
            .unwrap_or(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
        Err(error) => return Err(error.into()),
    };
    if should_promote {
        checkpoint.save_with_model(&run.best_model, &canonical_path)?;
    }
    Ok(())
}

fn should_promote_canonical_best(
    existing: &NeuralCheckpoint,
    candidate: &NeuralCheckpoint,
    candidate_score: super::ppo::BestScore,
) -> bool {
    candidate.reward_config == existing.reward_config
        && candidate_score.is_better_than(super::ppo::best_score_from_checkpoint(existing))
}

fn validate_command(
    checkpoint_path: PathBuf,
    max_decisions: usize,
    threads: usize,
    config_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    run_id: String,
) -> Result<()> {
    let bytes = std::fs::read(&checkpoint_path)
        .with_context(|| format!("failed to read checkpoint {}", checkpoint_path.display()))?;
    let checkpoint: NeuralCheckpoint = serde_json::from_slice(&bytes)?;
    let game_config = Arc::new(load_config(config_path)?);
    let contract = MlContract::from_config(game_config.as_ref());
    let (checkpoint, model) =
        NeuralCheckpoint::load_with_model(&checkpoint_path, &contract, &checkpoint.seed_schedule)?;
    let device = default_policy_device();
    let progress = PpoProgress::new(1);
    progress.begin_iteration(0, 1);
    progress.begin_rollout(
        "validation",
        checkpoint.seed_schedule.validation.seeds().len(),
    );
    let validation = run_with_threads(threads, || {
        evaluate_clear_rate(
            &model,
            &device,
            game_config,
            checkpoint.seed_schedule.validation,
            &RolloutConfig {
                max_decisions_per_episode: max_decisions,
                greedy: true,
                reward_config: checkpoint.reward_config.clone(),
                ..RolloutConfig::default()
            },
            Some(progress.rollout_bar()),
        )
    });
    progress.finish_rollout();
    let report = validation?;
    progress.finish_validation(report.clear_rate);
    progress.finish();
    let checkpoint_sha256 = sha256_hex(&bytes);
    let provenance = EvaluationProvenance {
        run_id,
        git_revision: checkpoint.git_revision.clone(),
        config_digest: contract.config_digest.clone(),
        environment_version: contract.environment_version,
        action_schema_version: contract.action_schema_version,
        checkpoint_path: checkpoint_path.display().to_string(),
        checkpoint_sha256,
        seed_digest: checkpoint.seed_schedule.validation_digest.clone(),
        max_decisions,
        reward_config: checkpoint.reward_config.clone(),
    };
    let payload = serde_json::json!({ "provenance": provenance, "report": report });
    let json = serde_json::to_string_pretty(&payload)?;
    if let Some(output_path) = output_path {
        std::fs::write(&output_path, format!("{json}\n"))?;
    }
    println!("{json}");
    Ok(())
}

fn diagnostic_trace_command(
    checkpoint_path: PathBuf,
    seed: u64,
    max_decisions: usize,
    config_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
) -> Result<()> {
    let bytes = std::fs::read(&checkpoint_path)
        .with_context(|| format!("failed to read checkpoint {}", checkpoint_path.display()))?;
    let game_config = Arc::new(load_config(config_path)?);
    let contract = MlContract::from_config(game_config.as_ref());
    let (checkpoint, model) =
        NeuralCheckpoint::load_with_inference_model(&checkpoint_path, &contract)?;
    let scripted_trace =
        run_scripted_oracle_with_stage_limit(Arc::clone(&game_config), seed, None)?;
    let trace = collect_diagnostic_trace(
        Arc::new(model),
        Arc::new(default_policy_device()),
        Arc::clone(&game_config),
        seed,
        &RolloutConfig {
            max_decisions_per_episode: max_decisions,
            greedy: true,
            reward_config: checkpoint.reward_config.clone(),
            ..RolloutConfig::default()
        },
    )?;
    let first_divergence = compare_scripted_trace(&scripted_trace, &trace);
    let payload = serde_json::json!({
        "checkpoint_path": checkpoint_path,
        "checkpoint_sha256": sha256_hex(&bytes),
        "scripted_oracle": scripted_trace,
        "first_divergence": first_divergence,
        "trace": trace,
    });
    let json = serde_json::to_string_pretty(&payload)?;
    if let Some(output_path) = output_path {
        std::fs::write(output_path, format!("{json}\n"))?;
    }
    println!("{json}");
    Ok(())
}

fn compare_scripted_trace(
    scripted: &ScriptedOracleTrace,
    greedy: &super::rollout::DiagnosticTrace,
) -> Option<serde_json::Value> {
    for (index, (scripted_step, greedy_step)) in
        scripted.steps.iter().zip(&greedy.steps).enumerate()
    {
        let reason = if scripted_step.decision_point != greedy_step.decision_point {
            "decision_point"
        } else if scripted_step.legal_action_ids != greedy_step.legal_action_ids {
            "legal_action_ids"
        } else if scripted_step.selected_index != greedy_step.selected_index {
            "selected_index"
        } else if scripted_step.selected_action_id != greedy_step.selected_action_id {
            "selected_action_id"
        } else if scripted_step.pre_progress_fingerprint != greedy_step.pre_progress_fingerprint {
            "pre_progress_fingerprint"
        } else if scripted_step.state_hash != greedy_step.state_hash {
            "state_hash"
        } else {
            continue;
        };
        return Some(serde_json::json!({
            "decision_index": index,
            "reason": reason,
            "scripted_decision_point": scripted_step.decision_point,
            "greedy_decision_point": greedy_step.decision_point,
            "scripted_legal_action_ids": scripted_step.legal_action_ids,
            "greedy_legal_action_ids": greedy_step.legal_action_ids,
            "scripted_selected_index": scripted_step.selected_index,
            "greedy_selected_index": greedy_step.selected_index,
            "scripted_selected_action_id": scripted_step.selected_action_id,
            "greedy_selected_action_id": greedy_step.selected_action_id,
            "scripted_state_hash": scripted_step.state_hash,
            "greedy_state_hash": greedy_step.state_hash,
        }));
    }
    if scripted.steps.len() != greedy.steps.len() {
        return Some(serde_json::json!({
            "decision_index": scripted.steps.len().min(greedy.steps.len()),
            "reason": "decision_count",
            "scripted_decision_count": scripted.steps.len(),
            "greedy_decision_count": greedy.steps.len(),
        }));
    }
    None
}

#[cfg(test)]
mod diagnostic_trace_tests {
    use super::*;
    use crate::environment::{DecisionPoint, RewardComponents, StepReason};
    use crate::ml::rollout::DiagnosticTrace;
    use crate::policy_runner::ScriptedOracleStep;

    fn scripted_step(selected_index: usize, selected_action_id: &str) -> ScriptedOracleStep {
        ScriptedOracleStep {
            decision_point: DecisionPoint::Shop,
            legal_action_ids: vec![
                "purchase_shop_item:0".to_string(),
                "start_selecting_tower".to_string(),
            ],
            selected_index,
            selected_action_id: selected_action_id.to_string(),
            pre_progress_fingerprint: "pre".to_string(),
            post_progress_fingerprint: "post".to_string(),
            state_hash: "state".to_string(),
            reward: RewardComponents::default(),
        }
    }

    fn greedy_step(
        selected_index: usize,
        selected_action_id: &str,
    ) -> super::super::rollout::DiagnosticStep {
        super::super::rollout::DiagnosticStep {
            decision_point: DecisionPoint::Shop,
            legal_action_ids: vec![
                "purchase_shop_item:0".to_string(),
                "start_selecting_tower".to_string(),
            ],
            candidate_rows: Vec::new(),
            duplicate_candidate_row_groups: Vec::new(),
            logits: Vec::new(),
            selected_index,
            selected_action_id: selected_action_id.to_string(),
            pre_progress_fingerprint: "pre".to_string(),
            post_progress_fingerprint: "post".to_string(),
            state_hash: "state".to_string(),
            reward: RewardComponents::default(),
            terminated: false,
            truncated: false,
        }
    }

    #[test]
    fn compare_scripted_trace_reports_selection_divergence_after_matching_legal_order() {
        let scripted = ScriptedOracleTrace {
            seed: 1,
            steps: vec![scripted_step(1, "start_selecting_tower")],
            decision_count: 1,
            terminated: false,
            truncated: false,
            final_state_hash: "state".to_string(),
            final_stage: 1,
            total_towers_placed: 0,
            final_sim_tick: 0,
        };
        let greedy = DiagnosticTrace {
            seed: 1,
            steps: vec![greedy_step(0, "purchase_shop_item:0")],
            final_state_hash: "state".to_string(),
            final_stage: 1,
            episode_return: 0.0,
            termination_reason: StepReason::Terminal,
            first_repeated_state_hash: None,
            first_repeated_state_step: None,
            first_repeated_state_period: None,
            first_repeated_fingerprint_step: None,
        };

        let divergence = compare_scripted_trace(&scripted, &greedy).expect("divergence");

        assert_eq!(divergence["reason"], "selected_index");
        assert_eq!(divergence["scripted_selected_index"], 1);
        assert_eq!(divergence["greedy_selected_index"], 0);
        assert_eq!(
            divergence["scripted_legal_action_ids"],
            divergence["greedy_legal_action_ids"]
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn paired_baseline_command(
    checkpoint_path: PathBuf,
    seed_start: u64,
    seed_end_inclusive: u64,
    max_decisions: usize,
    database_path: PathBuf,
    run_id: String,
    config_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
) -> Result<()> {
    use super::validation::{
        BaselinePolicy, EvaluationProvenance, evaluate_baseline, paired_baseline_report,
    };
    use crate::recording::{SimRecorder, SimulationProvenance};

    let bytes = std::fs::read(&checkpoint_path)
        .with_context(|| format!("failed to read checkpoint {}", checkpoint_path.display()))?;
    let checkpoint: NeuralCheckpoint = serde_json::from_slice(&bytes)?;
    let game_config = Arc::new(load_config(config_path)?);
    let contract = MlContract::from_config(game_config.as_ref());
    NeuralCheckpoint::load_with_model(&checkpoint_path, &contract, &checkpoint.seed_schedule)?;
    let seed_range = SeedRange::try_new(seed_start, seed_end_inclusive)?;
    let left = evaluate_baseline(
        Arc::clone(&game_config),
        seed_range,
        BaselinePolicy::RandomLegal,
        max_decisions,
    )?;
    let right = evaluate_baseline(
        Arc::clone(&game_config),
        seed_range,
        BaselinePolicy::RandomLegal,
        max_decisions,
    )?;
    let paired = paired_baseline_report(&left, &right)?;
    let provenance = EvaluationProvenance {
        run_id,
        git_revision: checkpoint.git_revision.clone(),
        config_digest: contract.config_digest.clone(),
        environment_version: contract.environment_version,
        action_schema_version: contract.action_schema_version,
        checkpoint_path: checkpoint_path.display().to_string(),
        checkpoint_sha256: sha256_hex(&bytes),
        seed_digest: paired.seed_digest.clone(),
        max_decisions,
        reward_config: checkpoint.reward_config.clone(),
    };

    let recorder = SimRecorder::new(&database_path)?;
    for episode in &paired.episodes {
        let sim_id = format!("paired_{:016x}", episode.seed);
        recorder.record_simulation_start_with_provenance(
            &sim_id,
            "environment",
            "environment",
            "environment",
            "environment",
            "environment",
            episode.seed,
            &SimulationProvenance {
                runner_kind: "ml_cli".to_string(),
                policy_kind: "paired_baseline".to_string(),
                checkpoint_path: Some(provenance.checkpoint_path.clone()),
                checkpoint_iteration: Some(checkpoint.iteration),
                environment_version: Some(contract.environment_version),
                action_schema_version: Some(contract.action_schema_version),
                config_digest: Some(contract.config_digest.clone()),
                config_override: false,
                seed_schedule: Some(paired.seed_digest.clone()),
            },
        )?;
        recorder.record_simulation_end(
            &sim_id,
            episode.left_victory || episode.right_victory,
            0,
            (episode.left_clear_rate + episode.right_clear_rate) / 2.0,
            0.0,
            0,
            0,
            0,
            0.0,
            0,
        )?;
        recorder.record_events(
            &sim_id,
            &[SimEvent::GameEnd {
                final_stage: 0,
                victory: episode.left_victory,
                clear_rate: episode.left_clear_rate,
            }],
        )?;
    }

    let payload = serde_json::json!({ "provenance": provenance, "report": paired });
    let json = serde_json::to_string_pretty(&payload)?;
    if let Some(output_path) = output_path {
        std::fs::write(&output_path, format!("{json}\n"))?;
    }
    println!("{json}");
    Ok(())
}

fn overfit_gate_command(
    checkpoint_path: PathBuf,
    seed_count: u64,
    max_decisions: usize,
    threshold: OverfitGateThreshold,
    config_path: Option<PathBuf>,
) -> Result<()> {
    if seed_count != 1 && seed_count != 4 {
        bail!("overfit gate seed count must be exactly 1 or 4");
    }
    let bytes = std::fs::read(&checkpoint_path)?;
    let checkpoint: NeuralCheckpoint = serde_json::from_slice(&bytes)?;
    let game_config = Arc::new(load_config(config_path)?);
    let contract = MlContract::from_config(game_config.as_ref());
    let (checkpoint, model) =
        NeuralCheckpoint::load_with_model(&checkpoint_path, &contract, &checkpoint.seed_schedule)?;
    let device = default_policy_device();
    let seeds = (0..seed_count).collect::<Vec<_>>();
    let initial_model = super::model::DeepSetsActorCritic::<super::model::InferenceBackend>::new(
        checkpoint.model_config,
        &device,
    );
    let rollout_config = RolloutConfig {
        max_decisions_per_episode: max_decisions,
        greedy: true,
        reward_config: checkpoint.reward_config.clone(),
        ..RolloutConfig::default()
    };
    let initial = super::rollout::collect_rollouts(
        &initial_model,
        &device,
        Arc::clone(&game_config),
        &seeds,
        &rollout_config,
        None,
    )?;
    let final_batch = super::rollout::collect_rollouts(
        &model,
        &device,
        Arc::clone(&game_config),
        &seeds,
        &rollout_config,
        None,
    )?;
    let scripted_trajectory =
        crate::policy_runner::run_scripted_oracle_trajectory(Arc::clone(&game_config), 0)?;
    let initial_supervised =
        super::rollout::evaluate_supervised_nll(&initial_model, &device, &scripted_trajectory)?;
    let final_supervised =
        super::rollout::evaluate_supervised_nll(&model, &device, &scripted_trajectory)?;
    let report = evaluate_overfit_gate(
        &initial.diagnostics(),
        &final_batch.diagnostics(),
        seed_count as usize,
        max_decisions,
        threshold,
        &initial_supervised,
        &final_supervised,
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!(report))?
    );
    if report.passed {
        Ok(())
    } else {
        bail!("actual-environment overfit gate failed")
    }
}

fn run_with_threads<T: Send>(
    threads: usize,
    operation: impl FnOnce() -> Result<T> + Send,
) -> Result<T> {
    let mut builder = ThreadPoolBuilder::new().thread_name(|index| format!("ml-{index}"));
    if threads > 0 {
        builder = builder.num_threads(threads);
    }
    builder.build()?.install(operation)
}

fn load_config(path: Option<PathBuf>) -> Result<GameConfig> {
    match path {
        Some(path) => crate::config::load_jsonc(&path)
            .with_context(|| format!("failed to load config {}", path.display())),
        None => Ok(GameConfig::default_config()),
    }
}

fn resolve_seed_range(
    name: &str,
    size: Option<u64>,
    start: Option<u64>,
    end: Option<u64>,
    default: SeedRange,
) -> Result<SeedRange> {
    match (size, start, end) {
        (Some(0), None, None) => bail!("{name}-size must be positive"),
        (Some(size), None, None) if size > 0 => {
            let range = if name == "train" {
                SeedRange::try_new(0, size - 1)?
            } else {
                SeedRange::try_new(u64::MAX - size + 1, u64::MAX)?
            };
            Ok(range)
        }
        (Some(_), _, _) => bail!("{name}-size cannot be combined with an explicit seed range"),
        (None, Some(start), Some(end)) => Ok(SeedRange::try_new(start, end)?),
        (None, None, None) => Ok(default),
        (None, _, _) => bail!("{name}-start and {name}-end must be provided together"),
    }
}

#[cfg(test)]
mod cycle_penalty_tests {
    use super::*;

    #[test]
    fn parser_preserves_non_positive_values() {
        assert_eq!(parse_no_progress_cycle_penalty("-0.25"), Ok(-0.25));
        assert_eq!(parse_no_progress_cycle_penalty("0.0"), Ok(0.0));
    }

    #[test]
    fn parser_rejects_positive_and_non_finite_values() {
        for value in ["0.25", "NaN", "inf", "-inf"] {
            assert!(parse_no_progress_cycle_penalty(value).is_err(), "{value}");
        }
    }

    #[test]
    fn train_cli_supports_space_and_equals_forms() {
        let space = Cli::try_parse_from([
            "td-simulator",
            "train",
            "--iterations",
            "1",
            "--no-progress-cycle-penalty",
            "-0.25",
        ])
        .expect("space-separated cycle penalty should parse");
        let equals = Cli::try_parse_from([
            "td-simulator",
            "train",
            "--iterations",
            "1",
            "--no-progress-cycle-penalty=-0.25",
        ])
        .expect("equals-separated cycle penalty should parse");

        let Command::Train {
            no_progress_cycle_penalty: space_value,
            ..
        } = space.command
        else {
            panic!("expected train command");
        };
        let Command::Train {
            no_progress_cycle_penalty: equals_value,
            ..
        } = equals.command
        else {
            panic!("expected train command");
        };
        assert_eq!(space_value, -0.25);
        assert_eq!(equals_value, -0.25);
    }
}

#[cfg(test)]
mod run_manifest_tests {
    use super::*;

    fn temp_root(name: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "tower-defense-{name}-{}-{nanos}",
            std::process::id()
        ))
    }

    fn test_manifest(
        mode: &'static str,
        input_artifact: Option<&std::path::Path>,
    ) -> ResolvedRunManifest {
        let root = std::env::temp_dir().join(format!(
            "tower-defense-manifest-{}-{}",
            std::process::id(),
            mode
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create manifest test directory");
        let checkpoint_path = root.join("checkpoint.json");
        std::fs::write(&checkpoint_path, b"checkpoint").expect("write checkpoint fixture");
        if let Some(input_artifact) = input_artifact {
            std::fs::write(input_artifact, b"input").expect("write input fixture");
        }
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).expect("train range"),
            SeedRange::try_new(1, 1).expect("validation range"),
        )
        .expect("schedule");
        let contract = MlContract::from_config(&GameConfig::default_config());
        emit_training_startup(
            mode,
            &crate::environment::RewardConfig {
                no_progress_cycle_penalty: -0.25,
                ..Default::default()
            },
            Some(&root),
            &checkpoint_path,
            input_artifact,
            &contract,
            &schedule,
        )
        .expect("emit startup manifest");
        let manifest: ResolvedRunManifest = serde_json::from_slice(
            &std::fs::read(root.join("resolved-manifest.json")).expect("read manifest"),
        )
        .expect("decode manifest");
        let _ = std::fs::remove_dir_all(root);
        manifest
    }

    #[test]
    fn manifest_distinguishes_fresh_init_and_resume_inputs() {
        let fresh = test_manifest("fresh", None);
        assert_eq!(fresh.mode, "fresh");
        assert!(fresh.input_artifact_path.is_none());
        assert!(fresh.input_artifact_sha256.is_none());
        assert_eq!(fresh.reward_config.no_progress_cycle_penalty, -0.25);

        let init_root = std::env::temp_dir().join(format!(
            "tower-defense-manifest-input-{}-init",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&init_root);
        std::fs::create_dir_all(&init_root).expect("create init input directory");
        let init_input = init_root.join("init.json");
        let init = test_manifest("init", Some(&init_input));
        assert_eq!(init.mode, "init");
        assert_eq!(
            init.input_artifact_path.as_deref(),
            Some(init_input.to_str().unwrap())
        );
        assert!(init.input_artifact_sha256.is_some());

        let resume_input = init_root.join("resume.json");
        let resume = test_manifest("resume", Some(&resume_input));
        assert_eq!(resume.mode, "resume");
        assert_eq!(
            resume.input_artifact_path.as_deref(),
            Some(resume_input.to_str().unwrap())
        );
        assert!(resume.input_artifact_sha256.is_some());
        let _ = std::fs::remove_dir_all(init_root);
    }

    #[test]
    fn training_persists_the_effective_negative_cycle_penalty() {
        let root = temp_root("reward-artifacts");
        let run_dir = root.join("run");
        let checkpoint = root.join("policy.json");
        let result = train_command_once(TrainCommandOptions {
            #[cfg(feature = "simulator-wgpu")]
            learner: LearnerArg::Cpu,
            #[cfg(feature = "simulator-wgpu")]
            wgpu_stream_rollout: false,
            checkpoint: checkpoint.clone(),
            train_start: None,
            train_end: None,
            validation_start: None,
            validation_end: None,
            train_size: Some(1),
            validation_size: Some(1),
            iterations: Some(1),
            ppo_epochs: 1,
            max_decisions: 1,
            learning_rate: 0.0001,
            no_progress_cycle_penalty: -0.25,
            damage_progress_weight: 0.0,
            adaptive_exploration: false,
            hidden_size: 8,
            minibatch_size: 1,
            rollout_step_budget: 0,
            max_queued_steps: 0,
            rollout_chunk_size: 0,
            run_dir: Some(run_dir.clone()),
            threads: 1,
            resume: None,
            init_checkpoint: None,
            resume_auto: false,
            require_improvement: false,
            config: None,
            curriculum_final_stage: None,
            curriculum_validation_start: None,
            curriculum_validation_end: None,
        });
        result.expect("minimal training run should persist artifacts");

        let manifest: ResolvedRunManifest = serde_json::from_slice(
            &std::fs::read(run_dir.join("resolved-manifest.json")).expect("read manifest"),
        )
        .expect("decode manifest");
        assert_eq!(manifest.reward_config.no_progress_cycle_penalty, -0.25);

        let trainer: super::super::trainer_checkpoint::TrainerState = serde_json::from_slice(
            &std::fs::read(run_dir.join("latest.json")).expect("read trainer state"),
        )
        .expect("decode trainer state");
        assert_eq!(trainer.reward_config.no_progress_cycle_penalty, -0.25);

        let neural: NeuralCheckpoint =
            serde_json::from_slice(&std::fs::read(&checkpoint).expect("read neural checkpoint"))
                .expect("decode neural checkpoint");
        assert_eq!(neural.reward_config.no_progress_cycle_penalty, -0.25);
        assert!(run_dir.join("checkpoints").exists());
        assert!(
            run_dir
                .join("checkpoints")
                .join("iter-00000000-gen-00000000")
                .join("optimizer.bin")
                .exists()
        );

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn invalid_cycle_penalty_creates_no_training_artifacts() {
        let root = temp_root("invalid-reward");
        let run_dir = root.join("run");
        let result = train_command_once(TrainCommandOptions {
            #[cfg(feature = "simulator-wgpu")]
            learner: LearnerArg::Cpu,
            #[cfg(feature = "simulator-wgpu")]
            wgpu_stream_rollout: false,
            checkpoint: root.join("policy.json"),
            train_start: None,
            train_end: None,
            validation_start: None,
            validation_end: None,
            train_size: Some(1),
            validation_size: Some(1),
            iterations: Some(1),
            ppo_epochs: 1,
            max_decisions: 1,
            learning_rate: 0.0001,
            no_progress_cycle_penalty: 0.25,
            damage_progress_weight: 0.0,
            adaptive_exploration: false,
            hidden_size: 8,
            minibatch_size: 1,
            rollout_step_budget: 0,
            max_queued_steps: 0,
            rollout_chunk_size: 0,
            run_dir: Some(run_dir.clone()),
            threads: 1,
            resume: None,
            init_checkpoint: None,
            resume_auto: false,
            require_improvement: false,
            config: None,
            curriculum_final_stage: None,
            curriculum_validation_start: None,
            curriculum_validation_end: None,
        });
        assert!(result.is_err());
        assert!(!run_dir.exists());
        assert!(!root.join("policy.json").exists());
        assert!(!root.join("policy.mpk").exists());
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn canonical_best_promotion_rejects_a_different_reward_objective() {
        let current = crate::environment::RewardConfig {
            no_progress_cycle_penalty: -0.25,
            ..Default::default()
        };
        let existing = crate::environment::RewardConfig {
            no_progress_cycle_penalty: 0.0,
            ..Default::default()
        };
        let mut existing_checkpoint = test_checkpoint(existing);
        let candidate_checkpoint = test_checkpoint(current);
        existing_checkpoint.best_validation_clear_rate = 1.0;
        assert!(!should_promote_canonical_best(
            &existing_checkpoint,
            &candidate_checkpoint,
            super::super::ppo::BestScore {
                full_clear_count: 0,
                clear_rate: 1.0,
                truncated_count: 0,
            }
        ));
    }

    fn test_checkpoint(reward_config: crate::environment::RewardConfig) -> NeuralCheckpoint {
        NeuralCheckpoint {
            checkpoint_schema_version: NEURAL_CHECKPOINT_SCHEMA_VERSION,
            policy_schema_version: POLICY_SCHEMA_VERSION,
            entity_encoder_schema_version: super::super::model::ENTITY_ENCODER_SCHEMA_VERSION,
            contract: MlContract::from_config(&GameConfig::default_config()),
            seed_schedule: TrainingSeedSchedule::try_new(
                SeedRange::try_new(0, 0).unwrap(),
                SeedRange::try_new(1, 1).unwrap(),
            )
            .unwrap(),
            iteration: 0,
            best_iteration: 0,
            git_revision: "test".to_string(),
            model_config: ModelConfig::default(),
            model_file: "model.mpk".to_string(),
            train_clear_rate: 0.0,
            validation_clear_rate: 0.0,
            best_validation_clear_rate: 0.0,
            best_validation_full_clear_count: 0,
            best_validation_truncated_count: 0,
            reward_config,
            hyperparameters: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod paired_baseline_tests {
    use super::super::validation::{BaselinePolicy, evaluate_baseline, paired_baseline_report};
    use super::*;
    use crate::recording::{SimRecorder, SimulationProvenance};
    use std::sync::Arc;

    fn temp_db_path(name: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("tower_defense_{name}_{nanos}.db"))
    }

    #[test]
    fn paired_baseline_fixture_persists_rows_with_checkpoint_provenance() {
        let config = Arc::new(GameConfig::default_config());
        let range = SeedRange::try_new(0, 1).unwrap();
        let left =
            evaluate_baseline(Arc::clone(&config), range, BaselinePolicy::RandomLegal, 64).unwrap();
        let right = evaluate_baseline(config, range, BaselinePolicy::RandomLegal, 64).unwrap();
        let paired = paired_baseline_report(&left, &right).unwrap();

        let db_path = temp_db_path("paired_baseline_fixture");
        let recorder = SimRecorder::new(&db_path).unwrap();
        let checkpoint_iteration = 7usize;
        for episode in &paired.episodes {
            let sim_id = format!("paired_{:016x}", episode.seed);
            recorder
                .record_simulation_start_with_provenance(
                    &sim_id,
                    "environment",
                    "environment",
                    "environment",
                    "environment",
                    "environment",
                    episode.seed,
                    &SimulationProvenance {
                        runner_kind: "ml_cli".to_string(),
                        policy_kind: "paired_baseline".to_string(),
                        checkpoint_path: Some("test_checkpoint.json".to_string()),
                        checkpoint_iteration: Some(checkpoint_iteration),
                        environment_version: Some(1),
                        action_schema_version: Some(1),
                        config_digest: Some("digest".to_string()),
                        config_override: false,
                        seed_schedule: Some(paired.seed_digest.clone()),
                    },
                )
                .unwrap();
            recorder
                .record_events(
                    &sim_id,
                    &[SimEvent::GameEnd {
                        final_stage: 0,
                        victory: episode.left_victory,
                        clear_rate: episode.left_clear_rate,
                    }],
                )
                .unwrap();
        }

        let connection = rusqlite::Connection::open(&db_path).unwrap();
        for episode in &paired.episodes {
            let sim_id = format!("paired_{:016x}", episode.seed);
            let (policy_kind, stored_checkpoint_path, stored_iteration, seed_schedule): (
                String,
                String,
                i64,
                String,
            ) = connection
                .query_row(
                    "SELECT policy_kind, checkpoint_path, checkpoint_iteration, seed_schedule FROM simulations WHERE id = ?1",
                    [&sim_id],
                    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
                )
                .unwrap();
            assert_eq!(policy_kind, "paired_baseline");
            assert_eq!(stored_checkpoint_path, "test_checkpoint.json");
            assert_eq!(stored_iteration, checkpoint_iteration as i64);
            assert_eq!(seed_schedule, paired.seed_digest);

            let (event_type, event_data): (String, String) = connection
                .query_row(
                    "SELECT event_type, event_data FROM simulation_events WHERE simulation_id = ?1",
                    [&sim_id],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .unwrap();
            assert_eq!(event_type, "game_end");
            let parsed: serde_json::Value = serde_json::from_str(&event_data).unwrap();
            let game_end = parsed
                .get("GameEnd")
                .expect("externally tagged GameEnd payload");
            assert_eq!(
                game_end["clear_rate"],
                serde_json::json!(f64::from(episode.left_clear_rate))
            );
        }

        std::fs::remove_file(&db_path).unwrap();
    }
}
