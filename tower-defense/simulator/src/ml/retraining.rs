//! Explicit checkpoint transfer to compatible content, rules and configuration changes.

use super::contract::MlContract;
use super::feature_contract::InputContract;
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

const LEGACY_ENCODER_HASHES: &[(&str, &str)] = &[
    (
        "contract.rs",
        "8a5d60564f9392a89b16b0d912dea1890435619919a0a27f0cfe81918361c27c",
    ),
    (
        "features.rs",
        "9bd7e218d5563d373c1eb813ccc0ed1baab4b9200627a7b1b72585787829449a",
    ),
    (
        "feature_contract.rs",
        "48baf83ea75b0cdf910c17f4be6cbd2d4512d5cd5e292fd6a6166c701a6c301a",
    ),
    (
        "model.rs",
        "9d8acc12407a3d24b208804f32e7c23938030aabdc36eee55a6d51a7067bfc95",
    ),
    (
        "policy_v2.rs",
        "368f3f693dd2c62cdcc5e963b223528d73367e69cab7f637386eb0b4c42c2dd6",
    ),
    (
        "semantic_bc.rs",
        "1a6475e973208e7bd56169b7613fcedf64981f0605e6e167dbc8c355e01ef423",
    ),
    (
        "semantic_candidates.rs",
        "9a5821cd078bfb8d24032c0de30a46995c22f9c6976296542af2c251058a4340",
    ),
    (
        "spatial.rs",
        "7923a637caf1111347f349909b3ec52c9db48313de55316660ed894c6f34cd84",
    ),
    (
        "vocabulary.rs",
        "083ab3186b2f20274789cffc1e13e096f544098a9f2fc3a0085abc2b3f3aea25",
    ),
    (
        "encoding/mod.rs",
        "1c62e3e9b3feaf5eaca61e92991c337305e9efc231aedc456e3077186b50f78e",
    ),
    (
        "encoding/entity.rs",
        "3ad25fd83a4ebe02701e7ba3419443d900baa17490e67ebcae6fed5d7b3853c3",
    ),
    (
        "encoding/observation.rs",
        "e8c344802538d58db5b11d54a702be7876e86fa8ddf1156f199d73f059d49d83",
    ),
    (
        "encoding/normalize.rs",
        "0ac20eb35cc581cf5803b80b8ec3f7a927abc6f2a44ec7707299a868fec5f411",
    ),
    (
        "encoding/batch.rs",
        "ad2ec02adeae078f912ad354710f64f322bb5860ae43a53e716c23e222dfb70c",
    ),
    (
        "encoding/tensor.rs",
        "99c9a16ce222fb286e358867ff72f1897667832b6c93036049ab6942c207b886",
    ),
];

// Exact schema-11 manifest before PR-only lint cleanup. The changed policy
// expressions and effect test imports preserve encoding/model semantics.
const PRE_PR_SCHEMA11_SOURCES: &[(&str, &str)] = &[
    (
        "candidate",
        "b902db193542d9085d706aceee3b5e8811162e7d5c304c3c6eb303ccadde2abb",
    ),
    (
        "card_service",
        "2edfe5c47fc299fbedea11f78a5c9dc4b14e30588b2795326b4a353af01ef34b",
    ),
    (
        "effects",
        "cc7558acf0055064cc507376910c779e16dfb614d63f6755f6dd64db895bc0be",
    ),
    (
        "feature_contract",
        "2488ec9c859eeda35ce4518d3bdf481c4428a075ce2523252db1b62240e651fe",
    ),
    (
        "features",
        "cc5ff7fd77d2889b38dfba9986683d10f26f93a5a625969181e15e17f2b7fb35",
    ),
    (
        "model",
        "20ef417c858ba152dd673bee516a8080da6656204411ac07d6a91b4e52f76374",
    ),
    (
        "normalization",
        "0ac20eb35cc581cf5803b80b8ec3f7a927abc6f2a44ec7707299a868fec5f411",
    ),
    (
        "observation",
        "51059fca61218a4691a9223dad870a67adb8c09d81b46b3984309488ee189427",
    ),
    (
        "policy",
        "954aab282cf69e9e0577a7dda69bf2e49abf35ea0f8f6143bbaedb7bb14be7ba",
    ),
];

