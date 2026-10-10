//! Explicit checkpoint transfer to compatible content, rules and configuration changes.

use super::contract::MlContract;
use super::model::ENTITY_ENCODER_SCHEMA_VERSION;
use super::phase4_dataset::{GAME_RULES_EPOCH, Phase4Split, core_tree_hash};
use super::phase4_eval::{EvalPolicy, evaluate_policies};
use super::semantic_bc::{BcCheckpointMetadata, SemanticPolicy};
use super::semantic_candidates::{
    CandidateMode, POLICY_CANDIDATE_SET_VERSION, SEMANTIC_CANDIDATE_ENCODER_VERSION,
};
use super::semantic_ppo::{
    self, ActorUpdateMode, CriticInputs, PpoActorFile, PpoConfig, PpoRunInput, PpoRunMetadata,
    TrainingBudget,
};
use crate::config::{GameConfig, config_digest, load_jsonc};
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentContract {
    pub schema_version: u32,
    pub ml: MlContract,
    pub game_config: GameConfig,
    pub game_rules_epoch: u32,
    pub entity_encoder_version: u32,
    pub candidate_encoder_version: u32,
    pub policy_candidate_set_version: u32,
    pub core_tree_hash: String,
    #[serde(default)]
    pub content_catalog: std::collections::BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub encoding_sources: std::collections::BTreeMap<String, String>,
}

impl EnvironmentContract {
    pub fn capture(config: &GameConfig) -> Self {
        Self {
            schema_version: 2,
            ml: MlContract::from_config(config),
            game_config: config.clone(),
            game_rules_epoch: GAME_RULES_EPOCH,
            entity_encoder_version: ENTITY_ENCODER_SCHEMA_VERSION,
            candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
            policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
            core_tree_hash: core_tree_hash(),
            content_catalog: content_catalog(),
            encoding_sources: encoding_sources(),
        }
    }

    pub fn check_encoding(&self) -> Result<()> {
        self.game_config.validate().map_err(anyhow::Error::msg)?;
        let mut current = Self::capture(&self.game_config);
        ensure!(
            self.schema_version == 1 || self.schema_version == 2,
            "unknown environment contract"
        );
        if !self.content_catalog.is_empty() {
            validate_catalog_extension(&self.content_catalog, &current.content_catalog)?;
            // Catalog extensions keep existing raw IDs and fixed model dimensions.
            current.ml.catalog_schema_version = self.ml.catalog_schema_version;
        }
        if !self.encoding_sources.is_empty() {
            ensure!(
                self.encoding_sources == current.encoding_sources,
                "encoder source changed; a representation migration adapter is required"
            );
        }
        ensure!(
            self.ml == current.ml
                && self.entity_encoder_version == current.entity_encoder_version
                && self.candidate_encoder_version == current.candidate_encoder_version
                && self.policy_candidate_set_version == current.policy_candidate_set_version,
            "checkpoint observation/action/feature contract is incompatible; a structural migration adapter is required"
        );
        Ok(())
    }

    pub fn check_run(&self, config: &GameConfig) -> Result<()> {
        self.check_encoding()?;
        ensure!(
            self.game_rules_epoch == GAME_RULES_EPOCH,
            "rules changed; use a new retraining run"
        );
        ensure!(
            self.ml.config_digest == config_digest(config),
            "game configuration differs from the saved run; use a new retraining run"
        );
        ensure!(
            self.core_tree_hash == core_tree_hash(),
            "core source changed since this run; resume requires the original rules source"
        );
        Ok(())
    }
}

/// Content IDs are append-only. The entity embedding already reserves 4096
/// entries, so adding a kind does not resize any saved parameter tensor.
fn content_catalog() -> std::collections::BTreeMap<String, Vec<String>> {
    [
        (
            "treasures",
            td_core::UpgradeKind::ALL
                .iter()
                .map(|kind| kind.key().to_string())
                .collect(),
        ),
        (
            "items",
            td_core::ItemKind::ALL
                .iter()
                .map(|kind| kind.key().to_string())
                .collect(),
        ),
        (
            "card_services",
            td_core::CardServiceKind::ALL
                .iter()
                .map(|kind| kind.key().to_string())
                .collect(),
        ),
    ]
    .into_iter()
    .map(|(key, values)| (key.to_string(), values))
    .collect()
}

