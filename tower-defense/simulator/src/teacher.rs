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

/// Groups `Reroll`/`BuildTower` candidates by their (sorted) card subset;
/// every other action kind gets its own singleton group. Used to select a
/// candidate_limit-sized subset without favoring whichever subset happens
/// to be generated first (see `select_candidates_fairly`).
fn candidate_group_key(action: &AgentAction) -> Option<Vec<usize>> {
    match action {
        AgentAction::BuildTower { card_ids, .. } | AgentAction::Reroll { card_ids } => {
            let mut ids = card_ids.clone();
            ids.sort_unstable();
            Some(ids)
        }
        _ => None,
    }
}

/// Selects up to `limit` candidates from `candidates` by reordering whole
/// groups (one group per card subset, plus one singleton group per other
/// action) instead of truncating the flattened, generation-ordered list.
///
/// A plain prefix truncate systematically favors whichever card subset or
/// action kind happens to be generated first: measured via
/// `phase2_candidate_limit_bias_report`, a limit of 64 fully excluded 97% of
/// card subsets and every `PurchaseShopItem`/`UseInventoryItem` candidate
/// (they are generated after all card actions), even though the position
/// proposal itself (`position_candidate_limit`) was already unbiased.
fn select_candidates_fairly(candidates: Vec<LegalAction>, limit: usize) -> Vec<LegalAction> {
    if candidates.len() <= limit {
        return candidates;
    }
    let mut groups: Vec<(Option<Vec<usize>>, Vec<LegalAction>)> = Vec::new();
    let mut group_index_by_key: std::collections::HashMap<Vec<usize>, usize> =
        std::collections::HashMap::new();
    for legal in candidates {
        match candidate_group_key(&legal.action) {
            Some(key) => {
                let group_index = *group_index_by_key.entry(key.clone()).or_insert_with(|| {
                    groups.push((Some(key), Vec::new()));
                    groups.len() - 1
                });
                groups[group_index].1.push(legal);
            }
            None => groups.push((None, vec![legal])),
        }
    }

    // Interleaving individual candidates round-robin (an earlier version of
    // this function) spreads the budget so thin across every group that no
    // card subset gets enough position depth to reach a good coverage
    // position - phase2_candidate_limit_bias_report showed it trading the
    // exclusion bias for materially worse coverage regret at low limits.
    // Instead, keep each subset's full candidate block intact (preserving
    // the position depth Phase 1 validated) and only reorder which whole
    // blocks come first, using the sum of a subset's card ids as the sort
    // key. That key has no relationship to a card's current hand-slot
    // index, which is what the original subset-mask generation order
    // encoded and what caused the bias, but non-card actions (typically a
    // handful of shop/inventory/treasure candidates) always sort first so
    // they are never crowded out.
    groups.sort_by_key(|(key, _)| match key {
        None => (0, 0, Vec::new()),
        Some(ids) => (1, ids.iter().sum::<usize>(), ids.clone()),
    });

    let mut selected = Vec::with_capacity(limit);
    for (_, group) in groups {
        if selected.len() >= limit {
            break;
        }
        let remaining = limit - selected.len();
        selected.extend(group.into_iter().take(remaining));
    }
    selected
}

