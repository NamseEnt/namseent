use crate::config::{GAME_CONFIG_VERSION, GameConfig};
use crate::deterministic_rng::RNG_ALGORITHM_VERSION;
use crate::simulator::environment::{ACTION_SCHEMA_VERSION, ENVIRONMENT_VERSION, Observation};
use crate::simulator::trajectory::{TRAJECTORY_SCHEMA_VERSION, Trajectory};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{Display, Formatter};

pub const DATASET_SCHEMA_VERSION: u32 = 5;
pub const FEATURE_SCHEMA_VERSION: u32 = 6;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlContract {
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub trajectory_schema_version: u32,
    pub dataset_schema_version: u32,
    pub feature_schema_version: u32,
    pub config_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
}

impl MlContract {
    pub fn from_config(config: &GameConfig) -> Self {
        Self {
            environment_version: ENVIRONMENT_VERSION,
            action_schema_version: ACTION_SCHEMA_VERSION,
            trajectory_schema_version: TRAJECTORY_SCHEMA_VERSION,
            dataset_schema_version: DATASET_SCHEMA_VERSION,
            feature_schema_version: FEATURE_SCHEMA_VERSION,
            config_version: GAME_CONFIG_VERSION,
            config_digest: config_digest(config),
            rng_algorithm_version: RNG_ALGORITHM_VERSION,
        }
    }

    pub fn validate_observation(&self, observation: &Observation) -> Result<(), MlContractError> {
        if observation.environment_version != self.environment_version {
            return Err(MlContractError::VersionMismatch {
                field: "environment_version",
                expected: self.environment_version,
                actual: observation.environment_version,
            });
        }
        if observation.action_schema_version != self.action_schema_version {
            return Err(MlContractError::VersionMismatch {
                field: "action_schema_version",
                expected: self.action_schema_version,
                actual: observation.action_schema_version,
            });
        }
        Ok(())
    }

    pub fn validate_trajectory(&self, trajectory: &Trajectory) -> Result<(), MlContractError> {
        let metadata = &trajectory.metadata;
        self.validate_version(
            "environment_version",
            self.environment_version,
            metadata.environment_version,
        )?;
        self.validate_version(
            "trajectory_schema_version",
            self.trajectory_schema_version,
            metadata.trajectory_schema_version,
        )?;
        self.validate_version(
            "config_version",
            self.config_version,
            metadata.config_version,
        )?;
        self.validate_version(
            "rng_algorithm_version",
            self.rng_algorithm_version,
            metadata.rng_algorithm_version,
        )?;
        if metadata.config_digest != self.config_digest {
            return Err(MlContractError::ConfigDigestMismatch {
                expected: self.config_digest.clone(),
                actual: metadata.config_digest.clone(),
            });
        }

        for (step_index, step) in trajectory.steps.iter().enumerate() {
            self.validate_observation(&step.pre_observation)?;
            self.validate_observation(&step.post_observation)?;
            if step.action_mask.len() != step.legal_actions.len() {
                return Err(MlContractError::ActionMaskLength {
                    step_index,
                    legal_actions: step.legal_actions.len(),
                    mask: step.action_mask.len(),
                });
            }
            let Some(action_index) = step
                .legal_actions
                .iter()
                .position(|legal| legal.action == step.action)
            else {
                return Err(MlContractError::ActionNotLegal { step_index });
            };
            if !step.action_mask[action_index] {
                return Err(MlContractError::ActionMasked { step_index });
            }
        }
        Ok(())
    }

    fn validate_version(
        &self,
        field: &'static str,
        expected: u32,
        actual: u32,
    ) -> Result<(), MlContractError> {
        (expected == actual)
            .then_some(())
            .ok_or(MlContractError::VersionMismatch {
                field,
                expected,
                actual,
            })
    }
}