fn validate_catalog_extension(
    source: &std::collections::BTreeMap<String, Vec<String>>,
    target: &std::collections::BTreeMap<String, Vec<String>>,
) -> Result<()> {
    ensure!(
        source.keys().eq(target.keys()),
        "content catalog families changed"
    );
    for (index, key) in target.get("treasures").into_iter().flatten().enumerate() {
        // Core's names are PascalCase; treasure decision observations use snake_case.
        let mut observation_key = String::new();
        for (position, ch) in key.chars().enumerate() {
            if ch.is_ascii_uppercase() && position > 0 {
                observation_key.push('_');
            }
            observation_key.push(ch.to_ascii_lowercase());
        }
        ensure!(
            super::vocabulary::upgrade_key_id(&observation_key) as usize == index + 1,
            "treasure {key}: register its stable observation vocabulary ID before retraining"
        );
    }
    for (family, old) in source {
        let new = &target[family];
        ensure!(
            new.starts_with(old) && new.len() < 4096,
            "{family}: existing content IDs changed or embedding capacity exceeded"
        );
    }
    Ok(())
}

fn encoding_sources() -> std::collections::BTreeMap<String, String> {
    [
        ("features", include_str!("features.rs")),
        ("model", include_str!("model.rs")),
        ("policy", include_str!("policy_v2.rs")),
        ("candidate", include_str!("semantic_candidates.rs")),
        ("observation", include_str!("encoding/observation.rs")),
        ("normalization", include_str!("encoding/normalize.rs")),
        ("feature_contract", include_str!("feature_contract.rs")),
    ]
    .into_iter()
    .map(|(key, source)| {
        (
            key.to_string(),
            format!("{:x}", Sha256::digest(source.as_bytes())),
        )
    })
    .collect()
}

