use super::contract::MlContract;
use super::model::{
    DeepSetsActorCritic, ENTITY_ENCODER_SCHEMA_VERSION, InferenceBackend, ModelConfig,
    TrainBackend, default_policy_device, load_inference_model, load_train_model, save_train_model,
};
use super::seed::TrainingSeedSchedule;
use crate::environment::RewardConfig;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

pub const NEURAL_CHECKPOINT_SCHEMA_VERSION: u32 = 5;
pub const POLICY_SCHEMA_VERSION: u32 = 5;

pub fn current_git_revision() -> Result<String> {
    let output = std::process::Command::new("git")
        .args(["-C", env!("CARGO_MANIFEST_DIR"), "rev-parse", "HEAD"])
        .output()
        .context("failed to invoke git for checkpoint metadata")?;
    if !output.status.success() {
        bail!("git rev-parse HEAD failed");
    }
    let revision = String::from_utf8(output.stdout).context("git revision was not UTF-8")?;
    let revision = revision.trim().to_string();
    if revision.is_empty() {
        bail!("git revision is empty");
    }
    Ok(revision)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeuralCheckpoint {
    pub checkpoint_schema_version: u32,
    pub policy_schema_version: u32,
    #[serde(default)]
    pub entity_encoder_schema_version: u32,
    pub contract: MlContract,
    pub seed_schedule: TrainingSeedSchedule,
    pub iteration: usize,
    pub best_iteration: usize,
    pub git_revision: String,
    pub model_config: ModelConfig,
    pub model_file: String,
    pub train_clear_rate: f64,
    pub validation_clear_rate: f64,
    pub best_validation_clear_rate: f64,
    pub best_validation_full_clear_count: usize,
    pub best_validation_truncated_count: usize,
    #[serde(default)]
    pub reward_config: RewardConfig,
    pub hyperparameters: BTreeMap<String, String>,
}

impl NeuralCheckpoint {
    pub fn load_metadata(
        path: &Path,
        expected_contract: &MlContract,
        expected_schedule: &TrainingSeedSchedule,
    ) -> Result<Self> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read neural checkpoint {}", path.display()))?;
        validate_neural_checkpoint_header(&bytes)?;
        let checkpoint = decode_neural_checkpoint(&bytes)
            .with_context(|| format!("invalid neural checkpoint {}", path.display()))?;
        checkpoint.validate(expected_contract, expected_schedule)?;
        Ok(checkpoint)
    }

    pub fn validate(
        &self,
        expected_contract: &MlContract,
        expected_schedule: &TrainingSeedSchedule,
    ) -> Result<()> {
        self.validate_for_inference(expected_contract)?;
        if &self.seed_schedule != expected_schedule {
            bail!("neural checkpoint seed schedule does not match the requested ranges");
        }
        Ok(())
    }

    pub fn validate_for_inference(&self, expected_contract: &MlContract) -> Result<()> {
        self.validate_for_inference_with_config_change(expected_contract, false)
    }

    pub fn validate_for_inference_with_config_change(
        &self,
        expected_contract: &MlContract,
        allow_config_change: bool,
    ) -> Result<()> {
        self.reward_config.validate().map_err(anyhow::Error::msg)?;
        if self.checkpoint_schema_version != NEURAL_CHECKPOINT_SCHEMA_VERSION {
            bail!(
                "unsupported neural checkpoint schema {}",
                self.checkpoint_schema_version
            );
        }
        if self.policy_schema_version != POLICY_SCHEMA_VERSION {
            bail!("unsupported policy schema {}", self.policy_schema_version);
        }
        if self.entity_encoder_schema_version != ENTITY_ENCODER_SCHEMA_VERSION {
            bail!(
                "unsupported entity encoder schema {}",
                self.entity_encoder_schema_version
            );
        }
        if !self.reward_config.terminal_win.is_finite()
            || !self.reward_config.terminal_loss.is_finite()
            || !self.reward_config.escaped_hp_penalty_scale.is_finite()
            || !self.reward_config.player_hp_loss_penalty_scale.is_finite()
            || !self.reward_config.potential_weight.is_finite()
            || !self.reward_config.potential_gamma.is_finite()
            || !self.reward_config.no_progress_cycle_penalty.is_finite()
        {
            bail!("neural checkpoint reward config is not finite");
        }
        let mut checkpoint_contract = self.contract.clone();
        checkpoint_contract
            .migrate_legacy_metadata()
            .map_err(|error| anyhow::anyhow!("{error}"))?;
        let config_digest_matches =
            checkpoint_contract.config_digest == expected_contract.config_digest;
        checkpoint_contract.config_digest = expected_contract.config_digest.clone();
        if checkpoint_contract != *expected_contract {
            bail!("neural checkpoint ML contract does not match the environment");
        }
        if !config_digest_matches && !allow_config_change {
            bail!("neural checkpoint config digest does not match the environment");
        }
        if self.git_revision.trim().is_empty() {
            bail!("neural checkpoint is missing git revision");
        }
        if self.model_config.hidden_size < 8 {
            bail!("neural checkpoint hidden size is invalid");
        }
        if !self.validation_clear_rate.is_finite() || !self.best_validation_clear_rate.is_finite() {
            bail!("neural checkpoint clear rates are not finite");
        }
        Ok(())
    }

    pub fn load_with_inference_model(
        path: &Path,
        expected_contract: &MlContract,
    ) -> Result<(Self, DeepSetsActorCritic<InferenceBackend>)> {
        Self::load_with_inference_model_with_config_change(path, expected_contract, false)
    }

    pub fn load_with_inference_model_with_config_change(
        path: &Path,
        expected_contract: &MlContract,
        allow_config_change: bool,
    ) -> Result<(Self, DeepSetsActorCritic<InferenceBackend>)> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read neural checkpoint {}", path.display()))?;
        validate_neural_checkpoint_header(&bytes)?;
        let checkpoint = decode_neural_checkpoint(&bytes)
            .with_context(|| format!("invalid neural checkpoint {}", path.display()))?;
        checkpoint
            .validate_for_inference_with_config_change(expected_contract, allow_config_change)?;
        let model_path = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(&checkpoint.model_file);
        let device = default_policy_device();
        let model = load_inference_model(checkpoint.model_config, &model_path, &device)?;
        Ok((checkpoint, model))
    }

    pub fn save_with_model(
        &self,
        model: &DeepSetsActorCritic<TrainBackend>,
        path: &Path,
    ) -> Result<()> {
        let model_path = path.with_extension("mpk");
        save_train_model(model, &model_path)?;
        let mut checkpoint = self.clone();
        checkpoint.model_file = model_path
            .file_name()
            .context("checkpoint model path has no file name")?
            .to_string_lossy()
            .into_owned();
        let bytes = serde_json::to_vec_pretty(&checkpoint)?;
        std::fs::write(path, bytes)
            .with_context(|| format!("failed to write neural checkpoint {}", path.display()))?;
        Ok(())
    }

    pub fn load_with_model(
        path: &Path,
        expected_contract: &MlContract,
        expected_schedule: &TrainingSeedSchedule,
    ) -> Result<(Self, DeepSetsActorCritic<TrainBackend>)> {
        let checkpoint = Self::load_metadata(path, expected_contract, expected_schedule)?;
        let model_path = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(&checkpoint.model_file);
        let device = default_policy_device();
        let model = load_train_model(checkpoint.model_config, &model_path, &device)?;
        Ok((checkpoint, model))
    }

    pub fn load_for_initialization(
        path: &Path,
        expected_contract: &MlContract,
    ) -> Result<(Self, DeepSetsActorCritic<TrainBackend>)> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read neural checkpoint {}", path.display()))?;
        validate_neural_checkpoint_header(&bytes)?;
        let checkpoint = decode_neural_checkpoint(&bytes)
            .with_context(|| format!("invalid neural checkpoint {}", path.display()))?;
        checkpoint.validate_for_inference(expected_contract)?;
        let model_path = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(&checkpoint.model_file);
        let device = default_policy_device();
        let model = load_train_model(checkpoint.model_config, &model_path, &device)?;
        Ok((checkpoint, model))
    }
}

