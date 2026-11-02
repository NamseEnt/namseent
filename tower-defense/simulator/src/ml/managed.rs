use super::contract::config_digest;
use super::feature_contract::InputContract;
use super::model::ModelConfig;
use super::phase4_dataset::{
    EpisodeRecord, Phase4Split, SourcePolicy, collect_into_directory, load_episodes,
};
use super::policy_v2::KindMode;
use super::retraining::prepare_transfer;
use super::semantic_bc::{
    BcCheckpointMetadata, BcTrainConfig, BcTrainInput, LabelSource,
    SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION, prepare_samples, train_bc_run,
};
use super::semantic_candidates::{
    CandidateMode, POLICY_CANDIDATE_SET_VERSION, SEMANTIC_CANDIDATE_ENCODER_VERSION,
};
use super::semantic_ppo::{
    CRITIC_CHECKPOINT_SCHEMA_VERSION, CriticCheckpointMetadata, CriticInputs, CriticPretrainConfig,
    PpoActorFile, PpoConfig, PpoRunInput, PpoRunMetadata, pretrain_critic, train_ppo_run,
    value_samples,
};
use super::support::{current_git_revision, sha256_hex};
use crate::config::GameConfig;
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub const ACTIVE_MANIFEST_SCHEMA_VERSION: u32 = 1;
const HIDDEN_SIZE: usize = 64;
const BOOTSTRAP_TRAIN_GAMES: usize = 256;
const BOOTSTRAP_VALIDATION_GAMES: usize = 64;
const BC_EPOCHS: usize = 6;
const CRITIC_EPOCHS: usize = 4;
const REWARD_SCALE: f32 = 0.1;
const DEVELOPMENT_SEEDS: usize = 128;
const PUBLISH_EVERY: usize = 1;
const TRANSFER_SEED_BASE: u64 = 20_000_000;
const TRANSFER_SEED_STRIDE: u64 = 1_000_000;
const COMPLETE_MARKER: &str = ".complete";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveManifest {
    pub schema_version: u32,
    pub run: String,
    pub iteration: usize,
    pub checkpoint: String,
    pub config_digest: String,
    pub development_seeds: usize,
    pub development_mean_terminal_clear_rate: f64,
    pub actor_sha256: String,
    pub git_revision: String,
}

pub struct TrainOptions {
    pub config: Option<PathBuf>,
    pub iterations: usize,
}

pub fn active_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("artifacts/ml")
}

fn manifest_path(root: &Path) -> PathBuf {
    root.join("active.json")
}

pub fn read_manifest(root: &Path) -> Result<ActiveManifest> {
    let path = manifest_path(root);
    let bytes = std::fs::read(&path).with_context(|| {
        format!(
            "no managed active policy at {}; run `ml train` first or pass --checkpoint",
            path.display()
        )
    })?;
    let manifest: ActiveManifest = serde_json::from_slice(&bytes)
        .with_context(|| format!("invalid active manifest {}", path.display()))?;
    ensure!(
        manifest.schema_version == ACTIVE_MANIFEST_SCHEMA_VERSION,
        "unsupported active manifest schema {}",
        manifest.schema_version
    );
    Ok(manifest)
}

/// Resolves the active iteration directory and verifies its actor weights.
pub fn selected_checkpoint_under(root: &Path) -> Result<PathBuf> {
    let manifest = read_manifest(root)?;
    let checkpoint = root.join(&manifest.checkpoint);
    let actor = checkpoint
        .join("actor.bin")
        .canonicalize()
        .with_context(|| format!("active checkpoint is missing: {}", checkpoint.display()))?;
    ensure!(
        sha256_hex(&std::fs::read(&actor)?) == manifest.actor_sha256,
        "active actor weights do not match the manifest"
    );
    Ok(checkpoint)
}

pub fn resolve_selected_checkpoint(explicit: Option<&Path>) -> Result<PathBuf> {
    match explicit {
        Some(path) => Ok(path.to_path_buf()),
        None => selected_checkpoint_under(&active_root()),
    }
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, bytes)?;
    std::fs::rename(&temporary, path)?;
    Ok(())
}

