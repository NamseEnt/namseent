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

    #[derive(serde::Serialize)]
    struct RecallLimitStat {
        position_limit: usize,
        sample_count: usize,
        mean_legal_candidate_recall: f64,
        exact_best_retention_rate: f64,
        top5_retention_rate: f64,
        top10_retention_rate: f64,
        mean_coverage_regret: f64,
        max_coverage_regret: usize,
        share_with_nonzero_regret: f64,
    }

    #[derive(serde::Serialize)]
    struct RecallSamplePointLimit {
        position_limit: usize,
        proposed_build_candidate_count: usize,
        legal_candidate_recall: f64,
        exact_best_retained: bool,
        top5_retained_count: usize,
        top10_retained_count: usize,
        best_covered_route_within_limit: usize,
        coverage_regret: usize,
    }

    #[derive(serde::Serialize)]
    struct RecallSamplePoint {
        seed: u64,
        decision_index: usize,
        oracle_build_candidate_count: usize,
        oracle_best_action_id: Option<String>,
        oracle_best_covered_route: usize,
        per_limit: Vec<RecallSamplePointLimit>,
    }

    #[derive(serde::Serialize)]
    struct RecallReport {
        methodology: String,
        seeds: Vec<u64>,
        position_limits: Vec<usize>,
        sample_points: Vec<RecallSamplePoint>,
        aggregated: Vec<RecallLimitStat>,
    }

    /// Phase 1 candidate-proposal recall report (see docs/game-ai/02-action-contract.md).
    ///
    /// The oracle ranking is `rank_build_tower_actions_by_heuristic`
    /// (coverage, then route distance, then damage) over the FULL oracle
    /// `BuildTower` candidate set (no position limit). It is a rollout-free
    /// oracle-quality proxy: a real rollout evaluation over the full oracle
    /// set is computationally intractable (tens of thousands to hundreds of
    /// thousands of candidates per decision), so "oracle best" here means
    /// best by this deterministic heuristic, not a ground-truth best future
    /// outcome. `coverage_regret` is the oracle-best candidate's route-tile
    /// coverage count minus the best coverage achievable among the
    /// position-limited proposal, i.e. how much of the primary ranking
    /// dimension is lost by truncating, not a rollout value gap.
    ///
    /// Run with: cargo test --release -- --ignored phase1_candidate_recall_report --nocapture
    #[test]
    #[ignore = "manual release benchmark; writes artifacts/benchmarks/phase1-candidate-recall.json"]
    fn phase1_candidate_recall_report() {
        use crate::policy_runner::rank_build_tower_actions_by_heuristic;
        use std::collections::HashSet;

        const SEEDS: std::ops::Range<u64> = 0..24;
        const SAMPLES_PER_SEED: usize = 6;
        const LIMITS: [usize; 5] = [8, 16, 32, 48, 64];
        const MAX_DECISIONS_PER_SEED: usize = 96;

        let mut sample_points = Vec::new();

        for seed in SEEDS {
            let mut environment =
                GameEnvironment::new(std::sync::Arc::new(GameConfig::default_config()), seed);
            environment
                .step(AgentAction::StartSelectingTower)
                .expect("start selecting tower should be legal");

            let mut samples_collected = 0usize;
            let mut decision_index = 0usize;
            while samples_collected < SAMPLES_PER_SEED && decision_index < MAX_DECISIONS_PER_SEED {
                decision_index += 1;
                let observation = environment.snapshot();
                let oracle_build_actions = environment
                    .semantic_legal_actions()
                    .into_iter()
                    .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                    .collect::<Vec<_>>();

                if !oracle_build_actions.is_empty() {
                    let oracle_ranking =
                        rank_build_tower_actions_by_heuristic(&observation, &oracle_build_actions);
                    let oracle_best = oracle_ranking.first();
                    let oracle_best_id = oracle_best.map(|score| score.action.action_id());
                    let oracle_best_covered_route =
                        oracle_best.map(|score| score.covered_route).unwrap_or(0);
                    let oracle_top5 = oracle_ranking
                        .iter()
                        .take(5)
                        .map(|score| score.action.action_id())
                        .collect::<HashSet<_>>();
                    let oracle_top10 = oracle_ranking
                        .iter()
                        .take(10)
                        .map(|score| score.action.action_id())
                        .collect::<HashSet<_>>();
                    let oracle_ids = oracle_build_actions
                        .iter()
                        .map(|legal| legal.id.clone())
                        .collect::<HashSet<_>>();

                    let per_limit = LIMITS
                        .iter()
                        .map(|&limit| {
                            let proposed = environment
                                .semantic_legal_actions_with_position_limit(Some(limit))
                                .into_iter()
                                .filter(|legal| {
                                    matches!(legal.action, AgentAction::BuildTower { .. })
                                })
                                .collect::<Vec<_>>();
                            let proposed_ids = proposed
                                .iter()
                                .map(|legal| legal.id.clone())
                                .collect::<HashSet<_>>();
                            assert!(
                                proposed_ids.is_subset(&oracle_ids),
                                "proposed candidates must stay within the oracle set"
                            );
                            let recall = proposed.len() as f64 / oracle_build_actions.len() as f64;
                            let exact_best_retained = oracle_best_id
                                .as_ref()
                                .is_some_and(|id| proposed_ids.contains(id));
                            let top5_retained_count = oracle_top5
                                .iter()
                                .filter(|id| proposed_ids.contains(*id))
                                .count();
                            let top10_retained_count = oracle_top10
                                .iter()
                                .filter(|id| proposed_ids.contains(*id))
                                .count();
                            let proposed_ranking =
                                rank_build_tower_actions_by_heuristic(&observation, &proposed);
                            let best_covered_route_within_limit = proposed_ranking
                                .first()
                                .map(|score| score.covered_route)
                                .unwrap_or(0);
                            let coverage_regret = oracle_best_covered_route
                                .saturating_sub(best_covered_route_within_limit);
                            RecallSamplePointLimit {
                                position_limit: limit,
                                proposed_build_candidate_count: proposed.len(),
                                legal_candidate_recall: recall,
                                exact_best_retained,
                                top5_retained_count,
                                top10_retained_count,
                                best_covered_route_within_limit,
                                coverage_regret,
                            }
                        })
                        .collect();

                    sample_points.push(RecallSamplePoint {
                        seed,
                        decision_index,
                        oracle_build_candidate_count: oracle_build_actions.len(),
                        oracle_best_action_id: oracle_best_id,
                        oracle_best_covered_route,
                        per_limit,
                    });
                    samples_collected += 1;
                }

                let legal_actions = environment.semantic_legal_actions_with_position_limit(Some(
                    DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
                ));
                let action = scripted_expert_action(&observation, &legal_actions)
                    .expect("scripted expert should find an action");
                let outcome = environment
                    .semantic_step(action)
                    .expect("scripted step should be accepted");
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
        }

        let aggregated = LIMITS
            .iter()
            .map(|&limit| {
                let entries = sample_points
                    .iter()
                    .flat_map(|point| point.per_limit.iter().filter(|l| l.position_limit == limit))
                    .collect::<Vec<_>>();
                let count = entries.len().max(1);
                let mean = |values: Vec<f64>| values.iter().sum::<f64>() / count as f64;
                RecallLimitStat {
                    position_limit: limit,
                    sample_count: entries.len(),
                    mean_legal_candidate_recall: mean(
                        entries.iter().map(|l| l.legal_candidate_recall).collect(),
                    ),
                    exact_best_retention_rate: mean(
                        entries
                            .iter()
                            .map(|l| l.exact_best_retained as u8 as f64)
                            .collect(),
                    ),
                    top5_retention_rate: mean(
                        entries
                            .iter()
                            .map(|l| l.top5_retained_count as f64 / 5.0)
                            .collect(),
                    ),
                    top10_retention_rate: mean(
                        entries
                            .iter()
                            .map(|l| l.top10_retained_count as f64 / 10.0)
                            .collect(),
                    ),
                    mean_coverage_regret: mean(
                        entries.iter().map(|l| l.coverage_regret as f64).collect(),
                    ),
                    max_coverage_regret: entries
                        .iter()
                        .map(|l| l.coverage_regret)
                        .max()
                        .unwrap_or(0),
                    share_with_nonzero_regret: mean(
                        entries
                            .iter()
                            .map(|l| (l.coverage_regret > 0) as u8 as f64)
                            .collect(),
                    ),
                }
            })
            .collect::<Vec<_>>();

        for stat in &aggregated {
            println!(
                "position_limit={} sample_count={} mean_legal_candidate_recall={:.4} \
                exact_best_retention_rate={:.4} top5_retention_rate={:.4} \
                top10_retention_rate={:.4} mean_coverage_regret={:.4} \
                max_coverage_regret={} share_with_nonzero_regret={:.4}",
                stat.position_limit,
                stat.sample_count,
                stat.mean_legal_candidate_recall,
                stat.exact_best_retention_rate,
                stat.top5_retention_rate,
                stat.top10_retention_rate,
                stat.mean_coverage_regret,
                stat.max_coverage_regret,
                stat.share_with_nonzero_regret
            );
        }

        let report = RecallReport {
            methodology: "oracle ranking is rank_build_tower_actions_by_heuristic (coverage, then \
                route distance, then damage) over the full oracle BuildTower candidate set (no \
                position limit); a real rollout evaluation over the full oracle set is \
                computationally intractable, so this is a rollout-free oracle-quality proxy, not a \
                ground-truth best action. legal_candidate_recall is |proposed BuildTower \
                candidates| / |oracle BuildTower candidates|. coverage_regret is the oracle-best \
                candidate's route-tile coverage minus the best coverage achievable within the \
                position-limited proposal at that sample point."
                .to_string(),
            seeds: SEEDS.collect(),
            position_limits: LIMITS.to_vec(),
            sample_points,
            aggregated,
        };

        let json = serde_json::to_string_pretty(&report).expect("report should serialize");
        std::fs::create_dir_all("../artifacts/benchmarks")
            .expect("benchmark artifact directory should be creatable");
        std::fs::write(
            "../artifacts/benchmarks/phase1-candidate-recall.json",
            &json,
        )
        .expect("benchmark report should be written");
    }
}
