use crate::config::{GAME_CONFIG_VERSION, GameConfig};
use crate::environment::{ACTION_SCHEMA_VERSION, ENVIRONMENT_VERSION, Observation};
use crate::trajectory::{TRAJECTORY_SCHEMA_VERSION, Trajectory};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub const DATASET_SCHEMA_VERSION: u32 = 5;
pub const FEATURE_SCHEMA_VERSION: u32 = 11;
/// Bumped for `MlContract::balance_scope` - see
/// `docs/game-ai/03-observation-contract.md`'s "Balance configuration"
/// section.
pub const ML_CONTRACT_SCHEMA_VERSION: u32 = 2;
pub use crate::config::CONFIG_DIGEST_VERSION;
pub const OBSERVATION_SCHEMA_VERSION: u32 = td_core::OBSERVATION_SCHEMA_VERSION;
pub const CATALOG_SCHEMA_VERSION: u32 = td_core::CATALOG_SCHEMA_VERSION;
pub const ACTION_WIRE_SCHEMA_VERSION: u32 = td_core::ACTION_WIRE_SCHEMA_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlContract {
    #[serde(default)]
    pub contract_schema_version: u32,
    pub environment_version: u32,
    pub action_schema_version: u32,
    #[serde(default)]
    pub observation_schema_version: u32,
    #[serde(default)]
    pub catalog_schema_version: u32,
    #[serde(default)]
    pub action_wire_schema_version: u32,
    pub trajectory_schema_version: u32,
    pub dataset_schema_version: u32,
    pub feature_schema_version: u32,
    pub config_version: u32,
    #[serde(default)]
    pub config_digest_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
    /// Whether this checkpoint/contract's policy is trained/validated
    /// against a single fixed balance configuration (`config_digest`) or a
    /// balance-conditioned range. Phase 2 only ever produces `Fixed`; see
    /// `docs/game-ai/03-observation-contract.md`'s "Balance configuration"
    /// section.
    #[serde(default)]
    pub balance_scope: PolicyBalanceScope,
}

/// See [`MlContract::balance_scope`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyBalanceScope {
    /// The policy was trained and validated against exactly the balance
    /// configuration identified by `config_digest`. No balance parameter is
    /// randomized or exposed to the policy as input. Applying this
    /// checkpoint to a different configuration is out of its validated
    /// range - retrain or use a `Conditioned` policy instead.
    #[default]
    Fixed,
    /// Reserved for a future balance-conditioned policy: `parameters` names
    /// which config parameters the policy was trained over and the raw
    /// range for each. Not implemented in Phase 2 - nothing in this
    /// repository constructs this variant with real ranges yet, since doing
    /// so would assert a training/validation range that doesn't exist.
    Conditioned {
        parameters: Vec<ConditionedBalanceParameter>,
    },
}

/// A single balance-conditioned policy input parameter and the raw value
/// range it was trained/validated over. See [`PolicyBalanceScope::Conditioned`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionedBalanceParameter {
    pub name: String,
    pub min_raw: i64,
    pub max_raw: i64,
}

impl MlContract {
    pub fn from_config(config: &GameConfig) -> Self {
        Self {
            contract_schema_version: ML_CONTRACT_SCHEMA_VERSION,
            environment_version: ENVIRONMENT_VERSION,
            action_schema_version: ACTION_SCHEMA_VERSION,
            observation_schema_version: OBSERVATION_SCHEMA_VERSION,
            catalog_schema_version: CATALOG_SCHEMA_VERSION,
            action_wire_schema_version: ACTION_WIRE_SCHEMA_VERSION,
            trajectory_schema_version: TRAJECTORY_SCHEMA_VERSION,
            dataset_schema_version: DATASET_SCHEMA_VERSION,
            feature_schema_version: FEATURE_SCHEMA_VERSION,
            config_version: GAME_CONFIG_VERSION,
            config_digest_version: CONFIG_DIGEST_VERSION,
            config_digest: config_digest(config),
            rng_algorithm_version: td_core::CORE_RNG_ALGORITHM_VERSION,
            balance_scope: PolicyBalanceScope::Fixed,
        }
    }