fn run_dirs(root: &Path) -> Result<Vec<(usize, PathBuf)>> {
    let runs = root.join("runs");
    if !runs.exists() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    for entry in std::fs::read_dir(runs)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if let Some(index) = name
            .strip_prefix("run-")
            .and_then(|index| index.parse::<usize>().ok())
        {
            found.push((index, entry.path()));
        }
    }
    found.sort();
    Ok(found)
}

fn read_ppo_metadata(ppo_dir: &Path) -> Result<Option<PpoRunMetadata>> {
    let path = ppo_dir.join("ppo.json");
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(serde_json::from_slice(&std::fs::read(&path)?)?))
}

fn iteration_dir(ppo_dir: &Path, iteration: usize) -> PathBuf {
    ppo_dir.join(format!("iter-{iteration:04}"))
}

/// Best development iteration, preferring the latest on ties.
fn best_iteration(metadata: &PpoRunMetadata) -> Option<(usize, f64)> {
    metadata
        .history
        .iter()
        .filter_map(|record| record.evaluation.as_ref())
        .filter_map(|evaluation| {
            evaluation
                .summaries
                .iter()
                .find(|summary| summary.policy == "ppo")
                .map(|summary| (evaluation.iteration, summary.mean_terminal_clear_rate))
        })
        .max_by(|left, right| left.1.total_cmp(&right.1).then(left.0.cmp(&right.0)))
}

fn publish(root: &Path, run_dir: &Path, metadata: &PpoRunMetadata) -> Result<()> {
    ensure!(
        metadata.completed_iterations > 0,
        "no completed PPO iteration to publish"
    );
    let ppo_dir = run_dir.join("ppo");
    let (iteration, clear_rate) = best_iteration(metadata)
        .context("run has no development evaluation; cannot choose a policy")?;
    let checkpoint = iteration_dir(&ppo_dir, iteration);
    let environment = metadata
        .environment
        .as_ref()
        .context("run metadata has no environment contract")?;
    let manifest = ActiveManifest {
        schema_version: ACTIVE_MANIFEST_SCHEMA_VERSION,
        run: run_dir
            .strip_prefix(root)
            .unwrap_or(run_dir)
            .display()
            .to_string(),
        iteration,
        checkpoint: checkpoint
            .strip_prefix(root)
            .unwrap_or(&checkpoint)
            .display()
            .to_string(),
        config_digest: config_digest(&environment.game_config),
        development_seeds: metadata.development_seeds,
        development_mean_terminal_clear_rate: clear_rate,
        actor_sha256: sha256_hex(&std::fs::read(checkpoint.join("actor.bin"))?),
        git_revision: current_git_revision()?,
    };
    write_atomic(&manifest_path(root), &serde_json::to_vec_pretty(&manifest)?)?;
    prune(&ppo_dir, metadata, iteration)
}

/// Keeps the initialization, the best and the latest iteration only.
fn prune(ppo_dir: &Path, metadata: &PpoRunMetadata, best: usize) -> Result<()> {
    for entry in std::fs::read_dir(ppo_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(iteration) = name
            .strip_prefix("iter-")
            .and_then(|index| index.parse::<usize>().ok())
        else {
            continue;
        };
        if iteration != 0 && iteration != best && iteration != metadata.completed_iterations {
            std::fs::remove_dir_all(entry.path())?;
        }
    }
    Ok(())
}

fn load_config(path: Option<&Path>) -> Result<Arc<GameConfig>> {
    Ok(Arc::new(match path {
        Some(path) => crate::config::load_jsonc(path)?,
        None => crate::config::load_default_config()?,
    }))
}

fn default_ppo_config() -> PpoConfig {
    PpoConfig {
        actor_learning_rate: 3e-4,
        entropy_coefficient: 0.01,
        ..PpoConfig::default()
    }
}

enum Plan {
    Resume {
        ppo_dir: PathBuf,
    },
    Transfer {
        source_checkpoint: PathBuf,
        source: Box<PpoRunMetadata>,
        transfer: Box<super::retraining::NumericTransfer>,
    },
    Bootstrap,
}

