//! Serializable trajectories produced by [`super::environment::GameEnvironment`].

use super::environment::{
    AgentAction, ENVIRONMENT_VERSION, LegalAction, Observation, RewardComponents, StepInfo,
    StepOutcome, StepReason,
};
use crate::config::{GAME_CONFIG_VERSION, GameConfig, config_digest};
use crate::ml::contract::{
    ACTION_WIRE_SCHEMA_VERSION, CATALOG_SCHEMA_VERSION, CONFIG_DIGEST_VERSION, MlContract,
    OBSERVATION_SCHEMA_VERSION,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::{BufWriter, Write};
use td_core::{
    CORE_REPLAY_SCHEMA_VERSION as REPLAY_SCHEMA_VERSION,
    CORE_RNG_ALGORITHM_VERSION as RNG_ALGORITHM_VERSION,
};

pub const TRAJECTORY_SCHEMA_VERSION: u32 = 5;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryOutcome {
    pub victory: bool,
    pub clear_rate: f32,
    pub terminated: bool,
    pub truncated: bool,
    pub termination_reason: StepReason,
    pub final_stage: usize,
    pub episode_return: f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrajectoryMetadata {
    pub environment_version: u32,
    pub trajectory_schema_version: u32,
    pub replay_schema_version: u32,
    pub config_version: u32,
    #[serde(default)]
    pub config_digest_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
    #[serde(default)]
    pub observation_schema_version: u32,
    #[serde(default)]
    pub catalog_schema_version: u32,
    #[serde(default)]
    pub action_wire_schema_version: u32,
    pub seed: u64,
}

impl TrajectoryMetadata {
    pub fn new(config: &GameConfig, seed: u64) -> Self {
        Self {
            environment_version: ENVIRONMENT_VERSION,
            trajectory_schema_version: TRAJECTORY_SCHEMA_VERSION,
            replay_schema_version: REPLAY_SCHEMA_VERSION,
            config_version: GAME_CONFIG_VERSION,
            config_digest_version: CONFIG_DIGEST_VERSION,
            config_digest: config_digest(config),
            rng_algorithm_version: RNG_ALGORITHM_VERSION,
            observation_schema_version: OBSERVATION_SCHEMA_VERSION,
            catalog_schema_version: CATALOG_SCHEMA_VERSION,
            action_wire_schema_version: ACTION_WIRE_SCHEMA_VERSION,
            seed,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryStep {
    pub pre_observation: Observation,
    pub legal_actions: Vec<LegalAction>,
    pub action_mask: Vec<bool>,
    pub action: AgentAction,
    #[serde(default)]
    pub player_command: Option<td_core::PlayerCommand>,
    pub post_observation: Observation,
    pub reward: RewardComponents,
    pub terminated: bool,
    pub truncated: bool,
    pub info: StepInfo,
    #[serde(default)]
    pub pre_state_hash: String,
    pub state_hash: String,
}

impl TrajectoryStep {
    pub fn from_step(
        pre_observation: Observation,
        legal_actions: Vec<LegalAction>,
        action_mask: Vec<bool>,
        action: AgentAction,
        outcome: StepOutcome,
    ) -> Self {
        let player_command = action.to_player_command();
        Self {
            pre_observation,
            legal_actions,
            action_mask,
            action,
            player_command,
            post_observation: outcome.observation,
            reward: outcome.reward,
            terminated: outcome.terminated,
            truncated: outcome.truncated,
            info: outcome.info,
            pre_state_hash: String::new(),
            state_hash: outcome.state_hash,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trajectory {
    pub metadata: TrajectoryMetadata,
    pub steps: Vec<TrajectoryStep>,
    pub outcome: Option<TrajectoryOutcome>,
}

impl Trajectory {
    pub fn new(metadata: TrajectoryMetadata) -> Self {
        Self {
            metadata,
            steps: Vec::new(),
            outcome: None,
        }
    }

    pub fn push(&mut self, step: TrajectoryStep) {
        self.steps.push(step);
    }

    pub fn is_terminal(&self) -> bool {
        self.steps
            .last()
            .is_some_and(|step| step.terminated || step.truncated)
    }

    #[cfg(feature = "simulator")]
    pub fn from_policy_steps(
        config: &GameConfig,
        seed: u64,
        steps: &[crate::policy_runner::PolicyStep],
    ) -> Self {
        let mut trajectory = Self::new(TrajectoryMetadata::new(config, seed));
        for step in steps {
            trajectory.push(TrajectoryStep::from_step(
                step.observation.clone(),
                step.legal_actions.clone(),
                vec![true; step.legal_actions.len()],
                step.action.clone(),
                step.outcome.clone(),
            ));
        }
        trajectory
    }

    /// Rebuilds a trajectory from a core replay carrying a policy trace.
    ///
    /// The replay contains the authoritative config/checkpoints while each
    /// trace row keeps the policy-level action separate from the command that
    /// actually reached the core.
    pub fn from_core_replay(replay: &td_core::CoreReplay) -> Result<Self, TrajectoryError> {
        replay
            .validate()
            .map_err(|error| TrajectoryError::ReplayValidation(format!("{error:?}")))?;
        let trace = replay
            .policy_trace
            .as_ref()
            .ok_or(TrajectoryError::MissingPolicyTrace)?;
        trace
            .validate_against_replay(replay)
            .map_err(|error| TrajectoryError::ReplayValidation(format!("{error:?}")))?;

        let metadata = TrajectoryMetadata {
            environment_version: trace.environment_version,
            trajectory_schema_version: TRAJECTORY_SCHEMA_VERSION,
            replay_schema_version: replay.replay_schema_version,
            config_version: replay.config_version,
            config_digest_version: if replay.config_digest_version == 0 {
                CONFIG_DIGEST_VERSION
            } else {
                replay.config_digest_version
            },
            config_digest: replay.config_digest.clone(),
            rng_algorithm_version: replay.rng_algorithm_version,
            observation_schema_version: OBSERVATION_SCHEMA_VERSION,
            catalog_schema_version: CATALOG_SCHEMA_VERSION,
            action_wire_schema_version: ACTION_WIRE_SCHEMA_VERSION,
            seed: replay.seed,
        };
        let mut trajectory = Self::new(metadata);
        for step in &trace.steps {
            let legal_actions = step
                .legal_actions
                .iter()
                .cloned()
                .map(|action| LegalAction {
                    id: action.action_id(),
                    action,
                })
                .collect();
            trajectory.push(TrajectoryStep {
                pre_observation: step.pre_observation.clone(),
                legal_actions,
                action_mask: step.action_mask.clone(),
                action: step.agent_action.clone(),
                player_command: step.player_command.clone(),
                post_observation: step.post_observation.clone(),
                reward: step.reward.clone(),
                terminated: step.terminated,
                truncated: step.truncated,
                info: step.info.clone(),
                pre_state_hash: step.pre_state_hash.clone(),
                state_hash: step.post_state_hash.clone(),
            });
        }
        trajectory.migrate_legacy_metadata()?;
        Ok(trajectory)
    }

    pub fn set_outcome(&mut self, outcome: TrajectoryOutcome) {
        self.outcome = Some(outcome);
    }

    pub fn to_json(&self) -> Result<String, TrajectoryError> {
        serde_json::to_string(self).map_err(TrajectoryError::from)
    }

    pub fn from_json(json: &str) -> Result<Self, TrajectoryError> {
        let mut trajectory: Self = serde_json::from_str(json).map_err(TrajectoryError::from)?;
        trajectory.migrate_legacy_metadata()?;
        Ok(trajectory)
    }

    pub(crate) fn migrate_legacy_metadata(&mut self) -> Result<(), TrajectoryError> {
        if self.metadata.config_digest_version == 0 {
            self.metadata.config_digest_version = CONFIG_DIGEST_VERSION;
        } else if self.metadata.config_digest_version != CONFIG_DIGEST_VERSION {
            return Err(TrajectoryError::UnsupportedVersion(
                "config_digest_version",
                self.metadata.config_digest_version,
            ));
        }
        for observation in self
            .steps
            .iter_mut()
            .flat_map(|step| [&mut step.pre_observation, &mut step.post_observation])
        {
            MlContract::migrate_legacy_observation(observation)
                .map_err(TrajectoryError::ContractVersion)?;
        }
        for (field, value, current) in [
            (
                "observation_schema_version",
                &mut self.metadata.observation_schema_version,
                OBSERVATION_SCHEMA_VERSION,
            ),
            (
                "catalog_schema_version",
                &mut self.metadata.catalog_schema_version,
                CATALOG_SCHEMA_VERSION,
            ),
            (
                "action_wire_schema_version",
                &mut self.metadata.action_wire_schema_version,
                ACTION_WIRE_SCHEMA_VERSION,
            ),
        ] {
            if *value == 0 {
                *value = current;
            } else if *value != current {
                return Err(TrajectoryError::UnsupportedVersion(field, *value));
            }
        }
        Ok(())
    }
}

pub trait TrajectorySink {
    fn write_episode(&mut self, trajectory: &Trajectory) -> Result<(), TrajectoryError>;
}

#[derive(Default)]
pub struct InMemoryTrajectorySink {
    episodes: Vec<Trajectory>,
}

impl InMemoryTrajectorySink {
    pub fn episodes(&self) -> &[Trajectory] {
        &self.episodes
    }

    pub fn into_episodes(self) -> Vec<Trajectory> {
        self.episodes
    }
}

impl TrajectorySink for InMemoryTrajectorySink {
    fn write_episode(&mut self, trajectory: &Trajectory) -> Result<(), TrajectoryError> {
        self.episodes.push(trajectory.clone());
        Ok(())
    }
}

pub struct JsonlTrajectorySink<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> JsonlTrajectorySink<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }

    pub fn flush(&mut self) -> Result<(), TrajectoryError> {
        self.writer.flush().map_err(TrajectoryError::from)
    }

    pub fn into_inner(self) -> Result<W, TrajectoryError> {
        self.writer
            .into_inner()
            .map_err(|error| TrajectoryError::from(error.into_error()))
    }
}

impl<W: Write> TrajectorySink for JsonlTrajectorySink<W> {
    fn write_episode(&mut self, trajectory: &Trajectory) -> Result<(), TrajectoryError> {
        serde_json::to_writer(&mut self.writer, trajectory).map_err(TrajectoryError::from)?;
        self.writer
            .write_all(b"\n")
            .map_err(TrajectoryError::from)?;
        self.flush()
    }
}

#[derive(Debug)]
pub enum TrajectoryError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingPolicyTrace,
    ReplayValidation(String),
    UnsupportedVersion(&'static str, u32),
    ContractVersion(crate::ml::contract::MlContractError),
}

impl Display for TrajectoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "trajectory I/O failed: {error}"),
            Self::Json(error) => write!(formatter, "trajectory JSON failed: {error}"),
            Self::MissingPolicyTrace => write!(formatter, "core replay has no policy trace"),
            Self::ReplayValidation(error) => write!(formatter, "replay validation failed: {error}"),
            Self::UnsupportedVersion(field, version) => {
                write!(formatter, "unsupported trajectory {field} {version}")
            }
            Self::ContractVersion(error) => {
                write!(formatter, "trajectory metadata failed: {error}")
            }
        }
    }
}

impl std::error::Error for TrajectoryError {}

impl From<std::io::Error> for TrajectoryError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for TrajectoryError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::GameEnvironment;
    use std::io::Cursor;
    use std::sync::Arc;

    #[test]
    fn trajectory_json_round_trip_preserves_metadata_and_steps() {
        let config = GameConfig::default_config();
        let metadata = TrajectoryMetadata::new(&config, 19);
        let mut trajectory = Trajectory::new(metadata);
        let mut environment = GameEnvironment::new(Arc::new(config), 19);
        let pre_observation = environment.snapshot();
        let legal_actions = environment.legal_actions();
        let action = AgentAction::StartSelectingTower;
        let outcome = environment
            .step(action.clone())
            .expect("action should be legal");
        trajectory.push(TrajectoryStep::from_step(
            pre_observation,
            legal_actions.clone(),
            vec![true; legal_actions.len()],
            action,
            outcome,
        ));

        let decoded =
            Trajectory::from_json(&trajectory.to_json().expect("serialize")).expect("deserialize");

        assert_eq!(decoded, trajectory);
        assert_eq!(
            decoded.steps[0].legal_actions,
            trajectory.steps[0].legal_actions
        );
        assert_eq!(
            decoded.steps[0].action_mask,
            trajectory.steps[0].action_mask
        );
        assert_eq!(decoded.steps[0].action, trajectory.steps[0].action);
    }

    #[test]
    fn jsonl_sink_writes_one_json_object_per_episode() {
        let config = GameConfig::default_config();
        let trajectory = Trajectory::new(TrajectoryMetadata::new(&config, 23));
        let mut sink = JsonlTrajectorySink::new(Cursor::new(Vec::new()));

        sink.write_episode(&trajectory).expect("write episode");
        let output = sink.into_inner().expect("flush writer").into_inner();

        assert_eq!(output.iter().filter(|byte| **byte == b'\n').count(), 1);
        let decoded: Trajectory =
            serde_json::from_slice(&output[..output.len() - 1]).expect("decode JSONL row");
        assert_eq!(decoded, trajectory);
    }

    #[test]
    fn core_replay_policy_trace_rebuilds_deterministic_trajectory() {
        let config = GameConfig::default_config();
        let mut environment = GameEnvironment::new(Arc::new(config), 29);
        let action = AgentAction::StartSelectingTower;

        environment
            .step(action.clone())
            .expect("action should be legal");

        let replay = environment.core_replay();
        let replay_json = serde_json::to_string(&replay).expect("replay should serialize");
        let replay: td_core::CoreReplay =
            serde_json::from_str(&replay_json).expect("replay should deserialize");
        let trace = replay
            .policy_trace
            .as_ref()
            .expect("environment replay should carry policy trace");
        assert_eq!(trace.steps.len(), 1);
        assert_eq!(trace.steps[0].agent_action, action);
        assert_eq!(
            trace.steps[0].player_command,
            Some(td_core::PlayerCommand::StartSelectingTower)
        );

        let trajectory = Trajectory::from_core_replay(&replay).expect("trace should convert");
        assert_eq!(trajectory.steps.len(), 1);
        assert_eq!(
            trajectory.steps[0].state_hash,
            trace.steps[0].post_state_hash
        );
        assert_eq!(trajectory.metadata.config_digest, replay.config_digest);

        let replayed_environment =
            GameEnvironment::from_core_replay(&replay).expect("core replay should execute");
        assert_eq!(replayed_environment.state_hash(), environment.state_hash());
        assert_eq!(
            replayed_environment.policy_trace().steps[0].post_state_hash,
            trace.steps[0].post_state_hash
        );
    }
}