pub fn evaluate_semantic_candidates(
    environment: &GameEnvironment,
    config: &RolloutTeacherConfig,
) -> Result<RolloutTeacherDecision> {
    let mut candidates =
        environment.semantic_legal_actions_with_position_limit(config.position_candidate_limit);
    if let Some(candidate_limit) = config.candidate_limit {
        candidates = select_candidates_fairly(candidates, candidate_limit.max(1));
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

    fn action_kind_key(action: &AgentAction) -> &'static str {
        match action {
            AgentAction::BuildTower { .. } => "build_tower",
            AgentAction::Reroll { .. } => "reroll",
            AgentAction::PurchaseShopItem { .. } => "purchase_shop_item",
            AgentAction::UseInventoryItem { .. } => "use_inventory_item",
            AgentAction::DiscardTreasure { .. } => "discard_treasure",
            _ => "other",
        }
    }

    use super::candidate_group_key as action_subset_key;

    #[derive(serde::Serialize)]
    struct ActionKindSurvival {
        kind: String,
        pre_truncation_count: usize,
        survived_count: usize,
    }

    #[derive(serde::Serialize)]
    struct CandidateLimitSamplePointLimit {
        strategy: &'static str,
        candidate_limit: usize,
        survived_total: usize,
        exact_best_retained: bool,
        top5_retained_count: usize,
        top10_retained_count: usize,
        coverage_regret: usize,
        /// Survival rate of card-subset candidates by generation-order
        /// quartile (quartile 0 = earliest subsets in the flattened list,
        /// quartile 3 = latest). A steep drop from quartile 0 to 3 shows
        /// truncation systematically favors early subsets.
        subset_quartile_survival_rate: [f64; 4],
        fully_excluded_subset_count: usize,
        action_kind_survival: Vec<ActionKindSurvival>,
    }

    #[derive(serde::Serialize)]
    struct CandidateLimitSamplePoint {
        seed: u64,
        decision_index: usize,
        total_pre_truncation_candidates: usize,
        total_subset_count: usize,
        per_limit: Vec<CandidateLimitSamplePointLimit>,
    }

    #[derive(serde::Serialize)]
    struct CandidateLimitAggregate {
        strategy: &'static str,
        candidate_limit: usize,
        sample_count: usize,
        mean_survived_fraction: f64,
        exact_best_retention_rate: f64,
        top5_retention_rate: f64,
        top10_retention_rate: f64,
        mean_coverage_regret: f64,
        max_coverage_regret: usize,
        mean_subset_quartile_survival_rate: [f64; 4],
        mean_fully_excluded_subset_fraction: f64,
    }

    #[derive(serde::Serialize)]
    struct CandidateLimitReport {
        methodology: String,
        position_candidate_limit: usize,
        seeds: Vec<u64>,
        candidate_limits: Vec<usize>,
        sample_points: Vec<CandidateLimitSamplePoint>,
        aggregated: Vec<CandidateLimitAggregate>,
    }

    /// Phase 2 candidate-limit pruning-bias report (see docs/game-ai/02-action-contract.md).
    ///
    /// Compares two selection strategies applied to the same flattened
    /// candidate list (`semantic_legal_actions_with_position_limit`'s
    /// output: card subsets in ascending subset-mask order, each
    /// contributing one Reroll plus up to `position_candidate_limit`
    /// BuildTower actions, followed by shop/inventory/treasure actions):
    ///
    /// - `prefix_truncate`: the original `Vec::truncate` behavior.
    /// - `fair_block_reorder`: `select_candidates_fairly`, which keeps each
    ///   card subset's full candidate block intact but reorders which
    ///   blocks come first by a key unrelated to hand-slot generation
    ///   order, instead of taking a prefix.
    ///
    /// Measures whether prefix truncation systematically drops later card
    /// subsets, action kinds, or high-quality candidates (by the same
    /// rollout-free coverage/route-distance/damage heuristic ranking as
    /// Phase 1), and whether round-robin selection fixes it.
    ///
    /// Run with: cargo test --release -- --ignored phase2_candidate_limit_bias_report --nocapture
    #[test]
    #[ignore = "manual release benchmark; writes artifacts/benchmarks/phase2-candidate-limit-bias.json"]
    fn phase2_candidate_limit_bias_report() {
        use crate::policy_runner::rank_build_tower_actions_by_heuristic;
        use std::collections::HashMap;

        const SEEDS: std::ops::Range<u64> = 0..24;
        const SAMPLES_PER_SEED: usize = 6;
        const MAX_DECISIONS_PER_SEED: usize = 96;
        const CANDIDATE_LIMITS: [usize; 6] = [64, 128, 256, 512, 1024, 2048];
        const STRATEGIES: [&str; 2] = ["prefix_truncate", "fair_block_reorder"];

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
                let full_candidates = environment.semantic_legal_actions_with_position_limit(Some(
                    DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
                ));
                let has_build = full_candidates
                    .iter()
                    .any(|legal| matches!(legal.action, AgentAction::BuildTower { .. }));

                if has_build {
                    // Generation-order rank of each distinct card subset,
                    // taken from its first appearance in the untruncated,
                    // already-generation-ordered candidate list.
                    let mut subset_first_rank: HashMap<Vec<usize>, usize> = HashMap::new();
                    let mut subset_order = Vec::new();
                    let mut subset_pre_counts: HashMap<Vec<usize>, usize> = HashMap::new();
                    for legal in &full_candidates {
                        if let Some(key) = action_subset_key(&legal.action) {
                            *subset_pre_counts.entry(key.clone()).or_insert(0) += 1;
                            subset_first_rank.entry(key.clone()).or_insert_with(|| {
                                let rank = subset_order.len();
                                subset_order.push(key);
                                rank
                            });
                        }
                    }
                    let total_subset_count = subset_order.len();

                    let full_build_actions = full_candidates
                        .iter()
                        .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                        .cloned()
                        .collect::<Vec<_>>();
                    let reference_ranking =
                        rank_build_tower_actions_by_heuristic(&observation, &full_build_actions);
                    let reference_best = reference_ranking.first();
                    let reference_best_id = reference_best.map(|score| score.action.action_id());
                    let reference_best_covered_route =
                        reference_best.map(|score| score.covered_route).unwrap_or(0);
                    let reference_top5 = reference_ranking
                        .iter()
                        .take(5)
                        .map(|score| score.action.action_id())
                        .collect::<std::collections::HashSet<_>>();
                    let reference_top10 = reference_ranking
                        .iter()
                        .take(10)
                        .map(|score| score.action.action_id())
                        .collect::<std::collections::HashSet<_>>();

                    let mut action_kind_pre_counts: HashMap<&'static str, usize> = HashMap::new();
                    for legal in &full_candidates {
                        *action_kind_pre_counts
                            .entry(action_kind_key(&legal.action))
                            .or_insert(0) += 1;
                    }

                    let per_limit = STRATEGIES
                        .iter()
                        .flat_map(|&strategy| {
                            CANDIDATE_LIMITS.iter().map(move |&limit| (strategy, limit))
                        })
                        .map(|(strategy, limit)| {
                            let truncated = if strategy == "fair_block_reorder" {
                                super::select_candidates_fairly(
                                    full_candidates.clone(),
                                    limit.max(1),
                                )
                            } else {
                                let mut truncated = full_candidates.clone();
                                truncated.truncate(limit.max(1));
                                truncated
                            };

                            let mut subset_survived: HashMap<Vec<usize>, usize> = HashMap::new();
                            let mut action_kind_survived: HashMap<&'static str, usize> =
                                HashMap::new();
                            for legal in &truncated {
                                if let Some(key) = action_subset_key(&legal.action) {
                                    *subset_survived.entry(key).or_insert(0) += 1;
                                }
                                *action_kind_survived
                                    .entry(action_kind_key(&legal.action))
                                    .or_insert(0) += 1;
                            }

                            let mut quartile_survived = [0usize; 4];
                            let mut quartile_total = [0usize; 4];
                            let mut fully_excluded_subset_count = 0usize;
                            for (key, &rank) in &subset_first_rank {
                                let quartile = if total_subset_count <= 1 {
                                    0
                                } else {
                                    (rank * 4 / total_subset_count).min(3)
                                };
                                let pre = *subset_pre_counts.get(key).unwrap_or(&0);
                                let survived = *subset_survived.get(key).unwrap_or(&0);
                                quartile_total[quartile] += pre;
                                quartile_survived[quartile] += survived;
                                if pre > 0 && survived == 0 {
                                    fully_excluded_subset_count += 1;
                                }
                            }
                            let subset_quartile_survival_rate = std::array::from_fn(|index| {
                                if quartile_total[index] == 0 {
                                    0.0
                                } else {
                                    quartile_survived[index] as f64 / quartile_total[index] as f64
                                }
                            });

                            let truncated_build_actions = truncated
                                .iter()
                                .filter(|legal| {
                                    matches!(legal.action, AgentAction::BuildTower { .. })
                                })
                                .cloned()
                                .collect::<Vec<_>>();
                            let truncated_ranking = rank_build_tower_actions_by_heuristic(
                                &observation,
                                &truncated_build_actions,
                            );
                            let truncated_best_covered_route = truncated_ranking
                                .first()
                                .map(|score| score.covered_route)
                                .unwrap_or(0);
                            let truncated_ids = truncated_build_actions
                                .iter()
                                .map(|legal| legal.id.clone())
                                .collect::<std::collections::HashSet<_>>();
                            let exact_best_retained = reference_best_id
                                .as_ref()
                                .is_some_and(|id| truncated_ids.contains(id));
                            let top5_retained_count = reference_top5
                                .iter()
                                .filter(|id| truncated_ids.contains(*id))
                                .count();
                            let top10_retained_count = reference_top10
                                .iter()
                                .filter(|id| truncated_ids.contains(*id))
                                .count();
                            let coverage_regret = reference_best_covered_route
                                .saturating_sub(truncated_best_covered_route);

                            let action_kind_survival = action_kind_pre_counts
                                .keys()
                                .map(|&kind| ActionKindSurvival {
                                    kind: kind.to_string(),
                                    pre_truncation_count: action_kind_pre_counts[kind],
                                    survived_count: *action_kind_survived.get(kind).unwrap_or(&0),
                                })
                                .collect();

                            CandidateLimitSamplePointLimit {
                                strategy,
                                candidate_limit: limit,
                                survived_total: truncated.len(),
                                exact_best_retained,
                                top5_retained_count,
                                top10_retained_count,
                                coverage_regret,
                                subset_quartile_survival_rate,
                                fully_excluded_subset_count,
                                action_kind_survival,
                            }
                        })
                        .collect();

                    sample_points.push(CandidateLimitSamplePoint {
                        seed,
                        decision_index,
                        total_pre_truncation_candidates: full_candidates.len(),
                        total_subset_count,
                        per_limit,
                    });
                    samples_collected += 1;
                }

                let action = scripted_expert_action(&observation, &full_candidates)
                    .expect("scripted expert should find an action");
                let outcome = environment
                    .semantic_step(action)
                    .expect("scripted step should be accepted");
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
        }

        let aggregated = STRATEGIES
            .iter()
            .flat_map(|&strategy| CANDIDATE_LIMITS.iter().map(move |&limit| (strategy, limit)))
            .map(|(strategy, limit)| {
                let entries = sample_points
                    .iter()
                    .flat_map(|point| {
                        point
                            .per_limit
                            .iter()
                            .filter(|l| l.strategy == strategy && l.candidate_limit == limit)
                    })
                    .collect::<Vec<_>>();
                let count = entries.len().max(1);
                let mean = |values: Vec<f64>| values.iter().sum::<f64>() / count as f64;
                let mean_survived_fraction = sample_points
                    .iter()
                    .flat_map(|point| {
                        point
                            .per_limit
                            .iter()
                            .filter(|l| l.strategy == strategy && l.candidate_limit == limit)
                            .map(|l| {
                                l.survived_total as f64
                                    / point.total_pre_truncation_candidates as f64
                            })
                    })
                    .sum::<f64>()
                    / count as f64;
                let mut quartile_sum = [0.0; 4];
                for entry in &entries {
                    for index in 0..4 {
                        quartile_sum[index] += entry.subset_quartile_survival_rate[index];
                    }
                }
                let mean_subset_quartile_survival_rate =
                    std::array::from_fn(|index| quartile_sum[index] / count as f64);
                CandidateLimitAggregate {
                    strategy,
                    candidate_limit: limit,
                    sample_count: entries.len(),
                    mean_survived_fraction,
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
                    mean_subset_quartile_survival_rate,
                    mean_fully_excluded_subset_fraction: mean(
                        sample_points
                            .iter()
                            .flat_map(|point| {
                                point
                                    .per_limit
                                    .iter()
                                    .filter(|l| {
                                        l.strategy == strategy && l.candidate_limit == limit
                                    })
                                    .map(|l| {
                                        l.fully_excluded_subset_count as f64
                                            / point.total_subset_count.max(1) as f64
                                    })
                            })
                            .collect(),
                    ),
                }
            })
            .collect::<Vec<_>>();

        for stat in &aggregated {
            println!(
                "strategy={} candidate_limit={} sample_count={} mean_survived_fraction={:.4} \
                exact_best_retention={:.4} top5_retention={:.4} top10_retention={:.4} \
                mean_coverage_regret={:.4} max_coverage_regret={} \
                quartile_survival(early->late)={:?} mean_fully_excluded_subset_fraction={:.4}",
                stat.strategy,
                stat.candidate_limit,
                stat.sample_count,
                stat.mean_survived_fraction,
                stat.exact_best_retention_rate,
                stat.top5_retention_rate,
                stat.top10_retention_rate,
                stat.mean_coverage_regret,
                stat.max_coverage_regret,
                stat.mean_subset_quartile_survival_rate,
                stat.mean_fully_excluded_subset_fraction
            );
        }

        let report = CandidateLimitReport {
            methodology: "reference ranking is rank_build_tower_actions_by_heuristic over the \
                full (position-limited, candidate-limit-unlimited) BuildTower candidate set; \
                coverage_regret and top-k retention are measured the same way as Phase 1's \
                recall report, but against this candidate_limit truncation instead of the \
                position_candidate_limit. subset_quartile_survival_rate buckets card subsets by \
                their first-appearance rank in the flattened, generation-ordered candidate list \
                (quartile 0 = earliest subsets, quartile 3 = latest) and reports mean candidate \
                survival rate per quartile after truncation."
                .to_string(),
            position_candidate_limit: DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
            seeds: SEEDS.collect(),
            candidate_limits: CANDIDATE_LIMITS.to_vec(),
            sample_points,
            aggregated,
        };

        let json = serde_json::to_string_pretty(&report).expect("report should serialize");
        std::fs::create_dir_all("../artifacts/benchmarks")
            .expect("benchmark artifact directory should be creatable");
        std::fs::write(
            "../artifacts/benchmarks/phase2-candidate-limit-bias.json",
            &json,
        )
        .expect("benchmark report should be written");
    }
}
