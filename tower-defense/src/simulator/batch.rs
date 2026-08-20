//! Parallel episode execution for environment policies.

use super::environment::{
    AgentAction, EnvironmentError, GameEnvironment, LegalAction, Observation, StepOutcome,
};
use super::trajectory::{Trajectory, TrajectoryMetadata, TrajectoryStep};
use crate::config::GameConfig;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const DEFAULT_MAX_DECISIONS_PER_EPISODE: usize = 10_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchConfig {
    pub episode_count: usize,
    pub base_seed: u64,
    pub max_decisions_per_episode: usize,
    pub record_trajectories: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            episode_count: 1,
            base_seed: 0,
            max_decisions_per_episode: DEFAULT_MAX_DECISIONS_PER_EPISODE,
            record_trajectories: true,
        }
    }
}

pub trait EnvironmentPolicy: Send {
    fn choose_action(
        &mut self,
        observation: &Observation,
        legal_actions: &[LegalAction],
    ) -> AgentAction;
}

impl<F> EnvironmentPolicy for F
where
    F: FnMut(&Observation, &[LegalAction]) -> AgentAction + Send,
{
    fn choose_action(
        &mut self,
        observation: &Observation,
        legal_actions: &[LegalAction],
    ) -> AgentAction {
        self(observation, legal_actions)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchEpisodeResult {
    pub episode_index: usize,
    pub seed: u64,
    pub decision_count: usize,
    pub terminated: bool,
    pub truncated: bool,
    pub final_observation: Observation,
    pub final_state_hash: String,
    pub trajectory: Option<Trajectory>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRunReport {
    pub config: BatchConfig,
    pub episodes: Vec<BatchEpisodeResult>,
}

impl BatchRunReport {
    pub fn terminated_count(&self) -> usize {
        self.episodes
            .iter()
            .filter(|episode| episode.terminated)
            .count()
    }

    pub fn truncated_count(&self) -> usize {
        self.episodes
            .iter()
            .filter(|episode| episode.truncated)
            .count()
    }
}

#[derive(Debug)]
pub enum BatchError {
    Environment {
        episode_index: usize,
        error: EnvironmentError,
    },
    NoLegalActions {
        episode_index: usize,
        state_hash: String,
    },
}

pub fn episode_seed(base_seed: u64, episode_index: usize) -> u64 {
    let mut value =
        base_seed.wrapping_add((episode_index as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

pub fn run_batch<P, F>(
    game_config: Arc<GameConfig>,
    batch_config: BatchConfig,
    policy_factory: F,
) -> Result<BatchRunReport, BatchError>
where
    P: EnvironmentPolicy,
    F: Fn(u64) -> P + Sync,
{
    let episodes = (0..batch_config.episode_count)
        .into_par_iter()
        .map(|episode_index| {
            let seed = episode_seed(batch_config.base_seed, episode_index);
            run_episode::<P, F>(
                episode_index,
                seed,
                Arc::clone(&game_config),
                &batch_config,
                &policy_factory,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BatchRunReport {
        config: batch_config,
        episodes,
    })
}

fn run_episode<P, F>(
    episode_index: usize,
    seed: u64,
    game_config: Arc<GameConfig>,
    batch_config: &BatchConfig,
    policy_factory: &F,
) -> Result<BatchEpisodeResult, BatchError>
where
    P: EnvironmentPolicy,
    F: Fn(u64) -> P,
{
    let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
    environment.set_max_advance_ticks(
        (batch_config.max_decisions_per_episode.max(1) as u64)
            .saturating_mul(super::environment::DEFAULT_MAX_ADVANCE_TICKS),
    );
    let mut policy = policy_factory(seed);
    let mut trajectory = batch_config
        .record_trajectories
        .then(|| Trajectory::new(TrajectoryMetadata::new(&game_config, seed)));
    let mut decision_count = 0;
    let mut terminated = false;
    let mut truncated = false;

    while decision_count < batch_config.max_decisions_per_episode {
        if matches!(
            environment.decision_point(),
            super::environment::DecisionPoint::Terminal
        ) {
            terminated = true;
            break;
        }

        let pre_observation = environment.snapshot();
        let legal_actions = environment.legal_actions();
        if legal_actions.is_empty() {
            return Err(BatchError::NoLegalActions {
                episode_index,
                state_hash: environment.state_hash(),
            });
        }
        let action_mask = environment.action_mask();
        let action = policy.choose_action(&pre_observation, &legal_actions);
        let outcome =
            environment
                .step(action.clone())
                .map_err(|error| BatchError::Environment {
                    episode_index,
                    error,
                })?;
        terminated = outcome.terminated;
        truncated = outcome.truncated;
        record_step(
            trajectory.as_mut(),
            pre_observation,
            legal_actions,
            action,
            action_mask,
            outcome,
        );
        decision_count += 1;
        if terminated || truncated {
            break;
        }
    }

    if !terminated && !truncated && decision_count >= batch_config.max_decisions_per_episode {
        truncated = true;
    }

    Ok(BatchEpisodeResult {
        episode_index,
        seed,
        decision_count,
        terminated,
        truncated,
        final_observation: environment.snapshot(),
        final_state_hash: environment.state_hash(),
        trajectory,
    })
}

fn record_step(
    trajectory: Option<&mut Trajectory>,
    pre_observation: Observation,
    legal_actions: Vec<LegalAction>,
    action: AgentAction,
    action_mask: Vec<bool>,
    outcome: StepOutcome,
) {
    if let Some(trajectory) = trajectory {
        trajectory.push(TrajectoryStep::from_step(
            pre_observation,
            legal_actions,
            action_mask,
            action,
            outcome,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;

    #[test]
    fn episode_seed_is_stable_and_distinct_for_adjacent_indices() {
        assert_eq!(episode_seed(7, 0), episode_seed(7, 0));
        assert_ne!(episode_seed(7, 0), episode_seed(7, 1));
    }

    #[test]
    fn batch_results_are_ordered_and_reproducible() {
        let config = Arc::new(GameConfig::default_config());
        let batch_config = BatchConfig {
            episode_count: 3,
            base_seed: 11,
            max_decisions_per_episode: 1,
            record_trajectories: true,
        };
        fn choose_action(_observation: &Observation, legal_actions: &[LegalAction]) -> AgentAction {
            legal_actions
                .iter()
                .find(|action| matches!(action.action, AgentAction::StartSelectingTower))
                .map(|action| action.action.clone())
                .unwrap_or_else(|| legal_actions[0].action.clone())
        }
        fn policy_factory(_seed: u64) -> fn(&Observation, &[LegalAction]) -> AgentAction {
            choose_action
        }

        let first = run_batch::<_, _>(Arc::clone(&config), batch_config.clone(), policy_factory)
            .expect("batch should run");
        let second =
            run_batch::<_, _>(config, batch_config, policy_factory).expect("batch should rerun");

        assert_eq!(
            first
                .episodes
                .iter()
                .map(|episode| episode.episode_index)
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        assert_eq!(first, second);
    }
}