fn plan(root: &Path, config: &GameConfig) -> Result<(Plan, usize)> {
    let runs = run_dirs(root)?;
    let next_index = runs.last().map_or(1, |(index, _)| index + 1);
    let Some((latest_index, latest)) = runs.last() else {
        return Ok((Plan::Bootstrap, next_index));
    };
    let ppo_dir = latest.join("ppo");
    let Some(metadata) = read_ppo_metadata(&ppo_dir)? else {
        return Ok((Plan::Bootstrap, *latest_index));
    };
    let latest_checkpoint = iteration_dir(&ppo_dir, metadata.completed_iterations);
    let actor: PpoActorFile =
        serde_json::from_slice(&std::fs::read(latest_checkpoint.join("ppo-actor.json"))?)?;
    let compatible = actor
        .environment
        .as_ref()
        .is_some_and(|environment| environment.check_run(config).is_ok());
    if compatible {
        return Ok((Plan::Resume { ppo_dir }, next_index));
    }
    let source_checkpoint = match best_iteration(&metadata) {
        Some((iteration, _)) if iteration_dir(&ppo_dir, iteration).exists() => {
            iteration_dir(&ppo_dir, iteration)
        }
        _ => latest_checkpoint,
    };
    match prepare_transfer(&source_checkpoint, None, config) {
        Ok((transfer, source)) => Ok((
            Plan::Transfer {
                source_checkpoint,
                source: Box::new(source),
                transfer: Box::new(transfer),
            },
            next_index,
        )),
        Err(error) => {
            eprintln!(
                "the active policy cannot be carried over to this configuration ({error:#}); \
                 training a new policy from scratch"
            );
            Ok((Plan::Bootstrap, next_index))
        }
    }
}

pub fn train(options: TrainOptions) -> Result<()> {
    ensure!(options.iterations > 0, "--iterations must be positive");
    let config = load_config(options.config.as_deref())?;
    let root = active_root();
    std::fs::create_dir_all(root.join("runs"))?;
    let (plan, next_index) = plan(&root, &config)?;
    let (run_dir, input_template, resumed_from) = match plan {
        Plan::Resume { ppo_dir } => {
            let run_dir = ppo_dir
                .parent()
                .context("ppo directory has no run")?
                .to_path_buf();
            let metadata =
                read_ppo_metadata(&ppo_dir)?.context("run to resume has no PPO metadata")?;
            let template = RunTemplate::from_metadata(&metadata);
            eprintln!("resuming {}", run_dir.display());
            (run_dir, template, true)
        }
        Plan::Transfer {
            source_checkpoint,
            source,
            transfer,
        } => {
            let run_dir = root.join(format!("runs/run-{next_index:03}"));
            let mut ppo = source.config.clone();
            ppo.seed = source.config.seed.wrapping_add(next_index as u64);
            ppo.train_seed_start =
                Some(TRANSFER_SEED_BASE + next_index as u64 * TRANSFER_SEED_STRIDE);
            ppo.train_seed_block_offset = 0;
            eprintln!(
                "configuration changed ({} numeric differences); warm-starting {} from {}",
                transfer.changes.len(),
                run_dir.display(),
                source_checkpoint.display()
            );
            let template = RunTemplate {
                config: ppo,
                init_bc_run: PathBuf::from(&source.init_bc_run),
                init_critic_run: None,
                transfer: Some(*transfer),
                evaluate_every: 1,
                development_seeds: DEVELOPMENT_SEEDS,
                development_seed_start: *Phase4Split::PpoDevelopment.range().start(),
            };
            (run_dir, template, false)
        }
        Plan::Bootstrap => {
            let run_dir = root.join(format!("runs/run-{next_index:03}"));
            let boot = run_dir.join("bootstrap");
            std::fs::create_dir_all(&boot)?;
            bootstrap(&config, &boot)?;
            let template = RunTemplate::fresh(&boot, default_ppo_config());
            (run_dir, template, false)
        }
    };
    let ppo_dir = run_dir.join("ppo");
    let completed =
        read_ppo_metadata(&ppo_dir)?.map_or(0, |metadata| metadata.completed_iterations);
    let target = completed + options.iterations;
    if resumed_from {
        eprintln!("continuing from iteration {completed} to {target}");
    }
    let mut reached = completed;
    while reached < target {
        reached = (reached + PUBLISH_EVERY).min(target);
        let metadata = train_ppo_run(Arc::clone(&config), &ppo_dir, input_template.input(reached))?;
        publish(&root, &run_dir, &metadata)?;
        let manifest = read_manifest(&root)?;
        eprintln!(
            "published iteration {} (development clear rate {:.2}) after {} completed iterations",
            manifest.iteration,
            manifest.development_mean_terminal_clear_rate,
            metadata.completed_iterations
        );
    }
    Ok(())
}

