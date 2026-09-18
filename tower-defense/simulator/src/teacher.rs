use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::environment::{
    AgentAction, DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT, GameEnvironment, LegalAction,
    StepOutcome,
};
use crate::policy_runner::scripted_expert_action;

pub const TEACHER_SCORE_SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_TEACHER_HORIZON_DECISIONS: usize = 8;
pub const DEFAULT_TEACHER_SCENARIO_COUNT: usize = 16;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutTeacherConfig {
    pub scenario_seeds: Vec<u64>,
    pub horizon_decisions: usize,
    pub position_candidate_limit: Option<usize>,
    #[serde(default)]
    pub candidate_limit: Option<usize>,
}

impl Default for RolloutTeacherConfig {
    fn default() -> Self {
        Self {
            scenario_seeds: (0..DEFAULT_TEACHER_SCENARIO_COUNT as u64).collect(),
            horizon_decisions: DEFAULT_TEACHER_HORIZON_DECISIONS,
            position_candidate_limit: Some(DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT),
            candidate_limit: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutCandidateEstimate {
    pub action: AgentAction,
    pub action_id: String,
    pub sample_count: usize,
    pub mean_score: f32,
    pub variance: f32,
    pub standard_error: f32,
    pub wins: usize,
    pub mean_clear_rate: f32,
    pub mean_final_stage: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutTeacherDecision {
    pub score_schema_version: u32,
    pub state_hash: String,
    pub observation: crate::environment::Observation,
    pub scenario_seed_digest: String,
    pub horizon_decisions: usize,
    pub candidate_count: usize,
    pub selected_action_id: String,
    pub selected_mean_score: f32,
    pub baseline_action_id: Option<String>,
    pub baseline_mean_score: Option<f32>,
    pub expert_regret: Option<f32>,
    pub candidates: Vec<RolloutCandidateEstimate>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutTeacherEpisode {
    pub seed: u64,
    pub max_decisions: usize,
    pub decision_count: usize,
    pub terminated: bool,
    pub truncated: bool,
    pub victory: bool,
    pub final_state_hash: String,
    pub decisions: Vec<RolloutTeacherDecision>,
}

#[derive(Default)]
struct EstimateAccumulator {
    count: usize,
    mean: f32,
    squared_delta_sum: f32,
    wins: usize,
    clear_rate_sum: f32,
    stage_sum: f32,
}

impl EstimateAccumulator {
    fn record(&mut self, score: f32, victory: bool, clear_rate: f32, stage: usize) {
        self.count += 1;
        let delta = score - self.mean;
        self.mean += delta / self.count as f32;
        self.squared_delta_sum += delta * (score - self.mean);
        self.wins += victory as usize;
        self.clear_rate_sum += clear_rate;
        self.stage_sum += stage as f32;
    }

    fn finish(self, action: AgentAction) -> RolloutCandidateEstimate {
        let variance = if self.count > 1 {
            self.squared_delta_sum / self.count as f32
        } else {
            0.0
        };
        RolloutCandidateEstimate {
            action_id: action.action_id(),
            action,
            sample_count: self.count,
            mean_score: self.mean,
            variance,
            standard_error: if self.count > 0 {
                (variance / self.count as f32).sqrt()
            } else {
                0.0
            },
            wins: self.wins,
            mean_clear_rate: self.clear_rate_sum / self.count.max(1) as f32,
            mean_final_stage: self.stage_sum / self.count.max(1) as f32,
        }
    }
}

pub fn evaluate_semantic_candidates(
    environment: &GameEnvironment,
    config: &RolloutTeacherConfig,
) -> Result<RolloutTeacherDecision> {
    let mut candidates =
        environment.semantic_legal_actions_with_position_limit(config.position_candidate_limit);
    if let Some(candidate_limit) = config.candidate_limit {
        candidates.truncate(candidate_limit.max(1));
    }
    evaluate_semantic_candidate_set(environment, &candidates, config)
}

pub fn evaluate_semantic_candidate_set(
    environment: &GameEnvironment,
    candidates: &[LegalAction],
    config: &RolloutTeacherConfig,
) -> Result<RolloutTeacherDecision> {
    if config.scenario_seeds.is_empty() {
        bail!("rollout teacher requires at least one scenario seed");
    }
    if config.horizon_decisions == 0 {
        bail!("rollout teacher horizon must be positive");
    }
    if candidates.is_empty() {
        bail!("rollout teacher requires at least one candidate");
    }
    let legal_actions =
        environment.semantic_legal_actions_with_position_limit(config.position_candidate_limit);
    if candidates.iter().any(|candidate| {
        !legal_actions
            .iter()
            .any(|legal| legal.action == candidate.action)
    }) {
        bail!("rollout teacher candidate is not legal in the source environment");
    }
    let observation = environment.snapshot();
    let baseline_action = scripted_expert_action(&observation, &legal_actions)?;

    let mut estimates = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let mut accumulator = EstimateAccumulator::default();
        for &scenario_seed in &config.scenario_seeds {
            let sample =
                evaluate_candidate_scenario(environment, candidate, scenario_seed, config)?;
            accumulator.record(
                sample.score,
                sample.victory,
                sample.clear_rate,
                sample.final_stage,
            );
        }
        estimates.push(accumulator.finish(candidate.action.clone()));
    }
    let selected_action_id = estimates
        .iter()
        .max_by(|left, right| {
            left.mean_score
                .total_cmp(&right.mean_score)
                .then_with(|| right.action_id.cmp(&left.action_id))
        })
        .expect("teacher estimates are non-empty")
        .action_id
        .clone();
    let selected_mean_score = estimates
        .iter()
        .find(|candidate| candidate.action_id == selected_action_id)
        .expect("selected teacher action must have an estimate")
        .mean_score;
    let baseline_action_id = baseline_action.action_id();
    let baseline_mean_score = estimates
        .iter()
        .find(|candidate| candidate.action_id == baseline_action_id)
        .map(|candidate| candidate.mean_score);
    Ok(RolloutTeacherDecision {
        score_schema_version: TEACHER_SCORE_SCHEMA_VERSION,
        state_hash: environment.state_hash(),
        observation,
        scenario_seed_digest: scenario_seed_digest(&config.scenario_seeds),
        horizon_decisions: config.horizon_decisions,
        candidate_count: estimates.len(),
        selected_action_id,
        selected_mean_score,
        baseline_action_id: baseline_mean_score.map(|_| baseline_action_id),
        baseline_mean_score,
        expert_regret: baseline_mean_score.map(|score| selected_mean_score - score),
        candidates: estimates,
    })
}

pub fn run_semantic_teacher_episode(
    environment: &mut GameEnvironment,
    config: &RolloutTeacherConfig,
    max_decisions: usize,
) -> Result<RolloutTeacherEpisode> {
    if max_decisions == 0 {
        bail!("rollout teacher max decisions must be positive");
    }
    let mut decisions = Vec::new();
    let mut terminated = false;
    let mut truncated = false;
    while decisions.len() < max_decisions {
        if matches!(
            environment.decision_point(),
            crate::environment::DecisionPoint::Terminal
        ) {
            terminated = true;
            break;
        }
        let decision = evaluate_semantic_candidates(environment, config)?;
        let selected_action = decision
            .candidates
            .iter()
            .find(|candidate| candidate.action_id == decision.selected_action_id)
            .map(|candidate| candidate.action.clone())
            .expect("selected teacher action must be in estimates");
        let mut outcome = environment
            .semantic_step(selected_action)
            .map_err(|error| anyhow::anyhow!("teacher episode action failed: {error:?}"))?;
        settle_forced_actions(environment, &mut outcome)?;
        terminated = outcome.terminated;
        truncated = outcome.truncated;
        decisions.push(decision);
        if terminated || truncated {
            break;
        }
    }
    if !terminated && !truncated && decisions.len() == max_decisions {
        truncated = true;
    }
    Ok(RolloutTeacherEpisode {
        seed: environment.seed(),
        max_decisions,
        decision_count: decisions.len(),
        terminated,
        truncated,
        victory: terminated && environment.clear_rate() >= 100.0,
        final_state_hash: environment.state_hash(),
        decisions,
    })
}

struct RolloutSample {
    score: f32,
    victory: bool,
    clear_rate: f32,
    final_stage: usize,
}

fn evaluate_candidate_scenario(
    environment: &GameEnvironment,
    candidate: &LegalAction,
    scenario_seed: u64,
    config: &RolloutTeacherConfig,
) -> Result<RolloutSample> {
    let mut rollout = environment
        .fork_for_rollout_seed(scenario_seed)
        .map_err(|error| anyhow::anyhow!("teacher rollout fork failed: {error}"))?;
    let mut outcome = rollout
        .semantic_step(candidate.action.clone())
        .map_err(|error| anyhow::anyhow!("teacher candidate failed: {error:?}"))?;
    settle_forced_actions(&mut rollout, &mut outcome)?;
    for _ in 1..config.horizon_decisions {
        if outcome.terminated || outcome.truncated {
            break;
        }
        let legal_actions =
            rollout.semantic_legal_actions_with_position_limit(config.position_candidate_limit);
        if legal_actions.is_empty() {
            break;
        }
        let action = scripted_expert_action(&rollout.snapshot(), &legal_actions)?;
        outcome = rollout
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("teacher continuation failed: {error:?}"))?;
        settle_forced_actions(&mut rollout, &mut outcome)?;
    }
    let observation = rollout.snapshot();
    let victory = outcome.terminated && rollout.clear_rate() >= 100.0;
    Ok(RolloutSample {
        score: rollout_score(&observation, victory),
        victory,
        clear_rate: rollout.clear_rate(),
        final_stage: observation.stage,
    })
}

fn settle_forced_actions(
    environment: &mut GameEnvironment,
    outcome: &mut StepOutcome,
) -> Result<()> {
    while !outcome.terminated && !outcome.truncated {
        let Some(action) = environment.forced_action() else {
            break;
        };
        *outcome = environment
            .step(action)
            .map_err(|error| anyhow::anyhow!("teacher forced action failed: {error:?}"))?;
    }
    Ok(())
}

fn rollout_score(observation: &crate::environment::Observation, victory: bool) -> f32 {
    let completion = if observation.stage_total_hp_raw > 0 {
        observation.stage_progress_raw as f32 / observation.stage_total_hp_raw as f32
    } else {
        0.0
    };
    let terminal_victory_bonus = if victory { 1_000.0 } else { 0.0 };
    observation.stage.saturating_sub(1) as f32 + completion.clamp(0.0, 1.0) + terminal_victory_bonus
}

pub fn scenario_seed_digest(seeds: &[u64]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"tower-defense-rollout-teacher-seed-schedule-v1");
    digest.update((seeds.len() as u64).to_be_bytes());
    for seed in seeds {
        digest.update(seed.to_be_bytes());
    }
    format!("{:x}", digest.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use std::sync::Arc;

    fn environment() -> GameEnvironment {
        GameEnvironment::new(Arc::new(GameConfig::default_config()), 7)
    }

    #[test]
    fn teacher_reuses_common_seed_schedule_deterministically() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("tower selection should start");
        let candidates = environment
            .semantic_legal_actions_with_position_limit(Some(2))
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let config = RolloutTeacherConfig {
            scenario_seeds: vec![11, 13],
            horizon_decisions: 1,
            position_candidate_limit: Some(2),
            candidate_limit: None,
        };
        let first = evaluate_semantic_candidate_set(&environment, &candidates, &config)
            .expect("teacher should evaluate candidates");
        let second = evaluate_semantic_candidate_set(&environment, &candidates, &config)
            .expect("teacher should repeat deterministically");

        assert_eq!(first, second);
        assert_eq!(first.candidate_count, 2);
        assert_eq!(first.candidates[0].sample_count, 2);
        assert_eq!(first.scenario_seed_digest, scenario_seed_digest(&[11, 13]));
        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.id == first.selected_action_id)
        );
    }

    #[test]
    fn teacher_rejects_empty_scenario_schedule() {
        let environment = environment();
        let candidate = environment
            .legal_actions()
            .into_iter()
            .take(1)
            .collect::<Vec<_>>();
        let error = evaluate_semantic_candidate_set(
            &environment,
            &candidate,
            &RolloutTeacherConfig {
                scenario_seeds: Vec::new(),
                ..RolloutTeacherConfig::default()
            },
        )
        .expect_err("empty scenario schedule should be rejected");
        assert!(error.to_string().contains("scenario seed"));
    }
}
