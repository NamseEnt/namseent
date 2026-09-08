use super::contract::MlContract;
use super::curriculum::{CurriculumConfig, CurriculumState};
use super::model::{
    DeepSetsActorCritic, ModelConfig, PolicyDevice, TrainBackend, load_train_model,
    save_train_model,
};
use super::neural_checkpoint::current_git_revision;
use super::seed::TrainingSeedSchedule;
use crate::environment::RewardConfig;
use anyhow::{Context, Result};
use burn::module::AutodiffModule;
use burn::optim::Optimizer;
use burn::record::{BinFileRecorder, FullPrecisionSettings, Recorder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub const TRAINER_CHECKPOINT_SCHEMA_VERSION: u32 = 6;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrainerState {
    pub schema_version: u32,
    pub generation: u64,
    pub iteration: usize,
    pub optimizer_step: u64,
    pub next_train_seed_offset: usize,
    pub learning_rate: f64,
    pub best_validation_clear_rate: f64,
    pub best_validation_full_clear_count: usize,
    pub best_validation_truncated_count: usize,
    pub best_iteration: usize,
    pub git_revision: String,
    pub contract: MlContract,
    pub model_config: ModelConfig,
    pub reward_config: RewardConfig,
    pub seed_schedule: TrainingSeedSchedule,
    pub hyperparameters: BTreeMap<String, String>,
    pub model_file: String,
    pub best_model_file: String,
    pub optimizer_file: String,
    pub best_optimizer_file: String,
    pub curriculum_config: Option<CurriculumConfig>,
    pub curriculum_state: Option<CurriculumState>,
}

pub struct TrainerCheckpointStore {
    root: PathBuf,
}

type TrainerResume<O> = (
    TrainerState,
    DeepSetsActorCritic<TrainBackend>,
    DeepSetsActorCritic<TrainBackend>,
    O,
    O,
);

impl TrainerCheckpointStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(root.join("checkpoints"))?;
        fs::create_dir_all(root.join("tmp"))?;
        Ok(Self { root })
    }

    pub fn save<M, O>(
        &self,
        state: &TrainerState,
        model: &DeepSetsActorCritic<TrainBackend>,
        best_model: &DeepSetsActorCritic<TrainBackend>,
        optimizer: &O,
        best_optimizer: &O,
    ) -> Result<PathBuf>
    where
        M: AutodiffModule<TrainBackend>,
        O: Optimizer<M, TrainBackend>,
    {
        let mut generation_number = state.generation;
        let (temporary, final_dir) = loop {
            let generation = format!("iter-{:08}-gen-{:08}", state.iteration, generation_number);
            let temporary = self.root.join("tmp").join(&generation);
            let final_dir = self.root.join("checkpoints").join(&generation);
            if !temporary.exists() && !final_dir.exists() {
                break (temporary, final_dir);
            }
            generation_number = generation_number.saturating_add(1);
        };
        if temporary.exists() {
            fs::remove_dir_all(&temporary)?;
        }
        fs::create_dir_all(&temporary)?;

        let mut state = state.clone();
        state.generation = generation_number;
        state.model_file = "model.mpk".to_string();
        state.best_model_file = "best-model.mpk".to_string();
        state.optimizer_file = "optimizer.bin".to_string();
        state.best_optimizer_file = "best-optimizer.bin".to_string();
        save_train_model(model, &temporary.join(&state.model_file))?;
        save_train_model(best_model, &temporary.join(&state.best_model_file))?;
        BinFileRecorder::<FullPrecisionSettings>::default()
            .record(optimizer.to_record(), temporary.join(&state.optimizer_file))
            .context("failed to save optimizer record")?;
        BinFileRecorder::<FullPrecisionSettings>::default()
            .record(
                best_optimizer.to_record(),
                temporary.join(&state.best_optimizer_file),
            )
            .context("failed to save best optimizer record")?;
        fs::write(
            temporary.join("trainer.json"),
            serde_json::to_vec_pretty(&state)?,
        )?;
        fs::rename(&temporary, &final_dir)?;
        let latest_tmp = self.root.join("tmp").join("latest.json.tmp");
        fs::write(&latest_tmp, serde_json::to_vec_pretty(&state)?)?;
        fs::rename(latest_tmp, self.root.join("latest.json"))?;
        Ok(final_dir)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn load_latest<O, M>(
        &self,
        model_config: super::model::ModelConfig,
        expected_contract: &MlContract,
        expected_reward_config: &RewardConfig,
        expected_schedule: &TrainingSeedSchedule,
        expected_hyperparameters: &BTreeMap<String, String>,
        expected_curriculum: Option<&CurriculumConfig>,
        optimizer: O,
        device: &PolicyDevice,
    ) -> Result<Option<TrainerResume<O>>>
    where
        M: AutodiffModule<TrainBackend>,
        O: Optimizer<M, TrainBackend>,
    {
        let latest_path = self.root.join("latest.json");
        if !latest_path.exists() {
            return Ok(None);
        }
        let latest_bytes = fs::read(&latest_path)?;
        validate_trainer_checkpoint_header(&latest_bytes)?;
        let mut state: TrainerState = serde_json::from_slice(&latest_bytes)?;
        state
            .contract
            .migrate_legacy_metadata()
            .map_err(|error| anyhow::anyhow!("{error}"))?;
        if state.schema_version != TRAINER_CHECKPOINT_SCHEMA_VERSION {
            anyhow::bail!(
                "unsupported trainer checkpoint schema: expected {}, got {}",
                TRAINER_CHECKPOINT_SCHEMA_VERSION,
                state.schema_version
            );
        }
        validate_trainer_state_contract(
            &state,
            model_config,
            expected_contract,
            expected_reward_config,
            expected_schedule,
            expected_hyperparameters,
            expected_curriculum,
        )?;
        if state.model_file.is_empty()
            || state.best_model_file.is_empty()
            || state.optimizer_file.is_empty()
            || state.best_optimizer_file.is_empty()
        {
            anyhow::bail!("trainer checkpoint has incomplete artifact metadata");
        }
        let generation = format!("iter-{:08}-gen-{:08}", state.iteration, state.generation);
        let directory = self.root.join("checkpoints").join(generation);
        let model = load_train_model(model_config, &directory.join(&state.model_file), device)?;
        let best_model = load_train_model(
            model_config,
            &directory.join(&state.best_model_file),
            device,
        )?;
        let record = BinFileRecorder::<FullPrecisionSettings>::default()
            .load(directory.join(&state.optimizer_file), device)
            .context("failed to load optimizer record")?;
        let optimizer = optimizer.load_record(record);
        let best_record = BinFileRecorder::<FullPrecisionSettings>::default()
            .load(directory.join(&state.best_optimizer_file), device)
            .context("failed to load best optimizer record")?;
        let best_optimizer = optimizer.clone().load_record(best_record);
        Ok(Some((state, model, best_model, optimizer, best_optimizer)))
    }

    pub fn has_latest(&self) -> bool {
        self.root.join("latest.json").exists()
    }
}