// Manifest written by the validated entity-6 / candidate-1 source-converted
// run. It captures the exact encoder implementation used before schema 7/2.
const ENTITY6_CANDIDATE1_SOURCES: &[(&str, &str)] = &[
    (
        "candidate",
        "fe84cd66c909ff41525fe1d30370a8628d1d0f830a9b9cdb04c1c2c8c946362c",
    ),
    (
        "effects",
        "11f120bf4c734f7872f0d51c276416b2774c4b58759327773011abcfb0000ff1",
    ),
    (
        "feature_contract",
        "2488ec9c859eeda35ce4518d3bdf481c4428a075ce2523252db1b62240e651fe",
    ),
    (
        "features",
        "cc5ff7fd77d2889b38dfba9986683d10f26f93a5a625969181e15e17f2b7fb35",
    ),
    (
        "model",
        "59e197688ab039152ab9fe0ded2f467a54fa39ec61ea6d3a648eabfc7d2e5938",
    ),
    (
        "normalization",
        "0ac20eb35cc581cf5803b80b8ec3f7a927abc6f2a44ec7707299a868fec5f411",
    ),
    (
        "observation",
        "594433b25f2ea972ad781e4ebfb1747ecb3030abc127dd91af151260bd9a5765",
    ),
    (
        "policy",
        "d95d02ca95d6d5e0f60b5f8b2578cf865c31d95a37b653dbdc072e30a1f1e86e",
    ),
];