struct RunTemplate {
    config: PpoConfig,
    init_bc_run: PathBuf,
    init_critic_run: Option<PathBuf>,
    transfer: Option<super::retraining::NumericTransfer>,
    evaluate_every: usize,
    development_seeds: usize,
    development_seed_start: u64,
}

impl RunTemplate {
    fn from_metadata(metadata: &PpoRunMetadata) -> Self {
        Self {
            config: metadata.config.clone(),
            init_bc_run: PathBuf::from(&metadata.init_bc_run),
            init_critic_run: metadata.init_critic_run.as_ref().map(PathBuf::from),
            transfer: metadata.transfer.clone(),
            evaluate_every: metadata.evaluate_every,
            development_seeds: metadata.development_seeds,
            development_seed_start: metadata.development_seed_start,
        }
    }

    fn fresh(boot: &Path, config: PpoConfig) -> Self {
        Self {
            config,
            init_bc_run: boot.join("bc"),
            init_critic_run: Some(boot.join("critic")),
            transfer: None,
            evaluate_every: 1,
            development_seeds: DEVELOPMENT_SEEDS,
            development_seed_start: *Phase4Split::PpoDevelopment.range().start(),
        }
    }

    fn input(&self, iterations: usize) -> PpoRunInput {
        PpoRunInput {
            transfer: self.transfer.clone(),
            config: self.config.clone(),
            init_bc_run: self.init_bc_run.clone(),
            init_critic_run: self.init_critic_run.clone(),
            init_ppo_iteration: None,
            iterations,
            evaluate_every: self.evaluate_every,
            development_seeds: self.development_seeds,
            development_seed_start: self.development_seed_start,
            position_reference_actor: None,
        }
    }
}

fn collect_if_missing(
    config: &Arc<GameConfig>,
    directory: &Path,
    split: Phase4Split,
    games: usize,
) -> Result<()> {
    if directory.join(COMPLETE_MARKER).exists() {
        return Ok(());
    }
    if directory.exists() {
        std::fs::remove_dir_all(directory)?;
    }
    eprintln!("collecting {games} canonical games for {}", split.name());
    let seeds = split.seeds(Some(games))?;
    collect_into_directory(
        Arc::clone(config),
        directory,
        split,
        &seeds,
        SourcePolicy::Canonical,
        (0, 1),
        None,
    )?;
    std::fs::write(directory.join(COMPLETE_MARKER), b"ok")?;
    Ok(())
}

fn load_nonempty(directory: &Path, config: &GameConfig) -> Result<Vec<EpisodeRecord>> {
    let episodes = load_episodes(directory, config)?;
    ensure!(
        !episodes.is_empty(),
        "{} contains no episodes",
        directory.display()
    );
    Ok(episodes)
}