fn validate_trainer_checkpoint_header(bytes: &[u8]) -> Result<()> {
    let value: Value =
        serde_json::from_slice(bytes).context("invalid trainer checkpoint header")?;
    let schema_version = value
        .get("schema_version")
        .and_then(Value::as_u64)
        .map(|version| version as u32)
        .ok_or_else(|| anyhow::anyhow!("trainer checkpoint header is missing schema version"))?;
    if schema_version != TRAINER_CHECKPOINT_SCHEMA_VERSION {
        anyhow::bail!(
            "unsupported trainer checkpoint schema: expected {}, got {}",
            TRAINER_CHECKPOINT_SCHEMA_VERSION,
            schema_version
        );
    }
    Ok(())
}

fn validate_trainer_state_contract(
    state: &TrainerState,
    expected_model_config: ModelConfig,
    expected_contract: &MlContract,
    expected_reward_config: &RewardConfig,
    expected_schedule: &TrainingSeedSchedule,
    expected_hyperparameters: &BTreeMap<String, String>,
    expected_curriculum: Option<&CurriculumConfig>,
) -> Result<()> {
    if state.seed_schedule != *expected_schedule {
        anyhow::bail!("trainer resume seed schedule does not match current configuration");
    }
    let mut state_contract = state.contract.clone();
    state_contract
        .migrate_legacy_metadata()
        .map_err(|error| anyhow::anyhow!("{error}"))?;
    if state_contract != *expected_contract {
        anyhow::bail!("trainer resume ML contract does not match current configuration");
    }
    if state.model_config != expected_model_config {
        anyhow::bail!("trainer resume model configuration does not match current configuration");
    }
    if state.reward_config != *expected_reward_config {
        anyhow::bail!("trainer resume reward configuration does not match current configuration");
    }
    if state.hyperparameters != *expected_hyperparameters {
        anyhow::bail!("trainer resume hyperparameters do not match current configuration");
    }
    if state.curriculum_config.as_ref() != expected_curriculum {
        anyhow::bail!(
            "trainer resume curriculum configuration does not match current configuration"
        );
    }
    Ok(())
}

pub fn new_trainer_state(
    schedule: TrainingSeedSchedule,
    hyperparameters: BTreeMap<String, String>,
) -> Result<TrainerState> {
    Ok(TrainerState {
        schema_version: TRAINER_CHECKPOINT_SCHEMA_VERSION,
        generation: 0,
        iteration: 0,
        optimizer_step: 0,
        next_train_seed_offset: 0,
        learning_rate: 0.0,
        best_validation_clear_rate: f64::NEG_INFINITY,
        best_validation_full_clear_count: 0,
        best_validation_truncated_count: usize::MAX,
        best_iteration: 0,
        git_revision: current_git_revision()?,
        contract: MlContract {
            contract_schema_version: crate::ml::contract::ML_CONTRACT_SCHEMA_VERSION,
            environment_version: 0,
            action_schema_version: 0,
            observation_schema_version: 0,
            catalog_schema_version: 0,
            action_wire_schema_version: 0,
            trajectory_schema_version: 0,
            dataset_schema_version: 0,
            feature_schema_version: 0,
            config_version: 0,
            config_digest_version: 0,
            config_digest: String::new(),
            rng_algorithm_version: 0,
        },
        model_config: ModelConfig::default(),
        reward_config: RewardConfig::default(),
        seed_schedule: schedule,
        hyperparameters,
        model_file: String::new(),
        best_model_file: String::new(),
        optimizer_file: String::new(),
        best_optimizer_file: String::new(),
        curriculum_config: None,
        curriculum_state: None,
    })
}