pub fn file_sha256(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn verify_legacy_encoding(revision: &str) -> Result<()> {
    ensure!(
        revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "legacy source needs a recorded Git commit to verify its encoding"
    );
    let files = [
        ("contract.rs", include_str!("contract.rs")),
        ("features.rs", include_str!("features.rs")),
        ("feature_contract.rs", include_str!("feature_contract.rs")),
        ("model.rs", include_str!("model.rs")),
        ("policy_v2.rs", include_str!("policy_v2.rs")),
        ("semantic_bc.rs", include_str!("semantic_bc.rs")),
        (
            "semantic_candidates.rs",
            include_str!("semantic_candidates.rs"),
        ),
        ("spatial.rs", include_str!("spatial.rs")),
        ("vocabulary.rs", include_str!("vocabulary.rs")),
        ("encoding/mod.rs", include_str!("encoding/mod.rs")),
        ("encoding/entity.rs", include_str!("encoding/entity.rs")),
        (
            "encoding/observation.rs",
            include_str!("encoding/observation.rs"),
        ),
        (
            "encoding/normalize.rs",
            include_str!("encoding/normalize.rs"),
        ),
        ("encoding/batch.rs", include_str!("encoding/batch.rs")),
        ("encoding/tensor.rs", include_str!("encoding/tensor.rs")),
    ];
    for (path, compiled) in files {
        let object = format!("{revision}:tower-defense/simulator/src/ml/{path}");
        let output = std::process::Command::new("git")
            .args([
                "-C",
                env!("CARGO_MANIFEST_DIR"),
                "cat-file",
                "blob",
                &object,
            ])
            .output()?;
        // The only byte-different legacy adapter is additive candidate expansion;
        // pin both source and target blobs so future encoder edits cannot inherit it.
        ensure!(
            output.status.success()
                && (output.stdout == compiled.as_bytes()
                    || (path == "semantic_candidates.rs"
                        && format!("{:x}", Sha256::digest(compiled.as_bytes()))
                            == "9a5821cd078bfb8d24032c0de30a46995c22f9c6976296542af2c251058a4340"
                        && format!("{:x}", Sha256::digest(&output.stdout))
                            == "56d519a1d5bb718bfba024de780d33bbce41d55651f20321854f66af148cfffb")),
            "legacy source encoding cannot be verified at {path}; an explicit migration adapter is required"
        );
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigChange {
    pub path: String,
    pub before: Value,
    pub after: Value,
}

pub fn numeric_changes(source: &GameConfig, target: &GameConfig) -> Result<Vec<ConfigChange>> {
    source.validate().map_err(anyhow::Error::msg)?;
    target.validate().map_err(anyhow::Error::msg)?;
    fn visit(
        path: &str,
        before: &Value,
        after: &Value,
        changes: &mut Vec<ConfigChange>,
    ) -> Result<()> {
        match (before, after) {
            (Value::Object(left), Value::Object(right)) if left.keys().eq(right.keys()) => {
                for (key, value) in left {
                    visit(&format!("{path}/{key}"), value, &right[key], changes)?;
                }
            }
            (Value::Array(left), Value::Array(right)) if left.len() == right.len() => {
                for (index, (left, right)) in left.iter().zip(right).enumerate() {
                    visit(&format!("{path}/{index}"), left, right, changes)?;
                }
            }
            (Value::Number(_), Value::Number(_)) if before != after =>
            {
                ensure!(
                    !path.ends_with("/kind") && !path.ends_with("/stage"),
                    "catalog/stage identity changed at {path}; numeric transfer cannot remap identities"
                );
                changes.push(ConfigChange {
                    path: path.to_string(),
                    before: before.clone(),
                    after: after.clone(),
                });
            }
            _ if before == after => {}
            _ => bail!(
                "configuration structure changed at {path}; a structural migration adapter is required"
            ),
        }
        Ok(())
    }
    let mut changes = Vec::new();
    let normalized = |config: &GameConfig| -> Result<Value> {
        let mut value = serde_json::to_value(config)?;
        Ok(value)
    };
    visit("", &normalized(source)?, &normalized(target)?, &mut changes)?;
    Ok(changes)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericTransfer {
    pub source_checkpoint: PathBuf,
    pub source_actor_sha256: String,
    pub source_critic_sha256: String,
    pub source_actor_metadata_sha256: String,
    pub source_environment: EnvironmentContract,
    pub source_git_commit: String,
    pub legacy_source_contract: bool,
    pub source_budget: TrainingBudget,
    pub source_bc_metadata: BcCheckpointMetadata,
    pub critic_inputs: CriticInputs,
    pub changes: Vec<ConfigChange>,
    pub optimizer_policy: String,
    #[serde(default)]
    pub target_candidate_mode: CandidateMode,
}

impl NumericTransfer {
    pub fn validate(&self, target: &GameConfig) -> Result<()> {
        self.source_environment.check_encoding()?;
        if self.legacy_source_contract {
            verify_legacy_encoding(&self.source_git_commit)?;
        }
        ensure!(
            self.optimizer_policy == "fresh_adam",
            "unsupported optimizer transfer policy"
        );
        ensure!(
            self.changes == numeric_changes(&self.source_environment.game_config, target)?,
            "numeric migration report does not match source/target configuration"
        );
        ensure!(
            self.source_actor_sha256 == file_sha256(&self.source_checkpoint.join("actor.bin"))?
                && self.source_critic_sha256
                    == file_sha256(&self.source_checkpoint.join("critic.bin"))?
                && self.source_actor_metadata_sha256
                    == file_sha256(&self.source_checkpoint.join("ppo-actor.json"))?,
            "source checkpoint weights changed after migration was prepared"
        );
        let actor: PpoActorFile = read_json(&self.source_checkpoint.join("ppo-actor.json"))?;
        ensure!(
            self.legacy_source_contract == actor.environment.is_none(),
            "source contract classification changed"
        );
        if let Some(environment) = &actor.environment {
            ensure!(
                environment == &self.source_environment,
                "source environment differs from migration metadata"
            );
        }
        ensure!(
            matches!(
                actor.candidate_mode,
                CandidateMode::Top8 | CandidateMode::AllBuildOptions
            ) && matches!(
                self.target_candidate_mode,
                CandidateMode::Top8 | CandidateMode::AllBuildOptions
            ) && !(actor.candidate_mode == CandidateMode::AllBuildOptions
                && self.target_candidate_mode == CandidateMode::Top8)
                && actor.model_config == self.source_bc_metadata.model_config
                && actor.kind_mode == self.source_bc_metadata.config.kind_mode
                && actor.input_contract == self.source_bc_metadata.config.input_contract,
            "retraining requires Top8 or all-build-options and a matching model/input contract"
        );
        // The normal loader still validates representation and checkpoint versions.
        Ok(())
    }
}

fn default_evaluate_every() -> usize {
    5
}
fn default_development_seeds() -> usize {
    128
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RetrainingSpec {
    pub schema_version: u32,
    /// Optional explicit Top8 -> all legal construction options migration.
    pub candidate_mode: Option<CandidateMode>,
    pub source_checkpoint: PathBuf,
    /// Required for a historical checkpoint without a config snapshot.
    pub source_config: Option<PathBuf>,
    pub target_config: PathBuf,
    pub run_dir: PathBuf,
    pub iterations: usize,
    pub train_seed_start: u64,
    pub seed: u64,
    /// Omitted: retain the source run's PPO recipe, using new seed streams.
    pub ppo: Option<PpoConfig>,
    #[serde(default = "default_evaluate_every")]
    pub evaluate_every: usize,
    #[serde(default = "default_development_seeds")]
    pub development_seeds: usize,
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    serde_json::from_slice(
        &std::fs::read(path).with_context(|| format!("read {}", path.display()))?,
    )
    .with_context(|| format!("decode {}", path.display()))
}

pub fn write_json_atomic(path: &Path, value: &impl Serialize) -> Result<()> {
    let temporary = path.with_extension("json.tmp");
    std::fs::write(&temporary, serde_json::to_vec_pretty(value)?)?;
    std::fs::rename(temporary, path)?;
    Ok(())
}

pub fn prepare_transfer(
    source_checkpoint: &Path,
    source_config: Option<&GameConfig>,
    target: &GameConfig,
) -> Result<(NumericTransfer, PpoRunMetadata)> {
    let source_checkpoint = source_checkpoint.canonicalize()?;
    let parent = source_checkpoint
        .parent()
        .context("source checkpoint has no run directory")?;
    let source: PpoRunMetadata = read_json(&parent.join("ppo.json"))?;
    let actor: PpoActorFile = read_json(&source_checkpoint.join("ppo-actor.json"))?;
    ensure!(
        actor.iteration <= source.completed_iterations
            && actor.game_rules_epoch == source.game_rules_epoch
            && actor.model_config == source.model_config
            && actor.candidate_mode == source.candidate_mode
            && actor.environment == source.environment,
        "source checkpoint does not match its parent run metadata"
    );
    ensure!(
        source.config.actor_update_mode == ActorUpdateMode::Full,
        "numeric retraining currently supports full-actor PPO checkpoints only"
    );
    let legacy_source_contract = actor.environment.is_none();
    let environment = match &actor.environment {
        Some(environment) => {
            if let Some(config) = source_config {
                ensure!(
                    environment.ml.config_digest == config_digest(config),
                    "supplied source_config differs from the source checkpoint"
                );
            }
            environment.clone()
        }
        None => {
            let config = source_config.context("legacy checkpoint has no config snapshot; supply source_config in the retraining spec")?;
            ensure!(
                !source.init_bc_metadata.train_provenance.is_empty(),
                "legacy checkpoint has no source dataset provenance"
            );
            for provenance in &source.init_bc_metadata.train_provenance {
                let mut compatible = provenance.clone();
                compatible.game_rules_epoch = GAME_RULES_EPOCH;
                compatible.check_current(config)?;
                ensure!(
                    provenance.candidate_encoder_version == SEMANTIC_CANDIDATE_ENCODER_VERSION,
                    "legacy candidate encoder is incompatible"
                );
            }

            let mut environment = EnvironmentContract::capture(config);
            environment.game_rules_epoch = source.game_rules_epoch;
            environment.core_tree_hash = std::process::Command::new("git")
                .args([
                    "-C",
                    env!("CARGO_MANIFEST_DIR"),
                    "rev-parse",
                    &format!("{}:tower-defense/core", source.git_commit),
                ])
                .output()
                .ok()
                .filter(|out| out.status.success())
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|text| text.trim().to_string())
                .unwrap_or_else(|| "legacy:unavailable".to_string());
            environment
        }
    };
    let source_budget = semantic_ppo::budget_until(&source.history, actor.iteration);
    let transfer = NumericTransfer {
        source_actor_sha256: file_sha256(&source_checkpoint.join("actor.bin"))?,
        source_critic_sha256: file_sha256(&source_checkpoint.join("critic.bin"))?,
        source_actor_metadata_sha256: file_sha256(&source_checkpoint.join("ppo-actor.json"))?,
        source_checkpoint,
        changes: numeric_changes(&environment.game_config, target)?,
        source_environment: environment,
        source_git_commit: source.git_commit.clone(),
        legacy_source_contract,
        source_budget,
        source_bc_metadata: source.init_bc_metadata.clone(),
        critic_inputs: source.critic_inputs,
        optimizer_policy: "fresh_adam".to_string(),
        target_candidate_mode: actor.candidate_mode,
    };
    transfer.validate(target)?;
    Ok((transfer, source))
}

/// Use only for an explicit new transfer run, after representation validation.
/// Ordinary evaluation/resume still requires the current rules epoch.
fn load_transfer_policy(path: &Path) -> Result<SemanticPolicy> {
    let actor: PpoActorFile = read_json(&path.join("ppo-actor.json"))?;
    Ok(SemanticPolicy::new(semantic_ppo::load_transfer_actor_as::<
        super::model::InferenceBackend,
    >(path, &super::model::default_policy_device())?)
    .with_candidate_mode(actor.candidate_mode))
}

pub fn check_evaluation_config(path: &Path, config: &GameConfig) -> Result<()> {
    if path.join("ppo-actor.json").exists() {
        let actor: PpoActorFile = read_json(&path.join("ppo-actor.json"))?;
        if let Some(environment) = actor.environment {
            return environment.check_run(config);
        }
        let parent: PpoRunMetadata = read_json(
            &path
                .parent()
                .context("checkpoint has no parent")?
                .join("ppo.json"),
        )?;
        for provenance in &parent.init_bc_metadata.train_provenance {
            provenance.check_current(config)?;
        }
    } else {
        let metadata = super::semantic_bc::load_checkpoint_metadata(path)?;
        for provenance in &metadata.train_provenance {
            provenance.check_current(config)?;
        }
    }
    Ok(())
}

fn validate_training_seeds(start: u64, episodes: usize, iterations: usize) -> Result<()> {
    let count = episodes
        .checked_mul(iterations)
        .context("training seed budget overflow")?;
    ensure!(
        count > 0,
        "retraining needs a positive iteration and episode budget"
    );
    let end = start
        .checked_add(u64::try_from(count)? - 1)
        .context("training seed range overflow")?;
    for split in [
        Phase4Split::CanonicalTrain,
        Phase4Split::CanonicalValidation,
        Phase4Split::TeacherTrain,
        Phase4Split::DevelopmentEvaluation,
        Phase4Split::FinalEvaluation,
        Phase4Split::Phase4bCanonicalTrain,
        Phase4Split::Phase4bCanonicalValidation,
        Phase4Split::PpoDevelopment,
        Phase4Split::Phase4bFinal,
        Phase4Split::V2Final,
        Phase4Split::RetrainingValidation,
    ] {
        let reserved = split.range();
        ensure!(
            end < *reserved.start() || start > *reserved.end(),
            "training seeds overlap reserved {} seeds",
            split.name()
        );
    }
    let reserved = super::phase4_dataset::PHASE3_RESERVED_GAME_SEEDS;
    ensure!(
        end < *reserved.start() || start > *reserved.end(),
        "training seeds overlap Phase 3 reserved seeds"
    );
    Ok(())
}

pub fn run_spec(path: &Path) -> Result<()> {
    let workflow_started = std::time::Instant::now();
    let mut spec: RetrainingSpec = read_json(path)?;
    ensure!(
        spec.schema_version == 1,
        "unsupported retraining spec version"
    );
    ensure!(
        spec.evaluate_every > 0 && spec.development_seeds > 0,
        "retraining requires development evaluation"
    );
    Phase4Split::PpoDevelopment.seeds(Some(spec.development_seeds))?;
    let base = path
        .canonicalize()?
        .parent()
        .context("spec has no parent")?
        .to_path_buf();
    let resolve = |path: &Path| {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            base.join(path)
        }
    };
    spec.source_checkpoint = resolve(&spec.source_checkpoint).canonicalize()?;
    spec.target_config = resolve(&spec.target_config).canonicalize()?;
    spec.source_config = spec
        .source_config
        .as_deref()
        .map(|path| resolve(path).canonicalize())
        .transpose()?;
    spec.run_dir = resolve(&spec.run_dir);
    let target = Arc::new(load_jsonc(&spec.target_config)?);
    let source_config = spec.source_config.as_deref().map(load_jsonc).transpose()?;
    let (mut transfer, parent) =
        prepare_transfer(&spec.source_checkpoint, source_config.as_ref(), &target)?;
    if let Some(mode) = spec.candidate_mode {
        transfer.target_candidate_mode = mode;
        transfer.validate(&target)?;
    }
    let mut ppo = spec.ppo.clone().unwrap_or_else(|| parent.config.clone());
    ensure!(
        ppo.actor_update_mode == ActorUpdateMode::Full,
        "retraining currently supports full-actor updates only"
    );
    ppo.seed = spec.seed;
    ppo.train_seed_start = Some(spec.train_seed_start);
    ppo.train_seed_block_offset = 0;
    validate_training_seeds(
        spec.train_seed_start,
        ppo.episodes_per_iteration,
        spec.iterations,
    )?;
    let end =
        spec.train_seed_start + u64::try_from(ppo.episodes_per_iteration * spec.iterations)? - 1;
    for record in &parent.history {
        if record.iteration != 0 {
            ensure!(
                end < record.train_seeds.0 || spec.train_seed_start > record.train_seeds.1,
                "new training seeds overlap the source run; choose a fresh train_seed_start"
            );
        }
    }
    // Refuse to put a new run inside its source, or over an unrelated artifact.
    std::fs::create_dir_all(&spec.run_dir)?;
    spec.run_dir = spec.run_dir.canonicalize()?;
    let source_run = spec
        .source_checkpoint
        .parent()
        .context("source has no run")?;
    ensure!(
        !spec.run_dir.starts_with(source_run) && !source_run.starts_with(&spec.run_dir),
        "source and target run directories must be separate"
    );
    let migration_path = spec.run_dir.join("migration.json");
    if migration_path.exists() {
        let saved: NumericTransfer = read_json(&migration_path)?;
        ensure!(
            saved == transfer,
            "existing migration differs from this retraining spec"
        );
    } else {
        ensure!(
            std::fs::read_dir(&spec.run_dir)?.next().is_none(),
            "target directory contains unrelated artifacts"
        );
        write_json_atomic(&migration_path, &transfer)?;
    }
    let preparation_seconds = workflow_started.elapsed().as_secs_f64();
    let metadata = semantic_ppo::train_ppo_run(
        Arc::clone(&target),
        &spec.run_dir,
        PpoRunInput {
            config: ppo,
            init_bc_run: PathBuf::from(&parent.init_bc_run),
            init_critic_run: None,
            init_ppo_iteration: None,
            position_reference_actor: None,
            transfer: Some(transfer.clone()),
            iterations: spec.iterations,
            evaluate_every: spec.evaluate_every,
            development_seeds: spec.development_seeds,
        },
    )?;
    write_json_atomic(&spec.run_dir.join("retraining-spec.json"), &spec)?;
    let selected_iteration = metadata
        .history
        .iter()
        .filter_map(|record| record.evaluation.as_ref())
        .filter_map(|evaluation| {
            evaluation
                .summaries
                .iter()
                .find(|summary| summary.policy == "ppo")
                .map(|summary| {
                    (
                        evaluation.iteration,
                        summary.victories,
                        summary.mean_terminal_clear_rate,
                    )
                })
        })
        .max_by(|left, right| left.2.total_cmp(&right.2).then(left.0.cmp(&right.0)))
        .context("retraining has no completed development evaluation")?
        .0;
    let selected = spec.run_dir.join(format!("iter-{selected_iteration:04}"));
    let policies = [
        EvalPolicy::Canonical,
        EvalPolicy::Learned {
            name: "source".to_string(),
            policy: Box::new(load_transfer_policy(&spec.source_checkpoint)?),
        },
        EvalPolicy::Learned {
            name: "retrained".to_string(),
            policy: Box::new(SemanticPolicy::from_path(&selected)?),
        },
    ];
    let seeds = Phase4Split::PpoDevelopment.seeds(Some(spec.development_seeds))?;
    let evaluation = evaluate_policies(
        Arc::clone(&target),
        "retraining_development",
        &seeds,
        &policies,
        &[
            ("retrained".to_string(), "source".to_string()),
            ("retrained".to_string(), "canonical".to_string()),
        ],
    )?;
    let report = serde_json::json!({
        "schema_version": 1, "environment": metadata.environment,
        "source_checkpoint": transfer.source_checkpoint, "source_budget": transfer.source_budget,
        "post_change_budget": metadata.history.last().map(|record| record.cumulative).unwrap_or_default(),
        "completed_iterations": metadata.completed_iterations, "selected_iteration": selected_iteration,
        "selection_rule": "greatest development mean terminal progress; latest iteration breaks ties",
        "preparation_seconds": preparation_seconds,
        "workflow_invocation_seconds": workflow_started.elapsed().as_secs_f64(),
        "budget_scope": "post_change_budget includes rollout, optimizer, and in-training evaluation; preparation and report evaluation are separate",
        "evaluation": evaluation, "speedup_verified": false,
        "note": "Development result only; scratch controls and independent repeats are required to establish retraining efficiency."
    });
    write_json_atomic(&spec.run_dir.join("retraining-report.json"), &report)?;
    eprintln!(
        "completed {} retraining iterations; selected iteration {selected_iteration}; report {}",
        metadata.completed_iterations,
        spec.run_dir.join("retraining-report.json").display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_change_allows_weight_transfer_but_never_run_resume() {
        let config = GameConfig::default_config();
        let mut contract = EnvironmentContract::capture(&config);
        contract.game_rules_epoch += 1;
        contract.check_encoding().unwrap();
        assert!(contract.check_run(&config).is_err());
        contract
            .encoding_sources
            .insert("observation".into(), "changed".into());
        assert!(contract.check_encoding().is_err());
    }

    #[test]
    fn content_extension_preserves_existing_ids_and_rejects_remapping() {
        let source = content_catalog();
        let mut target = source.clone();
        target.get_mut("items").unwrap().push("FutureItem".into());
        validate_catalog_extension(&source, &target).unwrap();
        target.get_mut("items").unwrap().swap(0, 1);
        assert!(validate_catalog_extension(&source, &target).is_err());
        assert!(validate_catalog_extension(&target, &source).is_err());
    }

    #[test]
    fn numeric_transfer_detects_changes_and_rejects_identity_or_shape_changes() {
        let source = GameConfig::default_config();
        let mut target = source.clone();
        target.towers.entries[0].damage_raw += 1_000;
        target.player.starting_gold += 5;
        let changes = numeric_changes(&source, &target).unwrap();
        assert_eq!(changes.len(), 2);
        assert!(
            changes
                .iter()
                .any(|change| change.path.ends_with("/damage_raw"))
        );
        let duplicate = target.monsters.stage_waves[0].entries[0].clone();
        target.monsters.stage_waves[0].entries.push(duplicate);
        assert!(numeric_changes(&source, &target).is_err());
    }

    #[test]
    fn run_contract_rejects_changed_balance_and_encoder_versions() {
        let source = GameConfig::default_config();
        let mut contract = EnvironmentContract::capture(&source);
        contract.check_run(&source).unwrap();
        let mut target = source.clone();
        target.player.starting_gold += 1;
        assert!(
            contract
                .check_run(&target)
                .unwrap_err()
                .to_string()
                .contains("new retraining run")
        );
        contract.entity_encoder_version += 1;
        assert!(contract.check_encoding().is_err());
    }

    #[test]
    fn retraining_training_seeds_never_overlap_final_or_development_splits() {
        assert!(validate_training_seeds(5_000_000, 2, 2).is_ok());
        assert!(validate_training_seeds(4_200_000, 2, 2).is_err());
        assert!(validate_training_seeds(3_999_999, 2, 2).is_err());
        assert!(validate_training_seeds(u64::MAX, 2, 2).is_err());
    }
}
