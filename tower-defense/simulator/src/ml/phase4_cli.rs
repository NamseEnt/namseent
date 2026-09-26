use super::model::{InferenceBackend, ModelConfig, TrainBackend, default_policy_device};
use super::neural_checkpoint::current_git_revision;
use super::phase4_dataset::{
    Phase4Split, SourcePolicy, collect_into_directory, load_episodes, merge_directories,
    reward_invariant_report,
};
use super::phase4_eval::{EvalPolicy, evaluate_policies};
use super::semantic_bc::{
    BcCheckpointMetadata, BcTrainConfig, BcTrainInput, LabelSource,
    SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION, SemanticPolicy, evaluate_samples, load_selected_model,
    prepare_samples, train_bc_run,
};
use super::semantic_candidates::{
    POLICY_CANDIDATE_SET_VERSION, SEMANTIC_CANDIDATE_ENCODER_VERSION, encode_candidate,
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
    /// Train (or resume) a semantic BC run.
    TrainBc {
        #[arg(long, required = true, num_args = 1..)]
        train: Vec<PathBuf>,
        /// Use only the first `train_episodes` episodes (by seed) of `train`.
        #[arg(long)]
        train_episodes: Option<usize>,
        #[arg(long)]
        validation: Option<PathBuf>,
        #[arg(long)]
        run_dir: PathBuf,
        /// Start from the selected model of another run (distillation).
        #[arg(long)]
        init_run_dir: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = LabelArg::Chosen)]
        label: LabelArg,
        #[arg(long, default_value_t = 1.0)]
        override_weight: f32,
        #[arg(long, default_value_t = 20)]
        epochs: usize,
        #[arg(long, default_value_t = 1e-3)]
        learning_rate: f64,
        #[arg(long, default_value_t = 64)]
        batch_size: usize,
        #[arg(long, default_value_t = 64)]
        hidden_size: usize,
        #[arg(long, default_value_t = 0)]
        seed: u64,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
    /// Offline BC metrics of a run's selected model on a dataset.
    EvaluateBc {
        #[arg(long)]
        run_dir: PathBuf,
        #[arg(long)]
        dataset: PathBuf,
        #[arg(long, value_enum, default_value_t = LabelArg::Chosen)]
        label: LabelArg,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
    /// Teacher override/disagreement analysis of a teacher corpus.
    TeacherAnalysis {
        #[arg(long)]
        dataset: PathBuf,
        /// Canonical BC run whose predictions are compared on the same states.
        #[arg(long)]
        bc_run_dir: Option<PathBuf>,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
    /// Paired terminal evaluation of canonical and learned policies.
    TerminalEval {
        #[arg(long, value_enum)]
        split: Phase4Split,
        #[arg(long)]
        count: Option<usize>,
        /// `name=run_dir` for each learned policy.
        #[arg(long = "policy")]
        policies: Vec<String>,
        /// `policy:reference` paired comparisons (default: each vs canonical).
        #[arg(long = "compare")]
        comparisons: Vec<String>,
        #[arg(long)]
        output: PathBuf,
        /// Required for the one-time final evaluation.
        #[arg(long)]
        confirm_final: bool,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
    /// Supervised critic pretraining on canonical trajectories.
    PretrainCritic {
        #[arg(long)]
        train: PathBuf,
        #[arg(long)]
        train_episodes: Option<usize>,
        #[arg(long)]
        validation: PathBuf,
        #[arg(long)]
        run_dir: PathBuf,
        #[arg(long, default_value_t = 8)]
        epochs: usize,
        #[arg(long, default_value_t = 1e-3)]
        learning_rate: f64,
        #[arg(long, default_value_t = 256)]
        batch_size: usize,
        #[arg(long, default_value_t = 64)]
        hidden_size: usize,
        #[arg(long, default_value_t = 0.1)]
        reward_scale: f32,
        #[arg(long, default_value_t = 0)]
        seed: u64,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
    /// Train (or resume) semantic PPO from a BC checkpoint.
    PpoTrain {
        #[arg(long)]
        init_bc_run: PathBuf,
        #[arg(long)]
        init_critic_run: Option<PathBuf>,
        /// Continue from a PPO iteration directory (actor and critic).
        #[arg(long)]
        init_ppo_iteration: Option<PathBuf>,
        /// Offset into the `ppo_train` seed blocks.
        #[arg(long, default_value_t = 0)]
        train_seed_block_offset: usize,
        #[arg(long)]
        run_dir: PathBuf,
        #[arg(long)]
        iterations: usize,
        #[arg(long, default_value_t = 48)]
        episodes_per_iteration: usize,
        #[arg(long, default_value_t = 1.0)]
        gamma: f32,
        #[arg(long, default_value_t = 0.95)]
        gae_lambda: f32,
        #[arg(long, default_value_t = 0.1)]
        reward_scale: f32,
        #[arg(long, default_value_t = 1e-5)]
        actor_learning_rate: f64,
        #[arg(long, default_value_t = 3e-4)]
        critic_learning_rate: f64,
        #[arg(long, default_value_t = 4)]
        update_epochs: usize,
        #[arg(long, default_value_t = 256)]
        minibatch_size: usize,
        #[arg(long, default_value_t = 0.2)]
        clip_epsilon: f32,
        #[arg(long, default_value_t = 0.0)]
        entropy_coefficient: f32,
        #[arg(long, default_value_t = 0.0)]
        kl_to_init_coefficient: f32,
        /// Negative disables the early stop.
        #[arg(long, default_value_t = 0.02)]
        target_kl: f32,
        #[arg(long, default_value_t = 0.5)]
        max_grad_norm: f32,
        #[arg(long, default_value_t = 0)]
        critic_warmup_iterations: usize,
        #[arg(long, default_value_t = 0)]
        seed: u64,
        /// Development evaluation every N iterations (0: never).
        #[arg(long, default_value_t = 5)]
        evaluate_every: usize,
        #[arg(long, default_value_t = 128)]
        development_seeds: usize,
        #[arg(long, default_value_t = 0)]
        threads: usize,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum LabelArg {
    Chosen,
    Canonical,
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
    /// Decisions whose chosen candidate has the same encoding as another
    /// candidate, by chosen action kind.
    aliased_chosen_by_kind: std::collections::BTreeMap<String, usize>,
    mean_candidates_per_decision: f64,
    max_candidates_per_decision: usize,
    reward_invariant: super::phase4_dataset::RewardInvariantReport,
}

fn label_source(label: LabelArg) -> LabelSource {
    match label {
        LabelArg::Chosen => LabelSource::Chosen,
        LabelArg::Canonical => LabelSource::Canonical,
    }
}

fn load_limited(
    directory: &Path,
    config: &GameConfig,
    limit: Option<usize>,
) -> Result<Vec<super::phase4_dataset::EpisodeRecord>> {
    let mut episodes = load_episodes(directory, config)?;
    if let Some(limit) = limit {
        if episodes.len() < limit {
            bail!(
                "{} has {} episodes; {limit} requested",
                directory.display(),
                episodes.len()
            );
        }
        episodes.truncate(limit);
    }
    if episodes.is_empty() {
        bail!("{} contains no episodes", directory.display());
    }
    Ok(episodes)
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
            if source == SourcePolicy::Canonical && !split.is_canonical_data_split() {
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
            let mut aliased_chosen_by_kind = std::collections::BTreeMap::new();
            for sample in episodes.iter().flat_map(|episode| &episode.samples) {
                decisions += 1;
                *action_kind_counts
                    .entry(sample.action_kind.clone())
                    .or_insert(0) += 1;
                *decision_point_counts
                    .entry(format!("{:?}", sample.decision_point))
                    .or_insert(0) += 1;
                let encoded = sample
                    .candidates
                    .iter()
                    .map(|candidate| {
                        encode_candidate(&sample.observation, &candidate.policy_candidate())
                    })
                    .collect::<Vec<_>>();
                if encoded.iter().enumerate().any(|(index, set)| {
                    index != sample.chosen_index && *set == encoded[sample.chosen_index]
                }) {
                    *aliased_chosen_by_kind
                        .entry(sample.action_kind.clone())
                        .or_insert(0) += 1;
                }
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
                aliased_chosen_by_kind,
                mean_candidates_per_decision: candidates as f64 / decisions.max(1) as f64,
                max_candidates_per_decision: max_candidates,
                reward_invariant: reward_invariant_report(&episodes),
            };
            write_json(output.as_deref(), &report)
        }
        Phase4Command::TrainBc {
            train,
            train_episodes,
            validation,
            run_dir,
            init_run_dir,
            label,
            override_weight,
            epochs,
            learning_rate,
            batch_size,
            hidden_size,
            seed,
            threads,
        } => {
            configure_threads(threads)?;
            let started = std::time::Instant::now();
            let mut train_episodes_loaded = Vec::new();
            let mut train_provenance = Vec::new();
            for directory in &train {
                let episodes = load_limited(directory, &config, train_episodes)?;
                train_provenance.push(episodes[0].provenance.clone());
                train_episodes_loaded.extend(episodes);
            }
            let label = label_source(label);
            let train_samples = prepare_samples(&train_episodes_loaded, label, override_weight);
            drop(train_episodes_loaded);
            let validation_samples = match &validation {
                Some(directory) => {
                    prepare_samples(&load_limited(directory, &config, None)?, label, 1.0)
                }
                None => Vec::new(),
            };
            eprintln!(
                "loaded {} train / {} validation samples in {:.1}s",
                train_samples.len(),
                validation_samples.len(),
                started.elapsed().as_secs_f64()
            );
            let device = default_policy_device();
            let (model_config, init_model) = match &init_run_dir {
                Some(directory) => {
                    let (metadata, model) =
                        load_selected_model::<TrainBackend>(directory, &device)?;
                    (metadata.model_config, Some(model))
                }
                None => (ModelConfig { hidden_size }, None),
            };
            let config_record = BcTrainConfig {
                hidden_size: model_config.hidden_size,
                learning_rate,
                batch_size,
                epochs,
                seed,
                label,
                override_weight,
            };
            let metadata = BcCheckpointMetadata {
                schema_version: SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION,
                policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
                candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
                git_commit: current_git_revision()?,
                model_config,
                config: config_record,
                train_datasets: train
                    .iter()
                    .map(|path| match train_episodes {
                        Some(limit) => format!("{} (first {limit})", path.display()),
                        None => path.display().to_string(),
                    })
                    .collect(),
                train_provenance,
                validation_dataset: validation.as_ref().map(|path| path.display().to_string()),
                train_samples: train_samples.len(),
                validation_samples: validation_samples.len(),
                init_checkpoint: init_run_dir.as_ref().map(|path| path.display().to_string()),
                completed_epochs: 0,
                best_epoch: None,
                history: Vec::new(),
            };
            let result = train_bc_run(
                &run_dir,
                BcTrainInput {
                    train: &train_samples,
                    validation: &validation_samples,
                    metadata,
                    init_model,
                },
            )?;
            println!(
                "run {} complete: {} epochs, selected epoch {:?}, {:.1}s",
                run_dir.display(),
                result.completed_epochs,
                result.best_epoch,
                started.elapsed().as_secs_f64()
            );
            Ok(())
        }
        Phase4Command::EvaluateBc {
            run_dir,
            dataset,
            label,
            output,
            threads,
        } => {
            configure_threads(threads)?;
            let device = default_policy_device();
            let (_, model) = load_selected_model::<InferenceBackend>(&run_dir, &device)?;
            let samples = prepare_samples(
                &load_limited(&dataset, &config, None)?,
                label_source(label),
                1.0,
            );
            let metrics = evaluate_samples(&model, &samples, &device)?;
            write_json(output.as_deref(), &metrics)
        }
        Phase4Command::TeacherAnalysis {
            dataset,
            bc_run_dir,
            output,
            threads,
        } => {
            configure_threads(threads)?;
            let episodes = load_limited(&dataset, &config, None)?;
            let policy = bc_run_dir
                .as_deref()
                .map(SemanticPolicy::from_run_dir)
                .transpose()?;
            let analysis =
                super::phase4_analysis::analyze_teacher_corpus(&episodes, policy.as_ref())?;
            write_json(output.as_deref(), &analysis)
        }
        Phase4Command::TerminalEval {
            split,
            count,
            policies,
            comparisons,
            output,
            confirm_final,
            threads,
        } => {
            configure_threads(threads)?;
            if !split.is_evaluation_split() {
                bail!("terminal evaluation runs on development or final evaluation seeds only");
            }
            if split.is_final_split() {
                if !confirm_final {
                    bail!("the final evaluation runs once; pass --confirm-final");
                }
                if output.exists() {
                    bail!(
                        "{} exists: the final evaluation already ran",
                        output.display()
                    );
                }
            }
            let mut eval_policies = vec![EvalPolicy::Canonical];
            for spec in &policies {
                let (name, directory) = spec
                    .split_once('=')
                    .ok_or_else(|| anyhow::anyhow!("--policy expects name=run_dir"))?;
                eval_policies.push(EvalPolicy::Learned {
                    name: name.to_string(),
                    policy: SemanticPolicy::from_path(Path::new(directory))?,
                });
            }
            let comparisons = if comparisons.is_empty() {
                eval_policies[1..]
                    .iter()
                    .map(|policy| (policy.name().to_string(), "canonical".to_string()))
                    .collect::<Vec<_>>()
            } else {
                comparisons
                    .iter()
                    .map(|spec| {
                        spec.split_once(':')
                            .map(|(left, right)| (left.to_string(), right.to_string()))
                            .ok_or_else(|| anyhow::anyhow!("--compare expects policy:reference"))
                    })
                    .collect::<Result<Vec<_>>>()?
            };
            let seeds = split.seeds(count)?;
            let report =
                evaluate_policies(config, split.name(), &seeds, &eval_policies, &comparisons)?;
            for summary in &report.summaries {
                eprintln!(
                    "{}: mean {:.2} median {:.2} stage {:.2} decisions {:.1} illegal {} fallback {} post_sampling_mutations {} \
                     agreement {:.3} {:.3} ms/decision",
                    summary.policy,
                    summary.mean_terminal_clear_rate,
                    summary.median_terminal_clear_rate,
                    summary.mean_final_stage,
                    summary.mean_decisions,
                    summary.illegal_actions,
                    summary.fallback_actions,
                    summary.post_sampling_mutations,
                    summary.canonical_agreement_rate,
                    summary.mean_decision_ms
                );
            }
            for comparison in &report.comparisons {
                eprintln!(
                    "{} - {}: mean {:+.2} SE {:.2} median {:+.2} better/worse/tie {}/{}/{}",
                    comparison.policy,
                    comparison.reference,
                    comparison.mean,
                    comparison.se,
                    comparison.median,
                    comparison.better,
                    comparison.worse,
                    comparison.tie
                );
            }
            write_json(Some(&output), &report)
        }
        Phase4Command::PretrainCritic {
            train,
            train_episodes,
            validation,
            run_dir,
            epochs,
            learning_rate,
            batch_size,
            hidden_size,
            reward_scale,
            seed,
            threads,
        } => {
            configure_threads(threads)?;
            let started = std::time::Instant::now();
            let train_loaded = load_limited(&train, &config, train_episodes)?;
            let train_provenance = train_loaded[0].provenance.clone();
            let train_samples = super::semantic_ppo::value_samples(&train_loaded, reward_scale);
            drop(train_loaded);
            let validation_samples = super::semantic_ppo::value_samples(
                &load_limited(&validation, &config, None)?,
                reward_scale,
            );
            eprintln!(
                "loaded {} train / {} validation value samples in {:.1}s",
                train_samples.len(),
                validation_samples.len(),
                started.elapsed().as_secs_f64()
            );
            let metadata = super::semantic_ppo::CriticCheckpointMetadata {
                schema_version: super::semantic_ppo::CRITIC_CHECKPOINT_SCHEMA_VERSION,
                candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
                policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
                game_rules_epoch: super::phase4_dataset::GAME_RULES_EPOCH,
                git_commit: current_git_revision()?,
                model_config: ModelConfig { hidden_size },
                config: super::semantic_ppo::CriticPretrainConfig {
                    hidden_size,
                    learning_rate,
                    batch_size,
                    epochs,
                    reward_scale,
                    seed,
                },
                train_dataset: match train_episodes {
                    Some(limit) => format!("{} (first {limit})", train.display()),
                    None => train.display().to_string(),
                },
                validation_dataset: validation.display().to_string(),
                train_provenance: Some(train_provenance),
                train_samples: train_samples.len(),
                validation_samples: validation_samples.len(),
                initial_validation: Default::default(),
                history: Vec::new(),
                selected_epoch: 0,
            };
            let result = super::semantic_ppo::pretrain_critic(
                &run_dir,
                &train_samples,
                &validation_samples,
                metadata,
            )?;
            eprintln!("selected critic epoch {}", result.selected_epoch);
            Ok(())
        }
        Phase4Command::PpoTrain {
            init_bc_run,
            init_critic_run,
            init_ppo_iteration,
            train_seed_block_offset,
            run_dir,
            iterations,
            episodes_per_iteration,
            gamma,
            gae_lambda,
            reward_scale,
            actor_learning_rate,
            critic_learning_rate,
            update_epochs,
            minibatch_size,
            clip_epsilon,
            entropy_coefficient,
            kl_to_init_coefficient,
            target_kl,
            max_grad_norm,
            critic_warmup_iterations,
            seed,
            evaluate_every,
            development_seeds,
            threads,
        } => {
            configure_threads(threads)?;
            let ppo_config = super::semantic_ppo::PpoConfig {
                episodes_per_iteration,
                gamma,
                gae_lambda,
                reward_scale,
                actor_learning_rate,
                critic_learning_rate,
                update_epochs,
                minibatch_size,
                clip_epsilon,
                entropy_coefficient,
                kl_to_init_coefficient,
                target_kl: (target_kl > 0.0).then_some(target_kl),
                max_grad_norm,
                normalize_advantages: true,
                critic_warmup_iterations,
                seed,
                train_seed_block_offset,
            };
            let metadata = super::semantic_ppo::train_ppo_run(
                config,
                &run_dir,
                super::semantic_ppo::PpoRunInput {
                    config: ppo_config,
                    init_bc_run,
                    init_critic_run,
                    init_ppo_iteration,
                    iterations,
                    evaluate_every,
                    development_seeds,
                },
            )?;
            eprintln!("completed {} PPO iterations", metadata.completed_iterations);
            Ok(())
        }
    }
}