fn is_entity6_candidate1_sources(sources: &std::collections::BTreeMap<String, String>) -> bool {
    sources.len() == ENTITY6_CANDIDATE1_SOURCES.len()
        && ENTITY6_CANDIDATE1_SOURCES
            .iter()
            .all(|(name, hash)| sources.get(*name).is_some_and(|actual| actual == hash))
}

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

    pub fn check_transfer_encoding(&self) -> Result<()> {
        if self.check_encoding().is_ok() {
            return Ok(());
        }
        if self.ml.observation_schema_version == 11
            && self.entity_encoder_version == 8
            && self.candidate_encoder_version == SEMANTIC_CANDIDATE_ENCODER_VERSION
            && self.policy_candidate_set_version == POLICY_CANDIDATE_SET_VERSION
            && self.encoding_sources.len() == PRE_PR_SCHEMA11_SOURCES.len()
            && PRE_PR_SCHEMA11_SOURCES.iter().all(|(key, hash)| {
                self.encoding_sources
                    .get(*key)
                    .is_some_and(|actual| actual == hash)
            })
        {
            let mut migrated = self.clone();
            migrated.encoding_sources = encoding_sources();
            migrated.check_encoding()?;
            return Ok(());
        }
        // Explicit side-path migration for the retained CardPolisher iteration
        // 2 contract. It verifies every source encoder digest from that
        // artifact before widening only the entity encoder to schema 8.
        const CARD_POLISHER_V7: [(&str, &str); 8] = [
            (
                "candidate",
                "b434979dd2f803bede266790f507d80b166ecb96e511cecc55d2e221b2e3c12d",
            ),
            (
                "effects",
                "cc7558acf0055064cc507376910c779e16dfb614d63f6755f6dd64db895bc0be",
            ),
            (
                "feature_contract",
                "2488ec9c859eeda35ce4518d3bdf481c4428a075ce2523252db1b62240e651fe",
            ),
            (
                "features",
                "cc5ff7fd77d2889b38dfba9986683d10f26f93a5a625969181e15e17f2b7fb35",
            ),
            (
                "model",
                "109c56dc0bf05508687ac5dbe2de4537d5ef78c5b67ba9d4a094ae687fb17c03",
            ),
            (
                "normalization",
                "0ac20eb35cc581cf5803b80b8ec3f7a927abc6f2a44ec7707299a868fec5f411",
            ),
            (
                "observation",
                "51059fca61218a4691a9223dad870a67adb8c09d81b46b3984309488ee189427",
            ),
            (
                "policy",
                "f0daef376920718e3ec9aa837786444fc9cedb3137a68f31f8705568f257e740",
            ),
        ];
        if self.ml.observation_schema_version == 10
            && self.entity_encoder_version == 7
            && self.candidate_encoder_version == SEMANTIC_CANDIDATE_ENCODER_VERSION
            && self.policy_candidate_set_version == POLICY_CANDIDATE_SET_VERSION
            && self.encoding_sources.len() == CARD_POLISHER_V7.len()
            && CARD_POLISHER_V7.iter().all(|(key, hash)| {
                self.encoding_sources
                    .get(*key)
                    .is_some_and(|actual| actual == hash)
            })
        {
            let mut migrated = self.clone();
            migrated.ml.observation_schema_version = td_core::OBSERVATION_SCHEMA_VERSION;
            migrated.entity_encoder_version = ENTITY_ENCODER_SCHEMA_VERSION;
            migrated.encoding_sources = encoding_sources();
            migrated.check_encoding()?;
            return Ok(());
        }
        if (self.ml.observation_schema_version == 10
            || self.ml.observation_schema_version == td_core::OBSERVATION_SCHEMA_VERSION)
            && self.entity_encoder_version == 6
            && self.candidate_encoder_version == 1
            && self.policy_candidate_set_version == POLICY_CANDIDATE_SET_VERSION
            && is_entity6_candidate1_sources(&self.encoding_sources)
        {
            let mut migrated = self.clone();
            migrated.ml.observation_schema_version = td_core::OBSERVATION_SCHEMA_VERSION;
            migrated.entity_encoder_version = ENTITY_ENCODER_SCHEMA_VERSION;
            migrated.candidate_encoder_version = SEMANTIC_CANDIDATE_ENCODER_VERSION;
            migrated.encoding_sources = encoding_sources();
            migrated.check_encoding()?;
            return Ok(());
        }
        ensure!(
            self.ml.observation_schema_version == 9,
            "unsupported source observation schema for effects migration"
        );
        let names = [
            ("features", "features.rs"),
            ("model", "model.rs"),
            ("policy", "policy_v2.rs"),
            ("candidate", "semantic_candidates.rs"),
            ("observation", "encoding/observation.rs"),
            ("normalization", "encoding/normalize.rs"),
            ("feature_contract", "feature_contract.rs"),
        ];
        ensure!(
            !self.encoding_sources.is_empty(),
            "historical source needs Git verification"
        );
        ensure!(
            self.encoding_sources.len() == names.len(),
            "unsupported legacy encoder manifest"
        );
        for (key, path) in names {
            let expected = LEGACY_ENCODER_HASHES
                .iter()
                .find(|(p, _)| *p == path)
                .unwrap()
                .1;
            ensure!(
                self.encoding_sources
                    .get(key)
                    .is_some_and(|hash| hash == expected),
                "unrecognized source encoder {key}"
            );
        }
        let mut migrated = self.clone();
        migrated.ml.observation_schema_version = td_core::OBSERVATION_SCHEMA_VERSION;
        migrated.entity_encoder_version = ENTITY_ENCODER_SCHEMA_VERSION;
        migrated.candidate_encoder_version = SEMANTIC_CANDIDATE_ENCODER_VERSION;
        migrated.encoding_sources = encoding_sources();
        migrated.check_encoding()
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
        let mapped_id = super::vocabulary::upgrade_key_id(&observation_key) as usize;
        ensure!(
            mapped_id == index + 1,
            "treasure {key}: observation key {observation_key} maps to ID {mapped_id}, expected {}; register its stable observation vocabulary ID before retraining",
            index + 1
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
        ("effects", include_str!("effect_inputs.rs")),
        ("card_service", include_str!("card_service_inputs.rs")),
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
        let known = LEGACY_ENCODER_HASHES
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, hash)| *hash);
        let source_hash = format!("{:x}", Sha256::digest(&output.stdout));
        ensure!(
            output.status.success()
                && (output.stdout == compiled.as_bytes()
                    || known == Some(source_hash.as_str())
                    || (path == "semantic_candidates.rs"
                        && source_hash
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
            (Value::Number(_), Value::Number(_)) | (Value::Bool(_), Value::Bool(_))
                if before != after =>
            {
                ensure!(
                    before.is_number() || path == "/treasures/black_white_enabled",
                    "unsupported rule switch at {path}"
                );
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
        value["treasures"] = serde_json::to_value(&config.treasures)?;
        Ok(value)
    };
    visit("", &normalized(source)?, &normalized(target)?, &mut changes)?;
    Ok(changes)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferInitialization {
    #[default]
    ReuseWeights,
    Scratch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericTransfer {
    #[serde(default)]
    pub content_teacher: Option<super::content_teacher::ContentTeacherSpec>,
    #[serde(default)]
    pub effect_inputs: bool,
    #[serde(default)]
    pub initialization: TransferInitialization,
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
    #[serde(default = "current_entity_encoder_version")]
    pub target_entity_encoder_version: u32,
    #[serde(default = "current_candidate_encoder_version")]
    pub target_candidate_encoder_version: u32,
    #[serde(default = "current_observation_schema_version")]
    pub target_observation_schema_version: u32,
    #[serde(default)]
    pub target_input_contract: InputContract,
    #[serde(default)]
    pub copied_parts: Vec<String>,
    #[serde(default)]
    pub initialized_parts: Vec<String>,
}

fn current_entity_encoder_version() -> u32 {
    ENTITY_ENCODER_SCHEMA_VERSION
}
fn current_candidate_encoder_version() -> u32 {
    SEMANTIC_CANDIDATE_ENCODER_VERSION
}
fn current_observation_schema_version() -> u32 {
    td_core::OBSERVATION_SCHEMA_VERSION
}

impl NumericTransfer {
    pub fn validate(&self, target: &GameConfig) -> Result<()> {
        self.source_environment.check_transfer_encoding()?;
        if self.legacy_source_contract {
            verify_legacy_encoding(&self.source_git_commit)?;
        }
        ensure!(
            self.optimizer_policy == "fresh_adam",
            "unsupported optimizer transfer policy"
        );
        ensure!(
            self.target_entity_encoder_version == ENTITY_ENCODER_SCHEMA_VERSION
                && self.target_candidate_encoder_version == SEMANTIC_CANDIDATE_ENCODER_VERSION
                && self.target_observation_schema_version == td_core::OBSERVATION_SCHEMA_VERSION,
            "transfer target input contract is not the current supported schema"
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
                && (actor.input_contract == self.source_bc_metadata.config.input_contract
                    || actor.input_contract
                        == self.source_bc_metadata.config.input_contract.with_effects())
                && (self.target_input_contract == actor.input_contract
                    || (self.effect_inputs
                        && self.target_input_contract == actor.input_contract.with_effects())),
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
    #[serde(default)]
    pub content_teacher: Option<super::content_teacher::ContentTeacherSpec>,
    pub schema_version: u32,
    #[serde(default)]
    pub effect_inputs: bool,
    #[serde(default)]
    pub initialization: TransferInitialization,
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
    #[serde(default)]
    pub development_seed_start: Option<u64>,
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
                ensure!(
                    provenance.observation_schema_version == 9
                        && provenance.candidate_encoder_version == 1
                        && provenance.dataset_schema_version == 2
                        && provenance.catalog_schema_version == 1
                        && provenance.action_wire_schema_version == 4
                        && provenance.environment_action_schema_version == 8
                        && provenance.policy_candidate_set_version == POLICY_CANDIDATE_SET_VERSION,
                    "unsupported historical dataset provenance"
                );
                let mut compatible = provenance.clone();
                compatible.dataset_schema_version =
                    super::phase4_dataset::PHASE4_DATASET_SCHEMA_VERSION;
                compatible.game_rules_epoch = GAME_RULES_EPOCH;
                compatible.observation_schema_version = td_core::OBSERVATION_SCHEMA_VERSION;
                compatible.catalog_schema_version = super::contract::CATALOG_SCHEMA_VERSION;
                compatible.action_wire_schema_version = super::contract::ACTION_WIRE_SCHEMA_VERSION;
                compatible.environment_action_schema_version =
                    crate::environment::ACTION_SCHEMA_VERSION;
                verify_legacy_encoding(&source.git_commit)?;
                compatible.check_current(config)?;
            }

            let mut environment = EnvironmentContract::capture(config);
            environment.ml.observation_schema_version = 9;
            environment.entity_encoder_version = 6;
            environment.candidate_encoder_version = 1;
            environment.encoding_sources = [
                ("features", "features.rs"),
                ("model", "model.rs"),
                ("policy", "policy_v2.rs"),
                ("candidate", "semantic_candidates.rs"),
                ("observation", "encoding/observation.rs"),
                ("normalization", "encoding/normalize.rs"),
                ("feature_contract", "feature_contract.rs"),
            ]
            .into_iter()
            .map(|(key, path)| {
                let hash = LEGACY_ENCODER_HASHES
                    .iter()
                    .find(|(known, _)| *known == path)
                    .expect("legacy encoder hash is registered")
                    .1;
                (key.to_string(), hash.to_string())
            })
            .collect();
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
        content_teacher: None,
        effect_inputs: false,
        initialization: TransferInitialization::ReuseWeights,
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
        target_entity_encoder_version: ENTITY_ENCODER_SCHEMA_VERSION,
        target_candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
        target_observation_schema_version: td_core::OBSERVATION_SCHEMA_VERSION,
        target_input_contract: actor.input_contract,
        copied_parts: vec![
            "matching actor parameters (same-name, same-shape)".to_string(),
            "matching critic parameters (same-name, same-shape)".to_string(),
            "source input contract and candidate mode unless explicitly widened".to_string(),
            "learned effect and entity-migration adapter parameters".to_string(),
        ],
        initialized_parts: vec![
            "actor and critic Adam optimizers: fresh state".to_string(),
            "actor and critic card-service context residual adapters: zero initialized with stable IDs".to_string(),
            "Core observation 10/entity encoder 7 to observation 11/entity encoder 8: explicit allowlisted CardPolisher side-path migration".to_string(),
        ],
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
        Phase4Split::EffectAdaptationValidation,
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
    let development_seed_start = spec
        .development_seed_start
        .unwrap_or(*Phase4Split::PpoDevelopment.range().start());
    Phase4Split::PpoDevelopment
        .seeds_starting_at(development_seed_start, spec.development_seeds)?;
    spec.development_seed_start = Some(development_seed_start);
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
    transfer.content_teacher = spec.content_teacher.clone();
    transfer.effect_inputs = spec.effect_inputs;
    transfer.initialization = spec.initialization;
    let source_actor: PpoActorFile = read_json(&transfer.source_checkpoint.join("ppo-actor.json"))?;
    transfer.target_input_contract = if transfer.effect_inputs {
        source_actor.input_contract.with_effects()
    } else {
        source_actor.input_contract
    };
    if transfer.initialization == TransferInitialization::Scratch {
        transfer.copied_parts.clear();
        transfer.copied_parts.push(
            "source configuration, source checkpoint hashes, and transfer provenance".to_string(),
        );
        transfer.initialized_parts.insert(
            0,
            "actor and critic model weights: seeded scratch initialization".to_string(),
        );
    }
    if transfer.effect_inputs && !source_actor.input_contract.has_effects() {
        transfer.initialized_parts.insert(
            0,
            "actor and critic effect residual adapters: zero weights and bias".to_string(),
        );
    }
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
    if let Some(teacher) = &spec.content_teacher {
        ensure!(
            spec.effect_inputs
                && teacher.games_per_config > 0
                && teacher.scenario_count > 0
                && teacher.epochs > 0
                && teacher.learning_rate.is_finite()
                && teacher.learning_rate > 0.0,
            "invalid content teacher configuration"
        );
        validate_training_seeds(teacher.seed_start, teacher.games_per_config, 1)?;
        let teacher_end = teacher
            .seed_start
            .checked_add(u64::try_from(teacher.games_per_config)? - 1)
            .context("teacher seed overflow")?;
        let training_count = ppo
            .episodes_per_iteration
            .checked_mul(spec.iterations)
            .context("training budget overflow")?;
        let training_end = spec
            .train_seed_start
            .checked_add(u64::try_from(training_count)? - 1)
            .context("training seed overflow")?;
        ensure!(
            teacher_end < spec.train_seed_start || teacher.seed_start > training_end,
            "teacher and PPO seed ranges overlap"
        );
        for record in &parent.history {
            if record.iteration > 0 {
                ensure!(
                    teacher_end < record.train_seeds.0 || teacher.seed_start > record.train_seeds.1,
                    "teacher seeds overlap source training"
                );
            }
        }
    }
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
            development_seed_start,
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
    let seeds = Phase4Split::PpoDevelopment
        .seeds_starting_at(development_seed_start, spec.development_seeds)?;
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
    let content_teacher_report: Option<serde_json::Value> =
        if spec.run_dir.join("content-teacher-report.json").exists() {
            Some(read_json(
                &spec.run_dir.join("content-teacher-report.json"),
            )?)
        } else {
            None
        };
    let report = serde_json::json!({
        "schema_version": 1, "environment": metadata.environment,
        "initialization": transfer.initialization, "effect_inputs": transfer.effect_inputs, "content_teacher":content_teacher_report,
        "source_checkpoint": transfer.source_checkpoint, "source_budget": transfer.source_budget,
        "source_hashes": {"actor": transfer.source_actor_sha256, "critic": transfer.source_critic_sha256, "actor_metadata": transfer.source_actor_metadata_sha256},
        "source_contract": {"observation_schema": transfer.source_environment.ml.observation_schema_version, "entity_encoder": transfer.source_environment.entity_encoder_version, "candidate_encoder": transfer.source_environment.candidate_encoder_version, "candidate_mode": source_actor.candidate_mode, "input_contract": source_actor.input_contract},
        "target_contract": {"observation_schema": transfer.target_observation_schema_version, "entity_encoder": transfer.target_entity_encoder_version, "candidate_encoder": transfer.target_candidate_encoder_version, "candidate_mode": transfer.target_candidate_mode, "input_contract": transfer.target_input_contract},
        "copied_parts": transfer.copied_parts, "initialized_parts": transfer.initialized_parts,
        "post_change_budget": metadata.history.last().map(|record| record.cumulative).unwrap_or_default(),
        "completed_iterations": metadata.completed_iterations, "selected_iteration": selected_iteration,
        "selection_rule": "greatest development mean terminal progress; latest iteration breaks ties",
        "preparation_seconds": preparation_seconds,
        "workflow_invocation_seconds": workflow_started.elapsed().as_secs_f64(),
        "budget_scope": "post_change_budget includes PPO rollout, optimizer, and in-training evaluation; content_teacher reports its separate simulation/training costs; preparation and report evaluation are separate",
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
    use burn::{
        module::{Module, ModuleVisitor, Param, ParamId},
        tensor::Tensor,
    };

    fn params<B: burn::tensor::backend::Backend>(
        model: &super::super::model::DeepSetsActorCritic<B>,
    ) -> Vec<(ParamId, Vec<f32>)> {
        struct Snapshot<B: burn::tensor::backend::Backend> {
            values: Vec<(ParamId, Vec<f32>)>,
            marker: std::marker::PhantomData<B>,
        }
        impl<B: burn::tensor::backend::Backend> ModuleVisitor<B> for Snapshot<B> {
            fn visit_float<const D: usize>(&mut self, parameter: &Param<Tensor<B, D>>) {
                self.values.push((
                    parameter.id,
                    parameter.val().clone().into_data().to_vec::<f32>().unwrap(),
                ));
            }
        }
        let mut visitor = Snapshot::<B> {
            values: Vec::new(),
            marker: std::marker::PhantomData,
        };
        model.visit(&mut visitor);
        visitor.values
    }

    #[test]
    #[ignore = "requires the retained TrainingMelon source checkpoint"]
    fn existing_source_effect_and_migration_adapters_survive_transfer_materialization() {
        let checkpoint = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("artifacts/retraining/training-melon-20261013/run-final-freshdev/iter-0002");
        let device = super::super::model::default_policy_device();
        let metadata: PpoActorFile = read_json(&checkpoint.join("ppo-actor.json")).unwrap();
        let actor = semantic_ppo::load_transfer_actor_as::<super::super::model::InferenceBackend>(
            &checkpoint,
            &device,
        )
        .expect("load selected transfer source actor");
        let actor_before = params(&actor.scorer);
        let actor_materialized = actor
            .scorer
            .clone()
            .with_effect_adapters(metadata.model_config, &device);
        let actor_materialized =
            actor_materialized.with_migration_adapters(metadata.model_config, &device);
        assert_eq!(actor_before, params(&actor_materialized));

        let critic =
            crate::ml::semantic_bc::load_model_file::<super::super::model::InferenceBackend>(
                metadata.model_config,
                &checkpoint.join("critic.bin"),
                &device,
            )
            .expect("load source critic");
        let critic_before = params(&critic);
        let critic_materialized = critic
            .clone()
            .with_effect_adapters(metadata.model_config, &device)
            .with_migration_adapters(metadata.model_config, &device);
        assert_eq!(critic_before, params(&critic_materialized));

        let adapter_ids = [
            0x4546465354410000_u64,
            0x4546465354410001,
            0x45464643414e0000,
            0x45464643414e0001,
            0x4546465452450000,
            0x4546465452450001,
            0x4d49475253544100,
            0x4d49475253544101,
            0x4d49475243414e00,
            0x4d49475243414e01,
        ]
        .map(ParamId::from);
        let mut adapter_norms = Vec::new();
        for (index, id) in adapter_ids.into_iter().enumerate() {
            let actor_values = actor_before
                .iter()
                .find(|(actual, _)| *actual == id)
                .unwrap_or_else(|| panic!("source actor adapter {id:?} missing"));
            let critic_values = critic_before
                .iter()
                .find(|(actual, _)| *actual == id)
                .unwrap_or_else(|| panic!("source critic adapter {id:?} missing"));
            let norm =
                |values: &[f32]| values.iter().map(|value| value * value).sum::<f32>().sqrt();
            adapter_norms.push((index, norm(&actor_values.1), norm(&critic_values.1)));
        }
        println!(
            "SOURCE_ADAPTER_VALUE_NORMS={{\"index_actor_critic_l2\":{:?},\"all_values_preserved\":true}}",
            adapter_norms
        );
    }

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
        #[cfg(feature = "experimental-content")]
        {
            let mut treasure_source = source.clone();
            treasure_source.get_mut("treasures").unwrap().pop();
            let mut treasure_target = treasure_source.clone();
            treasure_target
                .get_mut("treasures")
                .unwrap()
                .push("TrainingMelon".into());
            validate_catalog_extension(&treasure_source, &treasure_target).unwrap();
            treasure_target.get_mut("treasures").unwrap().swap(0, 1);
            assert!(validate_catalog_extension(&treasure_source, &treasure_target).is_err());
            let mut service_source = source.clone();
            service_source.get_mut("card_services").unwrap().pop();
            let mut service_target = service_source.clone();
            service_target
                .get_mut("card_services")
                .unwrap()
                .push("card_polisher".into());
            validate_catalog_extension(&service_source, &service_target).unwrap();
            service_target.get_mut("card_services").unwrap().swap(0, 1);
            assert!(validate_catalog_extension(&service_source, &service_target).is_err());
        }
    }

    #[test]
    fn treasure_rule_switch_is_recorded_and_changes_config_digest() {
        let source = GameConfig::default_config();
        let mut target = source.clone();
        target.treasures.black_white_enabled = false;
        let changes = numeric_changes(&source, &target).unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "/treasures/black_white_enabled");
        assert_ne!(config_digest(&source), config_digest(&target));
        let parsed = GameConfig::from_jsonc_str(&target.to_jsonc_string().unwrap()).unwrap();
        assert_eq!(parsed, target);
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
    fn pre_pr_schema11_transfer_accepts_only_the_audited_lint_cleanup_manifest() {
        let config = GameConfig::default_config();
        let mut source = EnvironmentContract::capture(&config);
        source.encoding_sources = PRE_PR_SCHEMA11_SOURCES
            .iter()
            .map(|(key, hash)| (key.to_string(), hash.to_string()))
            .collect();
        assert!(source.check_encoding().is_err());
        source.check_transfer_encoding().unwrap();
        source
            .encoding_sources
            .insert("policy".into(), "unaudited-policy".into());
        assert!(source.check_transfer_encoding().is_err());
    }

    #[test]
    fn entity6_candidate1_contract_requires_the_exact_encoder_manifest() {
        let config = GameConfig::default_config();
        let mut source = EnvironmentContract::capture(&config);
        source.entity_encoder_version = 6;
        source.candidate_encoder_version = 1;
        source.encoding_sources = ENTITY6_CANDIDATE1_SOURCES
            .iter()
            .map(|(key, hash)| (key.to_string(), hash.to_string()))
            .collect();
        assert!(source.check_encoding().is_err());
        source.check_transfer_encoding().unwrap();

        source.encoding_sources.insert(
            "candidate".to_string(),
            "unknown-candidate-encoder".to_string(),
        );
        assert!(source.check_transfer_encoding().is_err());
        source.entity_encoder_version = 5;
        assert!(source.check_transfer_encoding().is_err());
    }

    #[test]
    fn available_entity6_candidate1_artifact_is_transfer_loadable() {
        let checkpoint = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "artifacts/retraining/effect-adaptation-isolated-20261011/source-converted/iter-0000",
        );
        if !checkpoint.exists() {
            return;
        }
        let actor: PpoActorFile = read_json(&checkpoint.join("ppo-actor.json")).unwrap();
        let environment = actor.environment.as_ref().unwrap();
        assert_eq!(environment.entity_encoder_version, 6);
        assert_eq!(environment.candidate_encoder_version, 1);
        assert_eq!(actor.candidate_mode, CandidateMode::Top8);
        let (transfer, _) = prepare_transfer(&checkpoint, None, &environment.game_config)
            .expect("validated schema-6/schema-1 checkpoint transfers to current inputs");
        assert!(!transfer.legacy_source_contract);
        let loaded = semantic_ppo::load_transfer_actor_as::<crate::ml::model::InferenceBackend>(
            &checkpoint,
            &crate::ml::model::default_policy_device(),
        )
        .expect("source actor loads through the explicit transfer loader");
        assert_eq!(loaded.inputs, actor.input_contract);
        assert!(
            semantic_ppo::load_ppo_actor_as::<crate::ml::model::InferenceBackend>(
                &checkpoint,
                &crate::ml::model::default_policy_device(),
            )
            .is_err(),
            "ordinary checkpoint loading remains strict"
        );
    }

    #[test]
    fn historical_observation9_entity6_checkpoint_uses_combined_explicit_migration() {
        let checkpoint = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("artifacts/phase4b/kl-epoch-transaction-r2-p2/iter-0200");
        if !checkpoint.exists() {
            return;
        }
        let converted = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "artifacts/retraining/effect-adaptation-isolated-20261011/source-converted/iter-0000/ppo-actor.json",
        );
        let converted_actor: PpoActorFile = read_json(&converted).unwrap();
        let config = converted_actor.environment.unwrap().game_config;
        let (transfer, _) = prepare_transfer(&checkpoint, Some(&config), &config)
            .expect("historical observation-9/entity-6/candidate-1 source is verified");
        assert!(transfer.legacy_source_contract);
        let loaded = semantic_ppo::load_transfer_actor_as::<crate::ml::model::InferenceBackend>(
            &checkpoint,
            &crate::ml::model::default_policy_device(),
        )
        .expect("historical actor loads via transfer-only compatibility");
        assert!(!loaded.inputs.has_effects());
    }

    #[test]
    fn available_effect_adapter_checkpoint_keeps_learned_adapter_and_adds_zero_path() {
        let checkpoint = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "artifacts/retraining/effect-adaptation-isolated-20261011/effect-reuse_weights-r1/iter-0004",
        );
        if !checkpoint.exists() {
            return;
        }
        let actor_file: PpoActorFile = read_json(&checkpoint.join("ppo-actor.json")).unwrap();
        let environment = actor_file.environment.as_ref().unwrap();
        assert_eq!(environment.entity_encoder_version, 6);
        assert_eq!(environment.candidate_encoder_version, 1);
        assert!(actor_file.input_contract.has_effects());
        prepare_transfer(&checkpoint, None, &environment.game_config)
            .expect("learned-effect schema-6/schema-1 checkpoint validates");
        let actor = semantic_ppo::load_transfer_actor_as::<crate::ml::model::InferenceBackend>(
            &checkpoint,
            &crate::ml::model::default_policy_device(),
        )
        .expect("learned effect adapter checkpoint loads");
        assert!(actor.scorer.has_effect_adapters());
        assert!(actor.scorer.has_migration_adapters());
    }

    #[test]
    fn retraining_training_seeds_never_overlap_final_or_development_splits() {
        assert!(validate_training_seeds(5_000_000, 2, 2).is_ok());
        assert!(validate_training_seeds(4_200_000, 2, 2).is_err());
        assert!(validate_training_seeds(3_999_999, 2, 2).is_err());
        assert!(validate_training_seeds(u64::MAX, 2, 2).is_err());
    }
}