    pub fn validate_observation(&self, observation: &Observation) -> Result<(), MlContractError> {
        Self::validate_compatibility_version(
            "observation_schema_version",
            OBSERVATION_SCHEMA_VERSION,
            observation.observation_schema_version,
        )?;
        Self::validate_compatibility_version(
            "catalog_schema_version",
            CATALOG_SCHEMA_VERSION,
            observation.catalog_schema_version,
        )?;
        Self::validate_compatibility_version(
            "action_wire_schema_version",
            ACTION_WIRE_SCHEMA_VERSION,
            observation.action_wire_schema_version,
        )?;
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
        self.validate_compatibility_versions()?;
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
        Self::validate_compatibility_version(
            "observation_schema_version",
            self.observation_schema_version,
            metadata.observation_schema_version,
        )?;
        Self::validate_compatibility_version(
            "catalog_schema_version",
            self.catalog_schema_version,
            metadata.catalog_schema_version,
        )?;
        Self::validate_compatibility_version(
            "action_wire_schema_version",
            self.action_wire_schema_version,
            metadata.action_wire_schema_version,
        )?;
        Self::validate_compatibility_version(
            "config_digest_version",
            self.config_digest_version,
            metadata.config_digest_version,
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

    pub fn migrate_legacy_metadata(&mut self) -> Result<(), MlContractError> {
        for (field, value, current) in [
            (
                "contract_schema_version",
                &mut self.contract_schema_version,
                ML_CONTRACT_SCHEMA_VERSION,
            ),
            (
                "observation_schema_version",
                &mut self.observation_schema_version,
                OBSERVATION_SCHEMA_VERSION,
            ),
            (
                "catalog_schema_version",
                &mut self.catalog_schema_version,
                CATALOG_SCHEMA_VERSION,
            ),
            (
                "action_wire_schema_version",
                &mut self.action_wire_schema_version,
                ACTION_WIRE_SCHEMA_VERSION,
            ),
            (
                "config_digest_version",
                &mut self.config_digest_version,
                CONFIG_DIGEST_VERSION,
            ),
        ] {
            if *value == 0 {
                *value = current;
            } else if *value != current {
                return Err(MlContractError::VersionMismatch {
                    field,
                    expected: current,
                    actual: *value,
                });
            }
        }
        Ok(())
    }

    pub fn migrate_legacy_observation(
        observation: &mut Observation,
    ) -> Result<(), MlContractError> {
        for (field, value, current) in [
            (
                "observation_schema_version",
                &mut observation.observation_schema_version,
                OBSERVATION_SCHEMA_VERSION,
            ),
            (
                "catalog_schema_version",
                &mut observation.catalog_schema_version,
                CATALOG_SCHEMA_VERSION,
            ),
            (
                "action_wire_schema_version",
                &mut observation.action_wire_schema_version,
                ACTION_WIRE_SCHEMA_VERSION,
            ),
        ] {
            if *value == 0 {
                *value = current;
            } else if *value != current {
                return Err(MlContractError::VersionMismatch {
                    field,
                    expected: current,
                    actual: *value,
                });
            }
        }
        Ok(())
    }

    fn validate_compatibility_versions(&self) -> Result<(), MlContractError> {
        for (field, actual, expected) in [
            (
                "contract_schema_version",
                self.contract_schema_version,
                ML_CONTRACT_SCHEMA_VERSION,
            ),
            (
                "observation_schema_version",
                self.observation_schema_version,
                OBSERVATION_SCHEMA_VERSION,
            ),
            (
                "catalog_schema_version",
                self.catalog_schema_version,
                CATALOG_SCHEMA_VERSION,
            ),
            (
                "action_wire_schema_version",
                self.action_wire_schema_version,
                ACTION_WIRE_SCHEMA_VERSION,
            ),
            (
                "config_digest_version",
                self.config_digest_version,
                CONFIG_DIGEST_VERSION,
            ),
        ] {
            Self::validate_compatibility_version(field, expected, actual)?;
        }
        Ok(())
    }

    fn validate_compatibility_version(
        field: &'static str,
        expected: u32,
        actual: u32,
    ) -> Result<(), MlContractError> {
        (actual == 0 || actual == expected)
            .then_some(())
            .ok_or(MlContractError::VersionMismatch {
                field,
                expected,
                actual,
            })
    }
}

pub fn config_digest(config: &GameConfig) -> String {
    crate::config::config_digest(config)
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
    use crate::environment::{AgentAction, GameEnvironment};
    use crate::policy_runner::run_scripted_oracle_trajectory;
    use crate::trajectory::{TrajectoryMetadata, TrajectoryStep};
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

    #[test]
    fn contract_metadata_round_trip_and_legacy_migration_are_explicit() {
        let config = GameConfig::default_config();
        let contract = MlContract::from_config(&config);
        let encoded = serde_json::to_value(&contract).expect("serialize contract");
        let decoded: MlContract = serde_json::from_value(encoded).expect("deserialize contract");
        assert_eq!(decoded, contract);

        let mut legacy = serde_json::to_value(&contract).expect("serialize contract");
        let object = legacy.as_object_mut().expect("contract object");
        for field in [
            "contract_schema_version",
            "observation_schema_version",
            "catalog_schema_version",
            "action_wire_schema_version",
            "config_digest_version",
        ] {
            object.remove(field);
        }
        let mut legacy: MlContract =
            serde_json::from_value(legacy).expect("deserialize legacy contract");
        legacy
            .migrate_legacy_metadata()
            .expect("legacy contract migration");
        assert_eq!(legacy, contract);
    }

    #[test]
    fn contract_from_config_is_fixed_balance_scope() {
        let config = GameConfig::default_config();
        let contract = MlContract::from_config(&config);
        assert_eq!(contract.balance_scope, PolicyBalanceScope::Fixed);
    }

    #[test]
    fn legacy_contract_without_balance_scope_field_defaults_to_fixed() {
        let config = GameConfig::default_config();
        let contract = MlContract::from_config(&config);
        let mut legacy = serde_json::to_value(&contract).expect("serialize contract");
        legacy
            .as_object_mut()
            .expect("contract object")
            .remove("balance_scope");

        let decoded: MlContract =
            serde_json::from_value(legacy).expect("deserialize legacy contract");

        assert_eq!(decoded.balance_scope, PolicyBalanceScope::Fixed);
        assert_eq!(decoded, contract);
    }

    #[test]
    fn conditioned_balance_scope_round_trips_through_json() {
        let scope = PolicyBalanceScope::Conditioned {
            parameters: vec![ConditionedBalanceParameter {
                name: "example_parameter".to_string(),
                min_raw: 0,
                max_raw: 1_000,
            }],
        };
        let encoded = serde_json::to_value(&scope).expect("serialize balance scope");
        let decoded: PolicyBalanceScope =
            serde_json::from_value(encoded).expect("deserialize balance scope");
        assert_eq!(decoded, scope);
        assert_ne!(decoded, PolicyBalanceScope::Fixed);
    }

    #[test]
    fn contract_rejects_unsupported_catalog_version() {
        let config = GameConfig::default_config();
        let contract = MlContract::from_config(&config);
        let environment = GameEnvironment::new(Arc::new(config), 7);
        let mut observation = environment.snapshot();
        observation.catalog_schema_version = CATALOG_SCHEMA_VERSION + 1;
        assert!(matches!(
            contract.validate_observation(&observation),
            Err(MlContractError::VersionMismatch {
                field: "catalog_schema_version",
                ..
            })
        ));
    }
}