/// Builds the behavior-cloned actor and the pretrained critic that start PPO.
fn bootstrap(config: &Arc<GameConfig>, boot: &Path) -> Result<()> {
    let train_dir = boot.join("train");
    let validation_dir = boot.join("validation");
    let bc_dir = boot.join("bc");
    let critic_dir = boot.join("critic");
    let bc_done = bc_dir.join(COMPLETE_MARKER).exists();
    let critic_done = critic_dir.join(COMPLETE_MARKER).exists();
    if !(bc_done && critic_done) {
        collect_if_missing(
            config,
            &train_dir,
            Phase4Split::CanonicalTrain,
            BOOTSTRAP_TRAIN_GAMES,
        )?;
        collect_if_missing(
            config,
            &validation_dir,
            Phase4Split::CanonicalValidation,
            BOOTSTRAP_VALIDATION_GAMES,
        )?;
    }
    if !bc_done {
        eprintln!("bootstrap: behavior cloning");
        let train = load_nonempty(&train_dir, config)?;
        let validation = load_nonempty(&validation_dir, config)?;
        let train_samples = prepare_samples(&train, LabelSource::Chosen, 1.0);
        let validation_samples = prepare_samples(&validation, LabelSource::Chosen, 1.0);
        let metadata = BcCheckpointMetadata {
            schema_version: SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION,
            policy_representation_version: super::policy_v2::POLICY_REPRESENTATION_VERSION,
            policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
            candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
            git_commit: current_git_revision()?,
            model_config: ModelConfig {
                hidden_size: HIDDEN_SIZE,
            },
            config: BcTrainConfig {
                hidden_size: HIDDEN_SIZE,
                learning_rate: 1e-3,
                batch_size: 64,
                epochs: BC_EPOCHS,
                seed: 0,
                label: LabelSource::Chosen,
                override_weight: 1.0,
                kind_mode: KindMode::Learned,
                input_contract: InputContract::Normalized,
                candidate_mode: CandidateMode::Top8,
                cell_label_smoothing: 0.0,
            },
            train_datasets: vec![train_dir.display().to_string()],
            train_provenance: vec![train[0].provenance.clone()],
            validation_dataset: Some(validation_dir.display().to_string()),
            train_samples: train_samples.len(),
            validation_samples: validation_samples.len(),
            init_checkpoint: None,
            completed_epochs: 0,
            best_epoch: None,
            history: Vec::new(),
        };
        train_bc_run(
            &bc_dir,
            BcTrainInput {
                train: &train_samples,
                validation: &validation_samples,
                metadata,
                init_model: None,
            },
        )?;
        std::fs::write(bc_dir.join(COMPLETE_MARKER), b"ok")?;
    }
    if !critic_done {
        eprintln!("bootstrap: critic pretraining");
        let train = load_nonempty(&train_dir, config)?;
        let validation = load_nonempty(&validation_dir, config)?;
        let train_samples = value_samples(&train, REWARD_SCALE);
        let validation_samples = value_samples(&validation, REWARD_SCALE);
        let metadata = CriticCheckpointMetadata {
            schema_version: CRITIC_CHECKPOINT_SCHEMA_VERSION,
            policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
            candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
            game_rules_epoch: super::phase4_dataset::GAME_RULES_EPOCH,
            git_commit: current_git_revision()?,
            model_config: ModelConfig {
                hidden_size: HIDDEN_SIZE,
            },
            config: CriticPretrainConfig {
                hidden_size: HIDDEN_SIZE,
                learning_rate: 1e-3,
                batch_size: 256,
                epochs: CRITIC_EPOCHS,
                reward_scale: REWARD_SCALE,
                seed: 0,
                inputs: CriticInputs::Contract,
            },
            train_dataset: train_dir.display().to_string(),
            validation_dataset: validation_dir.display().to_string(),
            train_provenance: Some(train[0].provenance.clone()),
            train_samples: train_samples.len(),
            validation_samples: validation_samples.len(),
            initial_validation: Default::default(),
            history: Vec::new(),
            selected_epoch: 0,
        };
        pretrain_critic(&critic_dir, &train_samples, &validation_samples, metadata)?;
        std::fs::write(critic_dir.join(COMPLETE_MARKER), b"ok")?;
    }
    for directory in [&train_dir, &validation_dir] {
        if directory.exists() {
            std::fs::remove_dir_all(directory)?;
        }
    }
    Ok(())
}

pub fn status() -> Result<()> {
    let root = active_root();
    match read_manifest(&root) {
        Ok(manifest) => {
            println!("active run: {}", manifest.run);
            println!(
                "active checkpoint: {}",
                root.join(&manifest.checkpoint).display()
            );
            println!("iteration: {}", manifest.iteration);
            println!(
                "development mean terminal clear rate: {:.2} over {} seeds",
                manifest.development_mean_terminal_clear_rate, manifest.development_seeds
            );
            println!("config digest: {}", manifest.config_digest);
            println!("git revision: {}", manifest.git_revision);
        }
        Err(error) => println!("{error:#}"),
    }
    for (index, run_dir) in run_dirs(&root)? {
        match read_ppo_metadata(&run_dir.join("ppo"))? {
            Some(metadata) => println!(
                "run-{index:03}: {} completed iterations{}",
                metadata.completed_iterations,
                if metadata.transfer.is_some() {
                    " (warm-started after a configuration change)"
                } else {
                    ""
                }
            ),
            None => println!("run-{index:03}: bootstrapping"),
        }
    }
    Ok(())
}
