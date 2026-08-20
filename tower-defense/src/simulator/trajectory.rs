//! Serializable trajectories produced by [`super::environment::GameEnvironment`].

use super::environment::{
    AgentAction, ENVIRONMENT_VERSION, LegalAction, Observation, RewardComponents, StepInfo,
    StepOutcome,
};
use crate::config::{GAME_CONFIG_VERSION, GameConfig};
use crate::deterministic_rng::RNG_ALGORITHM_VERSION;
use crate::game_state::replay::{REPLAY_SCHEMA_VERSION, config_digest};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::{BufWriter, Write};

pub const TRAJECTORY_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrajectoryMetadata {
    pub environment_version: u32,
    pub trajectory_schema_version: u32,
    pub replay_schema_version: u32,
    pub config_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
    pub seed: u64,
}

impl TrajectoryMetadata {
    pub fn new(config: &GameConfig, seed: u64) -> Self {
        Self {
            environment_version: ENVIRONMENT_VERSION,
            trajectory_schema_version: TRAJECTORY_SCHEMA_VERSION,
            replay_schema_version: REPLAY_SCHEMA_VERSION,
            config_version: GAME_CONFIG_VERSION,
            config_digest: config_digest(config),
            rng_algorithm_version: RNG_ALGORITHM_VERSION,
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
    pub post_observation: Observation,
    pub reward: RewardComponents,
    pub terminated: bool,
    pub truncated: bool,
    pub info: StepInfo,
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
        Self {
            pre_observation,
            legal_actions,
            action_mask,
            action,
            post_observation: outcome.observation,
            reward: outcome.reward,
            terminated: outcome.terminated,
            truncated: outcome.truncated,
            info: outcome.info,
            state_hash: outcome.state_hash,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trajectory {
    pub metadata: TrajectoryMetadata,
    pub steps: Vec<TrajectoryStep>,
}

impl Trajectory {
    pub fn new(metadata: TrajectoryMetadata) -> Self {
        Self {
            metadata,
            steps: Vec::new(),
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

    pub fn to_json(&self) -> Result<String, TrajectoryError> {
        serde_json::to_string(self).map_err(TrajectoryError::from)
    }

    pub fn from_json(json: &str) -> Result<Self, TrajectoryError> {
        serde_json::from_str(json).map_err(TrajectoryError::from)
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
}

impl Display for TrajectoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "trajectory I/O failed: {error}"),
            Self::Json(error) => write!(formatter, "trajectory JSON failed: {error}"),
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
    use crate::simulator::environment::GameEnvironment;
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
}