fn validate_neural_checkpoint_header(bytes: &[u8]) -> Result<()> {
    let value: Value = serde_json::from_slice(bytes).context("invalid neural checkpoint header")?;
    let schema_version = value
        .get("checkpoint_schema_version")
        .and_then(Value::as_u64)
        .map(|version| version as u32)
        .ok_or_else(|| anyhow::anyhow!("neural checkpoint header is missing schema version"))?;
    if schema_version != NEURAL_CHECKPOINT_SCHEMA_VERSION {
        bail!("unsupported neural checkpoint schema {}", schema_version);
    }

    let policy_schema_version = value
        .get("policy_schema_version")
        .and_then(Value::as_u64)
        .map(|version| version as u32)
        .ok_or_else(|| {
            anyhow::anyhow!("neural checkpoint header is missing policy schema version")
        })?;
    if policy_schema_version != POLICY_SCHEMA_VERSION {
        bail!("unsupported policy schema {}", policy_schema_version);
    }
    let entity_encoder_schema_version = value
        .get("entity_encoder_schema_version")
        .and_then(Value::as_u64)
        .map(|version| version as u32)
        .ok_or_else(|| {
            anyhow::anyhow!("neural checkpoint header is missing entity encoder schema version")
        })?;
    if entity_encoder_schema_version != ENTITY_ENCODER_SCHEMA_VERSION {
        bail!(
            "unsupported entity encoder schema {}",
            entity_encoder_schema_version
        );
    }
    Ok(())
}