pub fn config_digest(config: &GameConfig) -> String {
    let serialized = toml::to_string(config).expect("GameConfig serialization must succeed");
    let digest = Sha256::digest(serialized.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlContractError {
    VersionMismatch {
        field: &'static str,
        expected: u32,
        actual: u32,
    },
    ConfigDigestMismatch {
        expected: String,
        actual: String,
    },
    ActionMaskLength {
        step_index: usize,
        legal_actions: usize,
        mask: usize,
    },
    ActionNotLegal {
        step_index: usize,
    },
    ActionMasked {
        step_index: usize,
    },
}

impl Display for MlContractError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VersionMismatch {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "{field} mismatch: expected {expected}, got {actual}"
            ),
            Self::ConfigDigestMismatch { expected, actual } => {
                write!(
                    formatter,
                    "config digest mismatch: expected {expected}, got {actual}"
                )
            }
            Self::ActionMaskLength {
                step_index,
                legal_actions,
                mask,
            } => write!(
                formatter,
                "step {step_index} action mask has {mask} entries for {legal_actions} legal actions"
            ),
            Self::ActionNotLegal { step_index } => {
                write!(formatter, "step {step_index} action is not legal")
            }
            Self::ActionMasked { step_index } => {
                write!(formatter, "step {step_index} action is masked")
            }
        }
    }
}

impl std::error::Error for MlContractError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::simulator::environment::{AgentAction, GameEnvironment};
    use crate::simulator::policy_runner::run_scripted_oracle_trajectory;
    use crate::simulator::trajectory::{TrajectoryMetadata, TrajectoryStep};
    use std::sync::Arc;

    #[test]
    fn contract_accepts_a_valid_environment_step() {
        let config = GameConfig::default_config();
        let contract = MlContract::from_config(&config);
        let mut environment = GameEnvironment::new(Arc::new(config.clone()), 7);
        let pre_observation = environment.snapshot();
        let legal_actions = environment.legal_actions();
        let action = AgentAction::StartSelectingTower;
        let outcome = environment.step(action.clone()).expect("legal action");
        let mut trajectory = Trajectory::new(TrajectoryMetadata::new(&config, 7));
        trajectory.push(TrajectoryStep::from_step(
            pre_observation,
            legal_actions.clone(),
            vec![true; legal_actions.len()],
            action,
            outcome,
        ));

        contract
            .validate_trajectory(&trajectory)
            .expect("trajectory should match contract");
    }

    #[test]
    fn contract_rejects_version_and_mask_mismatches() {
        let config = GameConfig::default_config();
        let contract = MlContract::from_config(&config);
        let mut environment = GameEnvironment::new(Arc::new(config.clone()), 7);
        let mut observation = environment.snapshot();
        observation.environment_version += 1;
        assert!(matches!(
            contract.validate_observation(&observation),
            Err(MlContractError::VersionMismatch { .. })
        ));

        let pre_observation = environment.snapshot();
        let legal_actions = environment.legal_actions();
        let action = AgentAction::StartSelectingTower;
        let outcome = environment.step(action.clone()).expect("legal action");
        let mut trajectory = Trajectory::new(TrajectoryMetadata::new(&config, 7));
        trajectory.push(TrajectoryStep::from_step(
            pre_observation,
            legal_actions.clone(),
            vec![false; legal_actions.len()],
            action,
            outcome,
        ));
        assert!(matches!(
            contract.validate_trajectory(&trajectory),
            Err(MlContractError::ActionMasked { .. })
        ));
    }

    #[test]
    fn scripted_oracle_trajectory_passes_contract_and_json_round_trip() {
        let config = GameConfig::default_config();
        let trajectory = run_scripted_oracle_trajectory(Arc::new(config.clone()), 19)
            .expect("scripted trajectory");
        let contract = MlContract::from_config(&config);

        contract
            .validate_trajectory(&trajectory)
            .expect("scripted trajectory should match contract");
        let decoded =
            Trajectory::from_json(&trajectory.to_json().expect("serialize")).expect("deserialize");

        assert_eq!(decoded, trajectory);
        assert!(!trajectory.steps.is_empty());
        assert!(trajectory.steps.iter().all(|step| {
            step.legal_actions
                .iter()
                .position(|legal| legal.action == step.action)
                .is_some_and(|index| step.action_mask[index])
        }));
    }
}