pub fn unix_timestamp_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::ml::curriculum::{CurriculumConfig, CurriculumState};
    use crate::ml::seed::SeedRange;
    use serde_json::json;

    #[test]
    fn trainer_state_round_trip_preserves_curriculum_contract() {
        let config = CurriculumConfig {
            final_max_stage: 3,
            ..CurriculumConfig::default()
        };
        let state = CurriculumState::new(&config).expect("valid curriculum");
        let seed_schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 1).expect("train range"),
            SeedRange::try_new(u64::MAX - 1, u64::MAX).expect("validation range"),
        )
        .expect("valid schedule");
        let original = TrainerState {
            schema_version: TRAINER_CHECKPOINT_SCHEMA_VERSION,
            generation: 2,
            iteration: 1,
            optimizer_step: 4,
            next_train_seed_offset: 0,
            learning_rate: 0.001,
            best_validation_clear_rate: 0.5,
            best_validation_full_clear_count: 1,
            best_validation_truncated_count: 0,
            best_iteration: 1,
            git_revision: "test".to_string(),
            contract: MlContract {
                contract_schema_version: crate::ml::contract::ML_CONTRACT_SCHEMA_VERSION,
                environment_version: 1,
                action_schema_version: 1,
                observation_schema_version: crate::ml::contract::OBSERVATION_SCHEMA_VERSION,
                catalog_schema_version: crate::ml::contract::CATALOG_SCHEMA_VERSION,
                action_wire_schema_version: crate::ml::contract::ACTION_WIRE_SCHEMA_VERSION,
                trajectory_schema_version: 1,
                dataset_schema_version: 1,
                feature_schema_version: 1,
                config_version: 1,
                config_digest_version: crate::ml::contract::CONFIG_DIGEST_VERSION,
                config_digest: "test".to_string(),
                rng_algorithm_version: 1,
            },
            model_config: ModelConfig::default(),
            reward_config: RewardConfig::default(),
            seed_schedule,
            hyperparameters: BTreeMap::new(),
            model_file: "model.mpk".to_string(),
            best_model_file: "best.mpk".to_string(),
            optimizer_file: "optimizer.bin".to_string(),
            best_optimizer_file: "best-optimizer.bin".to_string(),
            curriculum_config: Some(config),
            curriculum_state: Some(state),
        };
        let encoded = serde_json::to_vec(&original).expect("serialize trainer state");
        let decoded: TrainerState =
            serde_json::from_slice(&encoded).expect("deserialize trainer state");
        assert_eq!(decoded, original);
    }

    #[test]
    fn legacy_trainer_schema_is_rejected_before_artifact_metadata() {
        let bytes = serde_json::to_vec(&json!({
            "schema_version": 1,
            "model_file": "missing.mpk"
        }))
        .expect("serialize legacy fixture");

        let error = validate_trainer_checkpoint_header(&bytes).expect_err("legacy schema");
        assert!(
            error
                .to_string()
                .contains("unsupported trainer checkpoint schema")
        );
    }

    #[test]
    fn typed_contract_mismatch_is_rejected_before_model_load() {
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).expect("train range"),
            SeedRange::try_new(1, 1).expect("validation range"),
        )
        .expect("schedule");
        let config = GameConfig::default_config();
        let expected_contract = MlContract::from_config(&config);
        let state = new_trainer_state(schedule.clone(), BTreeMap::new()).expect("state");
        let error = validate_trainer_state_contract(
            &state,
            ModelConfig::default(),
            &expected_contract,
            &RewardConfig::default(),
            &schedule,
            &BTreeMap::new(),
            None,
        )
        .expect_err("zero contract must be rejected");
        assert!(error.to_string().contains("ML contract"));
    }

    #[test]
    fn reward_mismatch_is_rejected_before_model_load() {
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).expect("train range"),
            SeedRange::try_new(1, 1).expect("validation range"),
        )
        .expect("schedule");
        let config = GameConfig::default_config();
        let contract = MlContract::from_config(&config);
        let mut state = new_trainer_state(schedule.clone(), BTreeMap::new()).expect("state");
        state.contract = contract.clone();
        state.seed_schedule = schedule.clone();
        state.reward_config.no_progress_cycle_penalty = 0.25;
        let expected_reward = RewardConfig {
            no_progress_cycle_penalty: -0.25,
            ..RewardConfig::default()
        };
        let error = validate_trainer_state_contract(
            &state,
            ModelConfig::default(),
            &contract,
            &expected_reward,
            &schedule,
            &BTreeMap::new(),
            None,
        )
        .expect_err("reward mismatch must be rejected");
        assert!(error.to_string().contains("reward configuration"));
    }
}