fn decode_neural_checkpoint(bytes: &[u8]) -> Result<NeuralCheckpoint> {
    validate_neural_checkpoint_header(bytes)?;
    let mut checkpoint: NeuralCheckpoint =
        serde_json::from_slice(bytes).context("invalid neural checkpoint metadata")?;
    checkpoint
        .contract
        .migrate_legacy_metadata()
        .map_err(|error| anyhow::anyhow!("{error}"))?;
    Ok(checkpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::ml::seed::{SeedRange, TrainingSeedSchedule};
    use serde_json::json;

    fn checkpoint(contract: MlContract) -> NeuralCheckpoint {
        let train = SeedRange::try_new(0, 0).expect("valid train range");
        let validation = SeedRange::try_new(u64::MAX, u64::MAX).expect("valid validation range");
        NeuralCheckpoint {
            checkpoint_schema_version: NEURAL_CHECKPOINT_SCHEMA_VERSION,
            policy_schema_version: POLICY_SCHEMA_VERSION,
            entity_encoder_schema_version: ENTITY_ENCODER_SCHEMA_VERSION,
            contract,
            seed_schedule: TrainingSeedSchedule::try_new(train, validation)
                .expect("valid seed schedule"),
            iteration: 1,
            best_iteration: 1,
            git_revision: "test-revision".to_string(),
            model_config: ModelConfig::default(),
            model_file: "model.mpk".to_string(),
            train_clear_rate: 0.0,
            validation_clear_rate: 0.0,
            best_validation_clear_rate: 0.0,
            best_validation_full_clear_count: 0,
            best_validation_truncated_count: 0,
            reward_config: RewardConfig::default(),
            hyperparameters: BTreeMap::new(),
        }
    }

    #[test]
    fn inference_config_override_only_allows_config_digest_change() {
        let expected = MlContract::from_config(&GameConfig::default_config());
        let mut changed_config = checkpoint(expected.clone());
        changed_config.contract.config_digest = "different-config".to_string();

        assert!(changed_config.validate_for_inference(&expected).is_err());
        changed_config
            .validate_for_inference_with_config_change(&expected, true)
            .expect("explicit config override should be accepted");

        changed_config.contract.environment_version += 1;
        assert!(
            changed_config
                .validate_for_inference_with_config_change(&expected, true)
                .is_err()
        );
    }

    #[test]
    fn legacy_neural_schema_is_rejected_before_required_fields() {
        let bytes = serde_json::to_vec(&json!({
            "checkpoint_schema_version": 2,
            "model_file": "missing.mpk"
        }))
        .expect("serialize legacy fixture");

        let error = validate_neural_checkpoint_header(&bytes).expect_err("legacy schema");
        assert!(
            error
                .to_string()
                .contains("unsupported neural checkpoint schema 2")
        );
    }

    #[test]
    fn checkpoint_with_wrong_entity_encoder_schema_is_rejected() {
        let expected = MlContract::from_config(&GameConfig::default_config());
        let mut changed = checkpoint(expected.clone());
        changed.entity_encoder_schema_version += 1;
        assert!(changed.validate_for_inference(&expected).is_err());

        let mut bytes = serde_json::to_vec(&changed).expect("serialize checkpoint");
        let error = validate_neural_checkpoint_header(&bytes).expect_err("header mismatch");
        assert!(
            error
                .to_string()
                .contains("unsupported entity encoder schema")
        );

        let missing: Value = serde_json::from_slice(&bytes).expect("parse checkpoint json");
        let mut missing = missing;
        missing["entity_encoder_schema_version"] = Value::Null;
        bytes = serde_json::to_vec(&missing).expect("serialize modified checkpoint");
        let error = validate_neural_checkpoint_header(&bytes).expect_err("missing version");
        assert!(error.to_string().contains("entity encoder schema version"));
    }

    #[test]
    fn legacy_checkpoint_contract_metadata_is_migrated_on_read() {
        let contract = MlContract::from_config(&GameConfig::default_config());
        let checkpoint = checkpoint(contract.clone());
        let mut value = serde_json::to_value(checkpoint).expect("serialize checkpoint");
        let contract_value = value
            .get_mut("contract")
            .and_then(Value::as_object_mut)
            .expect("checkpoint contract object");
        for field in [
            "contract_schema_version",
            "observation_schema_version",
            "catalog_schema_version",
            "action_wire_schema_version",
            "config_digest_version",
        ] {
            contract_value.remove(field);
        }
        let decoded = decode_neural_checkpoint(
            &serde_json::to_vec(&value).expect("serialize legacy checkpoint"),
        )
        .expect("legacy checkpoint should migrate");
        assert_eq!(decoded.contract, contract);
    }

    #[test]
    fn reward_config_round_trip_preserves_non_default_values() {
        let mut value = checkpoint(MlContract::from_config(&GameConfig::default_config()));
        value.reward_config.potential_gamma = 0.9;
        value.reward_config.potential_weight = 0.25;
        let encoded = serde_json::to_vec(&value).expect("serialize checkpoint");
        let decoded: NeuralCheckpoint =
            serde_json::from_slice(&encoded).expect("deserialize checkpoint");
        assert_eq!(decoded.reward_config, value.reward_config);
    }

    #[test]
    fn metadata_validation_rejects_reward_mismatch_before_model_load() {
        let expected = MlContract::from_config(&GameConfig::default_config());
        let schedule = checkpoint(expected.clone()).seed_schedule;
        let mut value = checkpoint(expected.clone());
        value.reward_config.no_progress_cycle_penalty = -0.25;
        let path = std::env::temp_dir().join(format!(
            "tower-defense-neural-metadata-{}.json",
            std::process::id()
        ));
        std::fs::write(
            &path,
            serde_json::to_vec(&value).expect("serialize checkpoint"),
        )
        .expect("write checkpoint metadata");

        let metadata = NeuralCheckpoint::load_metadata(&path, &expected, &schedule)
            .expect("metadata validation should not load model weights");
        assert_eq!(metadata.reward_config.no_progress_cycle_penalty, -0.25);
        assert!(!path.with_extension("mpk").exists());
        let _ = std::fs::remove_file(path);
    }
}
