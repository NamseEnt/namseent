use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[cfg(test)]
use crate::environment::DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT;
use crate::environment::{AgentAction, GameEnvironment, LegalAction, StepOutcome};
use crate::joint_action::DenseBuildTowerScoreTable;
use crate::policy_runner::canonical_scripted_semantic_action;
#[cfg(test)]
use crate::policy_runner::scripted_expert_action;

/// Bumped 1 -> 2: the baseline/regret contract changed from "baseline may be
/// absent from the candidate set" (`Option` fields) to "baseline is always
/// evaluated with the identical rollout and always present" (non-`Option`
/// fields), and the production candidate/continuation contract dropped the
/// legacy `position_candidate_limit` confound (see
/// docs/game-ai/05-rollout-teacher.md). Serialized `RolloutTeacherDecision`
/// values from schema version 1 are not compatible with this version.
pub const TEACHER_SCORE_SCHEMA_VERSION: u32 = 2;
pub const DEFAULT_TEACHER_HORIZON_DECISIONS: usize = 8;
pub const DEFAULT_TEACHER_SCENARIO_COUNT: usize = 16;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutTeacherConfig {
    pub scenario_seeds: Vec<u64>,
    pub horizon_decisions: usize,
    /// How many top-ranked `BuildTower` actions (by
    /// `DenseBuildTowerScoreTable`'s global ranking) to roll out per
    /// decision; every non-`BuildTower` semantic action is always included
    /// regardless of this limit (see `dense_semantic_candidates`). `None`
    /// rolls out every legal `BuildTower` action.
    pub build_tower_rollout_limit: Option<usize>,
}

impl Default for RolloutTeacherConfig {
    fn default() -> Self {
        Self {
            scenario_seeds: (0..DEFAULT_TEACHER_SCENARIO_COUNT as u64).collect(),
            horizon_decisions: DEFAULT_TEACHER_HORIZON_DECISIONS,
            build_tower_rollout_limit: None,
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
    pub baseline_action_id: String,
    pub baseline_mean_score: f32,
    pub expert_regret: f32,
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

/// Legacy candidate-pruning path (Phase 2 diagnostic, see
/// `phase2_candidate_limit_bias_report`): groups `Reroll`/`BuildTower`
/// candidates by their (sorted) card subset; every other action kind gets
/// its own singleton group. Used to select a candidate_limit-sized subset
/// without favoring whichever subset happens to be generated first (see
/// `select_candidates_fairly`). No longer used by
/// `evaluate_semantic_candidates` - see `dense_semantic_candidates`. Kept
/// `cfg(test)` since its only remaining caller is the Phase 2 benchmark.
#[cfg(test)]
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
/// Legacy: production candidate pruning is now
/// `dense_semantic_candidates`'s post-ranking top-K, not this. Kept
/// `cfg(test)` since its only remaining caller is the Phase 2 benchmark.
#[cfg(test)]
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

/// Assembles the production teacher's candidate set: every legal
/// non-`BuildTower` semantic action (`Reroll`, shop/inventory/treasure -
/// this set is small, so it is never pruned), plus the `build_tower_limit`
/// best `BuildTower` actions by `DenseBuildTowerScoreTable`'s global
/// ranking over every legal `(card subset, map position)` pair.
///
/// Unlike the legacy `position_candidate_limit` position proposal +
/// generation-order `candidate_limit` truncation
/// (`select_candidates_fairly`), pruning here is purely a teacher search
/// -budget decision made *after* every legal `BuildTower` action has been
/// scored: `build_tower_limit` can never cause a good action to be absent
/// from consideration, only absent from the (bounded) set actually rolled
/// out.
///
/// `build_tower_limit: None` rolls out every legal `BuildTower` action.
pub fn dense_semantic_candidates(
    environment: &GameEnvironment,
    build_tower_limit: Option<usize>,
) -> Vec<LegalAction> {
    let mut candidates = environment.semantic_non_build_actions();
    if !environment.semantic_card_decision_available() {
        return candidates;
    }
    let observation = environment.snapshot();
    let table = DenseBuildTowerScoreTable::compute(environment, &observation);
    let k = build_tower_limit.unwrap_or(usize::MAX);
    candidates.extend(table.top_k_actions(k).into_iter().map(|action| LegalAction {
        id: action.action_id(),
        action,
    }));
    candidates
}

pub fn evaluate_semantic_candidates(
    environment: &GameEnvironment,
    config: &RolloutTeacherConfig,
) -> Result<RolloutTeacherDecision> {
    let candidates = dense_semantic_candidates(environment, config.build_tower_rollout_limit);
    evaluate_semantic_candidate_set(environment, &candidates, config)
}

/// Evaluates `candidates` plus the canonical heuristic baseline action
/// (`canonical_scripted_semantic_action`) under an identical scenario/
/// continuation contract, so `baseline_mean_score`/`expert_regret` are
/// always populated - never a candidate-representation artifact of the
/// caller's `candidates` set being smaller than the full legal action space.
/// If the baseline action is not already present in `candidates` (by action
/// identity, i.e. `LegalAction::id`), it is added before evaluation; either
/// way every candidate (including the baseline) is evaluated with the same
/// `config.scenario_seeds` schedule and the same canonical continuation
/// policy.
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
    // Checked per-action (authoritative can_place_at/card-selectability,
    // see `GameEnvironment::semantic_action_is_legal`) rather than by
    // membership in a position-limited enumeration: dense `BuildTower`
    // candidates (`dense_semantic_candidates`) are ranked over the full
    // map, not just a position-limited window, and materializing the full
    // legal-action list here just to check membership would reintroduce the
    // O(subset x position) allocation this module's dense path exists to
    // avoid.
    if candidates
        .iter()
        .any(|candidate| !environment.semantic_action_is_legal(&candidate.action))
    {
        bail!("rollout teacher candidate is not legal in the source environment");
    }
    let observation = environment.snapshot();
    let baseline_action = canonical_scripted_semantic_action(environment)?;
    let baseline_action_id = baseline_action.action_id();

    let mut all_candidates = candidates.to_vec();
    if !all_candidates
        .iter()
        .any(|candidate| candidate.id == baseline_action_id)
    {
        all_candidates.push(LegalAction {
            id: baseline_action_id.clone(),
            action: baseline_action,
        });
    }
    let mut seen_ids = std::collections::HashSet::with_capacity(all_candidates.len());
    all_candidates.retain(|candidate| seen_ids.insert(candidate.id.clone()));

    let mut estimates = Vec::with_capacity(all_candidates.len());
    for candidate in &all_candidates {
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
    let baseline_mean_score = estimates
        .iter()
        .find(|candidate| candidate.action_id == baseline_action_id)
        .expect("baseline action is always added to the evaluated candidate set")
        .mean_score;
    Ok(RolloutTeacherDecision {
        score_schema_version: TEACHER_SCORE_SCHEMA_VERSION,
        state_hash: environment.state_hash(),
        observation,
        scenario_seed_digest: scenario_seed_digest(&config.scenario_seeds),
        horizon_decisions: config.horizon_decisions,
        candidate_count: estimates.len(),
        selected_action_id,
        selected_mean_score,
        baseline_action_id,
        baseline_mean_score,
        expert_regret: selected_mean_score - baseline_mean_score,
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
    // Fixed continuation policy for every candidate: the canonical dense
    // scripted heuristic, never a legacy position-limited proposal, a
    // per-candidate policy, or the teacher's own recursive selection (see
    // docs/game-ai/05-rollout-teacher.md's Continuation policy section).
    for _ in 1..config.horizon_decisions {
        if outcome.terminated
            || outcome.truncated
            || matches!(
                rollout.decision_point(),
                crate::environment::DecisionPoint::Terminal
            )
        {
            break;
        }
        let action = canonical_scripted_semantic_action(&rollout)?;
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

pub(crate) fn settle_forced_actions(
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
        let baseline = canonical_scripted_semantic_action(&environment)
            .expect("canonical baseline should be available");
        let candidates = vec![LegalAction {
            id: baseline.action_id(),
            action: baseline,
        }];
        let config = RolloutTeacherConfig {
            scenario_seeds: vec![11, 13],
            horizon_decisions: 1,
            build_tower_rollout_limit: None,
        };
        let first = evaluate_semantic_candidate_set(&environment, &candidates, &config)
            .expect("teacher should evaluate candidates");
        let second = evaluate_semantic_candidate_set(&environment, &candidates, &config)
            .expect("teacher should repeat deterministically");

        assert_eq!(first, second);
        assert_eq!(first.candidate_count, 1);
        assert_eq!(first.candidates[0].sample_count, 2);
        assert_eq!(first.scenario_seed_digest, scenario_seed_digest(&[11, 13]));
        assert_eq!(first.selected_action_id, candidates[0].id);
        assert_eq!(first.baseline_action_id, candidates[0].id);
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

    /// The baseline action must always be present in the evaluated
    /// candidate set - and `baseline_action_id`/`baseline_mean_score`/
    /// `expert_regret` must always be populated - even when
    /// `build_tower_rollout_limit` is small enough that
    /// `dense_semantic_candidates` would otherwise never include the
    /// canonical baseline's own `BuildTower` choice.
    #[test]
    fn baseline_action_is_always_present_even_with_a_tiny_rollout_limit() {
        let mut environment = card_decision_environment(0);
        // Drive a few scripted steps so a real BuildTower decision is live.
        for _ in 0..4 {
            let observation = environment.snapshot();
            let legal_actions = environment.semantic_legal_actions();
            let Ok(action) = scripted_expert_action(&observation, &legal_actions) else {
                break;
            };
            if environment.semantic_step(action).is_err() {
                break;
            }
        }
        let config = RolloutTeacherConfig {
            scenario_seeds: vec![1],
            horizon_decisions: 1,
            build_tower_rollout_limit: Some(1),
        };
        let decision = evaluate_semantic_candidates(&environment, &config)
            .expect("teacher should evaluate with a tiny rollout limit");
        assert!(!decision.baseline_action_id.is_empty());
        assert!(
            decision
                .candidates
                .iter()
                .any(|candidate| candidate.action_id == decision.baseline_action_id),
            "baseline action must be present in the evaluated candidate set"
        );
        assert_eq!(
            decision.expert_regret,
            decision.selected_mean_score - decision.baseline_mean_score
        );
    }

    /// Baseline and every teacher candidate must be scored under an
    /// identical scenario seed schedule/digest, and re-evaluating the same
    /// state with the same config must reproduce exactly the same decision.
    #[test]
    fn evaluating_the_same_state_and_config_twice_is_exactly_reproducible() {
        let environment = card_decision_environment(1);
        let config = RolloutTeacherConfig {
            scenario_seeds: vec![5, 6, 7],
            horizon_decisions: 2,
            build_tower_rollout_limit: Some(4),
        };
        let first = evaluate_semantic_candidates(&environment, &config)
            .expect("teacher should evaluate candidates");
        let second = evaluate_semantic_candidates(&environment, &config)
            .expect("teacher should repeat deterministically");
        assert_eq!(first, second);
        assert_eq!(
            first.scenario_seed_digest,
            scenario_seed_digest(&config.scenario_seeds)
        );
        for candidate in &first.candidates {
            assert_eq!(candidate.sample_count, config.scenario_seeds.len());
        }
    }

    // --- dense_semantic_candidates correctness -----------------------

    fn card_decision_environment(seed: u64) -> GameEnvironment {
        let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
    }

    /// A card-decision state with `extra_count` pending
    /// `stage_modifiers.extra_tower_cards` - i.e. `build_tower_slot_count()
    /// == extra_count + 1`, so `BuildTower` has real `hand_slot_index >= 1`
    /// choices. This state does arise from real play (the Rubber Cone
    /// item, used outside `PlacingTower`), but not reliably from a handful
    /// of fixed seeds, so it's seeded directly via the
    /// `test_only_seed_extra_tower_cards` fixture instead of hoping a
    /// scripted playthrough draws and uses that item.
    fn extra_tower_cards_environment(extra_count: usize) -> GameEnvironment {
        let mut environment = card_decision_environment(0);
        environment
            .test_only_seed_extra_tower_cards(extra_count)
            .expect("extra tower card fixture should be a valid snapshot");
        assert_eq!(environment.build_tower_slot_count(), extra_count + 1);
        environment
    }

    /// Independent brute-force oracle: every `(subset_index,
    /// hand_slot_index, position_index)` triple, scored by the exact same
    /// formula `DenseBuildTowerScoreTable::compute` uses, but with
    /// legality decided by `GameEnvironment::can_place_at` directly rather
    /// than the fast-path mask - deliberately not reusing any of
    /// `joint_action`'s internals - then sorted with an ordering written
    /// separately from `DenseBuildTowerScoreTable::top_k_indices`. Used to
    /// check that method's sort/truncate logic, not to re-prove
    /// legality-mask correctness (already covered by
    /// `full_map_legality_mask_matches_can_place_at_for_every_position`).
    fn brute_force_joint_ranking(
        environment: &GameEnvironment,
        observation: &crate::environment::Observation,
    ) -> Vec<(usize, usize, usize, crate::joint_action::JointBuildTowerScore)> {
        use crate::joint_action::{CardSubsetTable, MAP_POSITION_COUNT, position_xy};

        // Legality never depends on card subset or build slot (fixed 2x2
        // footprint), so it is computed once and reused - same principle
        // `DenseBuildTowerScoreTable`/`legality::full_map_legality_mask`
        // use, kept here via the authoritative per-position
        // `can_place_at` instead of the fast-path mask under test.
        let legal_positions = (0..MAP_POSITION_COUNT)
            .filter(|&position_index| {
                let (left, top) = position_xy(position_index).expect("index in range");
                environment.can_place_at(left, top)
            })
            .collect::<Vec<_>>();
        let route = &observation.route_coords;
        let score_at = |range_raw: i64, damage_raw: i64, position_index: usize| {
            let (left, top) = position_xy(position_index).expect("index in range");
            let covered_route = route
                .iter()
                .filter(|coord| {
                    let dx = (coord.x as i64 - left as i64)
                        .saturating_mul(1_000_000)
                        .saturating_sub(500_000);
                    let dy = (coord.y as i64 - top as i64)
                        .saturating_mul(1_000_000)
                        .saturating_sub(500_000);
                    dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
                        <= range_raw.saturating_mul(range_raw)
                })
                .count();
            let nearest_route = route
                .iter()
                .map(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top))
                .min()
                .unwrap_or(usize::MAX);
            crate::joint_action::JointBuildTowerScore {
                covered_route,
                nearest_route,
                damage_raw,
            }
        };

        let subsets = CardSubsetTable::from_observation(observation);
        // Fixed regardless of card subset - see
        // `joint_action::extra_slot_templates`.
        let extra_ranges = observation
            .extra_tower_card_templates
            .iter()
            .map(|template| (template.range_raw, template.damage_raw))
            .collect::<Vec<_>>();

        let mut ranked = Vec::new();
        for subset_index in 0..subsets.subset_count() {
            let card_ids = subsets
                .card_ids_for_subset(subset_index)
                .expect("subset_index is in range");
            // `rank_build_tower_actions_by_heuristic` matches
            // `candidate.card_ids` by exact (unsorted) equality, so it
            // can't be reused with `CardSubsetTable`'s id-sorted output
            // directly - match by sorted-id set here instead (same
            // approach `joint_action::subset_templates` uses) and compute
            // the score formula inline.
            if let Some(template) = observation.build_tower_candidates.iter().find_map(|candidate| {
                let mut ids = candidate.card_ids.clone();
                ids.sort_unstable();
                (ids == card_ids).then_some(&candidate.template)
            }) {
                let range_raw = template.range_raw;
                for &position_index in &legal_positions {
                    ranked.push((
                        subset_index,
                        0usize,
                        position_index,
                        score_at(range_raw, template.damage_raw, position_index),
                    ));
                }
            }
            // Every non-empty subset is a legal SelectTower choice, so
            // extra_tower_cards slots (fixed template, independent of
            // subset) are never gated on the subset's own template above.
            for (extra_offset, &(range_raw, damage_raw)) in extra_ranges.iter().enumerate() {
                let hand_slot_index = extra_offset + 1;
                for &position_index in &legal_positions {
                    ranked.push((
                        subset_index,
                        hand_slot_index,
                        position_index,
                        score_at(range_raw, damage_raw, position_index),
                    ));
                }
            }
        }
        ranked.sort_by(|left, right| {
            let left_key = (left.3.covered_route, left.3.nearest_route, left.3.damage_raw);
            let right_key = (
                right.3.covered_route,
                right.3.nearest_route,
                right.3.damage_raw,
            );
            right_key
                .0
                .cmp(&left_key.0)
                .then_with(|| left_key.1.cmp(&right_key.1))
                .then_with(|| right_key.2.cmp(&left_key.2))
                .then_with(|| left.0.cmp(&right.0))
                .then_with(|| left.1.cmp(&right.1))
                .then_with(|| left.2.cmp(&right.2))
        });
        ranked
    }

    /// `DenseBuildTowerScoreTable::top_k_indices`' sort/truncate must match
    /// an independently-written brute-force ranking exactly, for several K
    /// values including "all". Uses `extra_tower_cards_environment` (seed
    /// with a forced multi-build-slot state, see below) in addition to
    /// plain seeds so the `hand_slot_index >= 1` axis is exercised, not
    /// just the `hand_slot_index == 0` case the pre-migration table
    /// covered.
    #[test]
    fn dense_top_k_matches_brute_force_ranking() {
        let environments = (0..4u64)
            .map(card_decision_environment)
            .chain(std::iter::once(extra_tower_cards_environment(1)))
            .collect::<Vec<_>>();
        for (index, environment) in environments.iter().enumerate() {
            let observation = environment.snapshot();
            let table = DenseBuildTowerScoreTable::compute(environment, &observation);
            let brute_force = brute_force_joint_ranking(environment, &observation);
            if brute_force.is_empty() {
                continue;
            }
            for k in [1usize, 8, 32, brute_force.len()] {
                let dense_top_k = table.top_k_indices(k);
                let expected = brute_force
                    .iter()
                    .take(k)
                    .map(|(subset_index, hand_slot_index, position_index, _)| {
                        (*subset_index, *hand_slot_index, *position_index)
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    dense_top_k, expected,
                    "environment {index} k={k}: dense top-K must match the brute-force ranking exactly"
                );
                for &(subset_index, hand_slot_index, position_index) in &dense_top_k {
                    assert_eq!(
                        table.score(subset_index, hand_slot_index, position_index),
                        brute_force
                            .iter()
                            .find(|(s, h, p, _)| {
                                *s == subset_index && *h == hand_slot_index && *p == position_index
                            })
                            .map(|(_, _, _, score)| *score),
                        "environment {index} k={k}: scores must agree at \
                        ({subset_index}, {hand_slot_index}, {position_index})"
                    );
                }
            }
        }
    }

    /// Card-id-set identity for a `BuildTower` action, or `None` for any
    /// other action kind. `action_id()` and `PartialEq` are sensitive to
    /// `card_ids`' *order*, but a `BuildTower` selecting the same card
    /// *set* (any order) at the same hand slot and position is the same
    /// legal action - `card_ids_are_selectable` only checks membership.
    /// Comparing oracle vs. dense-table actions must use this, not raw
    /// identity.
    fn build_tower_identity_key(action: &AgentAction) -> Option<(Vec<usize>, usize, usize, usize)> {
        let AgentAction::BuildTower {
            card_ids,
            hand_slot_index,
            left,
            top,
        } = action
        else {
            return None;
        };
        let mut sorted_card_ids = card_ids.clone();
        sorted_card_ids.sort_unstable();
        Some((sorted_card_ids, *hand_slot_index, *left, *top))
    }

    /// Exhaustive bidirectional check that the dense `BuildTower` space
    /// exactly represents the oracle legal `BuildTower` set
    /// (`GameEnvironment::semantic_legal_actions`, the exhaustive/unpruned
    /// semantic legal-action generator), including states with
    /// `extra_tower_cards` (multi build slot). Covers:
    /// - every oracle-legal `BuildTower` action is scored in the dense
    ///   table (oracle subset dense);
    /// - every dense-scored `(subset, hand_slot, position)` materializes an
    ///   action that is itself oracle-legal (dense subset oracle) - i.e.
    ///   support is identical, not just overlapping;
    /// - extra-slot (`hand_slot_index >= 1`) scores are identical across
    ///   every card subset, since that tower's template never depends on
    ///   which subset was selected - the concrete form of "slot identity
    ///   doesn't depend on subset/order" for this axis.
    ///
    /// Release-only: the oracle side materializes every `BuildTower`
    /// action up front (the O(subset x position) allocation the dense path
    /// exists to avoid), same cost profile as
    /// `dense_scorer_matches_exhaustive_heuristic_oracle`.
    #[test]
    #[ignore = "exhaustive oracle comparison; release-only, same reason as \
        joint_action::tests::dense_scorer_matches_exhaustive_heuristic_oracle - run with \
        cargo test --release -- --ignored dense_build_tower_support_matches_oracle_bidirectionally"]
    fn dense_build_tower_support_matches_oracle_bidirectionally() {
        use crate::joint_action::{DenseBuildTowerScoreTable, build_tower_action, joint_index_for_action};
        use std::collections::HashSet;

        let environments = (0..3u64)
            .map(card_decision_environment)
            .chain([
                extra_tower_cards_environment(1),
                extra_tower_cards_environment(2),
            ])
            .collect::<Vec<_>>();

        for (index, environment) in environments.iter().enumerate() {
            let observation = environment.snapshot();
            let table = DenseBuildTowerScoreTable::compute(environment, &observation);
            let oracle_build_actions = environment
                .semantic_legal_actions()
                .into_iter()
                .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                .map(|legal| legal.action)
                .collect::<Vec<_>>();
            // `action_id()`/`PartialEq` are card_ids-order-sensitive, but
            // legality/identity is a card *set* (see
            // `GameEnvironment::semantic_action_is_legal`'s
            // `card_ids_are_selectable`, which is membership-only) - the
            // oracle's generation-order card_ids and the dense table's
            // id-sorted card_ids (`CardSubsetTable`) can legitimately
            // differ only in that order. Compare by a card-id-sorted key
            // instead of raw identity/`action_id()`.
            let oracle_build_action_ids = oracle_build_actions
                .iter()
                .map(build_tower_identity_key)
                .collect::<HashSet<_>>();
            assert!(
                !oracle_build_actions.is_empty(),
                "environment {index}: fixture should have at least one legal BuildTower action"
            );

            // oracle -> dense: every oracle-legal action must be scored.
            for action in &oracle_build_actions {
                let (subset_index, hand_slot_index, position_index) =
                    joint_index_for_action(&table.subsets, action).unwrap_or_else(|| {
                        panic!("environment {index}: oracle action {action:?} should map to a joint index")
                    });
                assert!(
                    table.score(subset_index, hand_slot_index, position_index).is_some(),
                    "environment {index}: oracle action {action:?} \
                    (subset={subset_index}, hand_slot={hand_slot_index}, position={position_index}) \
                    must be scored by the dense table"
                );
            }

            // dense -> oracle: every scored triple must materialize an
            // oracle-legal action, and extra-slot scores must be
            // subset-independent.
            let mut extra_slot_scores: std::collections::HashMap<
                (usize, usize),
                crate::joint_action::JointBuildTowerScore,
            > = std::collections::HashMap::new();
            for subset_index in 0..table.subsets.subset_count() {
                for hand_slot_index in 0..table.build_slot_count {
                    for position_index in 0..table.position_count {
                        let Some(score) = table.score(subset_index, hand_slot_index, position_index)
                        else {
                            continue;
                        };
                        let action = build_tower_action(
                            &table.subsets,
                            subset_index,
                            hand_slot_index,
                            position_index,
                        )
                        .unwrap_or_else(|| {
                            panic!(
                                "environment {index}: scored triple ({subset_index}, \
                                {hand_slot_index}, {position_index}) should materialize an action"
                            )
                        });
                        assert!(
                            oracle_build_action_ids.contains(&build_tower_identity_key(&action)),
                            "environment {index}: dense-scored action {action:?} \
                            (subset={subset_index}, hand_slot={hand_slot_index}, \
                            position={position_index}) must be oracle-legal"
                        );
                        if hand_slot_index >= 1 {
                            let key = (hand_slot_index, position_index);
                            if let Some(&existing) = extra_slot_scores.get(&key) {
                                assert_eq!(
                                    existing, score,
                                    "environment {index}: hand_slot {hand_slot_index} at position \
                                    {position_index} must score identically regardless of card \
                                    subset (subset {subset_index} disagreed)"
                                );
                            } else {
                                extra_slot_scores.insert(key, score);
                            }
                        }
                    }
                }
            }
        }
    }

    /// `dense_semantic_candidates`'s `BuildTower` portion must be exactly
    /// `DenseBuildTowerScoreTable::top_k_actions(k)` - i.e. wiring the
    /// production candidate path to the dense table doesn't lose or
    /// reorder anything relative to the table itself.
    #[test]
    fn dense_semantic_candidates_build_tower_portion_matches_table_top_k() {
        for seed in 0..4u64 {
            let environment = card_decision_environment(seed);
            let observation = environment.snapshot();
            let table = DenseBuildTowerScoreTable::compute(&environment, &observation);
            for k in [1usize, 8, 32] {
                let candidates = dense_semantic_candidates(&environment, Some(k));
                let build_tower_ids = candidates
                    .iter()
                    .filter(|candidate| {
                        matches!(candidate.action, AgentAction::BuildTower { .. })
                    })
                    .map(|candidate| candidate.id.clone())
                    .collect::<Vec<_>>();
                let expected_ids = table
                    .top_k_actions(k)
                    .into_iter()
                    .map(|action| action.action_id())
                    .collect::<Vec<_>>();
                assert_eq!(
                    build_tower_ids, expected_ids,
                    "seed {seed} k={k}: candidate BuildTower ids must match the dense table's top-K exactly"
                );
            }
        }
    }

    /// `top_k_actions`/`dense_semantic_candidates` must never spend rollout
    /// budget twice on the same semantic action - each `(subset_index,
    /// hand_slot_index, position_index)` triple the table scores is
    /// distinct by construction (see
    /// `joint_action::tests::no_two_distinct_joint_indices_materialize_the_same_action`),
    /// so this is really a top-K/materialization wiring check, not a fresh
    /// proof, but it's the check that would actually catch a regression in
    /// production's candidate list.
    #[test]
    fn dense_top_k_never_duplicates_an_action() {
        for environment in [
            card_decision_environment(0),
            extra_tower_cards_environment(1),
            extra_tower_cards_environment(2),
        ] {
            let observation = environment.snapshot();
            let table = DenseBuildTowerScoreTable::compute(&environment, &observation);
            let mut seen = std::collections::HashSet::new();
            for action in table.top_k_actions(usize::MAX) {
                assert!(
                    seen.insert(action.action_id()),
                    "top_k_actions produced a duplicate action: {action:?}"
                );
            }
        }
    }

    /// Extra `BuildTower` slots (`hand_slot_index >= 1`) place a tower
    /// whose stats are subset-independent (a fixed
    /// `stage_modifiers.extra_tower_cards` template - see
    /// `extra_slot_templates`), but the *resulting game state* is not:
    /// `SelectTower` still builds the primary-slot (0) template from the
    /// chosen subset and leaves it queued in `hand.slots[0]` for a future
    /// placement, so two `BuildTower` actions that differ only in which
    /// subset was selected (same `hand_slot_index >= 1`, same position)
    /// leave a genuinely different follow-up tower option behind. This is
    /// why the dense representation keeps a full `subset_index` axis for
    /// every `hand_slot_index`, not just slot `0` - collapsing it would
    /// conflate two actions with different consequences.
    #[test]
    fn extra_slot_actions_differing_only_by_subset_produce_different_states() {
        let mut environment = card_decision_environment(0);
        environment
            .test_only_seed_extra_tower_cards(1)
            .expect("extra tower card fixture should be a valid snapshot");
        let observation = environment.snapshot();
        let subsets = crate::joint_action::CardSubsetTable::from_observation(&observation);
        assert!(
            subsets.subset_count() >= 2,
            "fixture needs at least two card subsets to compare"
        );
        let subset_a = subsets.card_ids_for_subset(0).expect("subset 0 exists");
        let subset_b = subsets.card_ids_for_subset(1).expect("subset 1 exists");
        assert_ne!(subset_a, subset_b);

        let mut env_a = environment.fork_for_rollout_seed(0).expect("fork should succeed");
        env_a
            .semantic_step(AgentAction::BuildTower {
                card_ids: subset_a,
                hand_slot_index: 1,
                left: 0,
                top: 0,
            })
            .expect("placing the extra slot should be legal");

        let mut env_b = environment.fork_for_rollout_seed(0).expect("fork should succeed");
        env_b
            .semantic_step(AgentAction::BuildTower {
                card_ids: subset_b,
                hand_slot_index: 1,
                left: 0,
                top: 0,
            })
            .expect("placing the extra slot should be legal");

        // The placed tower itself is identical (same fixed extra-slot
        // template, same position)...
        assert_eq!(env_a.snapshot().towers, env_b.snapshot().towers);
        // ...but the overall resulting state is not: hand.slots[0] (the
        // still-unplaced primary-subset tower) differs.
        assert_ne!(
            env_a.state_hash(),
            env_b.state_hash(),
            "different card subsets at the same extra hand_slot_index/position must not collapse \
            to the same resulting state - the queued primary-slot tower still differs"
        );
    }

    /// Every legal non-`BuildTower` semantic action must always be present
    /// in `dense_semantic_candidates`' output, regardless of how small the
    /// `BuildTower` search budget is - it must never be dropped just
    /// because it sorts after `BuildTower` candidates in some generation
    /// order (the old `select_candidates_fairly`/prefix-truncate bias this
    /// migration replaces).
    #[test]
    fn dense_semantic_candidates_always_keeps_every_non_build_action() {
        for seed in 0..4u64 {
            let environment = card_decision_environment(seed);
            let expected_non_build = environment
                .semantic_non_build_actions()
                .into_iter()
                .map(|legal| legal.id)
                .collect::<std::collections::BTreeSet<_>>();
            for build_tower_limit in [Some(0), Some(1), None] {
                let candidates = dense_semantic_candidates(&environment, build_tower_limit);
                let actual_non_build = candidates
                    .iter()
                    .filter(|candidate| {
                        !matches!(candidate.action, AgentAction::BuildTower { .. })
                    })
                    .map(|candidate| candidate.id.clone())
                    .collect::<std::collections::BTreeSet<_>>();
                assert_eq!(
                    actual_non_build, expected_non_build,
                    "seed {seed} build_tower_limit={build_tower_limit:?}: non-BuildTower candidates must survive intact"
                );
            }
        }
    }

    /// The dense `BuildTower` ranking is keyed by the *set* of held card
    /// ids (`joint_action::CardSubsetTable` sorts by card id), never by
    /// which physical hand slot a card occupies - so `subset_index` is
    /// stable under any reordering of the same card id set.
    #[test]
    fn card_subset_index_is_independent_of_card_id_input_order() {
        let ascending = crate::joint_action::CardSubsetTable::from_hand_card_ids([2, 5, 9, 13]);
        let shuffled = crate::joint_action::CardSubsetTable::from_hand_card_ids([13, 2, 9, 5]);
        assert_eq!(ascending.subset_count(), shuffled.subset_count());
        for subset_index in 0..ascending.subset_count() {
            assert_eq!(
                ascending.card_ids_for_subset(subset_index),
                shuffled.card_ids_for_subset(subset_index),
                "subset_index {subset_index} must resolve to the same card ids regardless of construction order"
            );
        }
        assert_eq!(
            ascending.subset_index_for_card_ids(&[9, 2]),
            shuffled.subset_index_for_card_ids(&[2, 9]),
            "the same card id set must resolve to the same subset_index regardless of query order"
        );
    }

    /// Production correctness must not depend on `position_candidate_limit`:
    /// find a decision where the oracle-best `BuildTower` action (by the
    /// same heuristic Phase 1 used) sits outside the old
    /// `DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT`-nearest positions -
    /// Phase 1's corrected (decoupled-corpus) benchmark showed this
    /// actually happens ~26% of the time at limit=64, not the 0% the stale
    /// pre-migration artifact implied - and confirm `dense_semantic_candidates`
    /// still finds it while the legacy position-limited proposal does not.
    #[test]
    fn dense_semantic_candidates_finds_build_tower_actions_outside_legacy_position_limit() {
        use crate::policy_runner::{rank_build_tower_actions_by_heuristic, scripted_expert_action};

        let mut found_case = false;
        for seed in 0..24u64 {
            let mut environment = card_decision_environment(seed);
            for _decision in 0..24 {
                let observation = environment.snapshot();
                let oracle_build_actions = environment
                    .semantic_legal_actions()
                    .into_iter()
                    .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                    .collect::<Vec<_>>();
                if !oracle_build_actions.is_empty() {
                    let oracle_ranking =
                        rank_build_tower_actions_by_heuristic(&observation, &oracle_build_actions);
                    let oracle_best = oracle_ranking
                        .first()
                        .expect("non-empty oracle candidates rank at least one");
                    let legacy_limited = environment
                        .semantic_legal_actions_with_position_limit(Some(
                            DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
                        ))
                        .into_iter()
                        .any(|legal| legal.action == oracle_best.action);
                    if !legacy_limited {
                        found_case = true;
                        let dense_candidates = dense_semantic_candidates(&environment, None);
                        assert!(
                            dense_candidates
                                .iter()
                                .any(|candidate| candidate.action == oracle_best.action),
                            "seed {seed}: dense_semantic_candidates must find the oracle-best \
                            BuildTower action even when it sits outside the legacy \
                            position_candidate_limit window"
                        );
                        break;
                    }
                }

                let legal_actions = environment.semantic_legal_actions();
                let Ok(action) = scripted_expert_action(&observation, &legal_actions) else {
                    break;
                };
                let Ok(outcome) = environment.semantic_step(action) else {
                    break;
                };
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
            if found_case {
                break;
            }
        }
        assert!(
            found_case,
            "expected at least one sampled decision where the oracle best sits outside the legacy position limit"
        );
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

                // Trajectory advancement must stay independent of LIMITS: it
                // picks which 144 states this report measures, and
                // DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT is one of the
                // values under evaluation here (LIMITS.max()). Driving the
                // trajectory off it would make "is this limit enough"
                // self-referential - the state corpus would already have
                // been walked by an agent that had that limit's candidates
                // available. Use the unlimited legal action set instead, so
                // the same 144 states are sampled regardless of which
                // position limit LIMITS or the production default use.
                let legal_actions = environment.semantic_legal_actions();
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

                            // HashMap iteration order is randomized per
                            // process, so collect then sort by kind for a
                            // deterministic report across runs.
                            let mut action_kind_survival: Vec<_> = action_kind_pre_counts
                                .keys()
                                .map(|&kind| ActionKindSurvival {
                                    kind: kind.to_string(),
                                    pre_truncation_count: action_kind_pre_counts[kind],
                                    survived_count: *action_kind_survived.get(kind).unwrap_or(&0),
                                })
                                .collect();
                            action_kind_survival.sort_by(|a, b| a.kind.cmp(&b.kind));

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

    #[derive(serde::Serialize)]
    struct MigrationBenchmarkSamplePoint {
        seed: u64,
        decision_index: usize,
        legacy_candidate_generation_seconds: f64,
        legacy_materialized_build_tower_count: usize,
        legacy_candidate_count: usize,
        legacy_achieves_oracle_best_score: bool,
        legacy_teacher_decision_seconds: f64,
        dense_candidate_generation_seconds: f64,
        dense_materialized_build_tower_count: usize,
        dense_candidate_count: usize,
        dense_achieves_oracle_best_score: bool,
        dense_teacher_decision_seconds: f64,
    }

    #[derive(serde::Serialize)]
    struct MigrationBenchmarkReport {
        methodology: String,
        target_candidate_count: usize,
        scenario_count: usize,
        horizon_decisions: usize,
        sample_points: Vec<MigrationBenchmarkSamplePoint>,
        mean_legacy_candidate_generation_seconds: f64,
        mean_dense_candidate_generation_seconds: f64,
        candidate_generation_speedup: f64,
        mean_legacy_materialized_build_tower_count: f64,
        mean_dense_materialized_build_tower_count: f64,
        legacy_oracle_best_score_achievement_rate: f64,
        dense_oracle_best_score_achievement_rate: f64,
        mean_legacy_teacher_decision_seconds: f64,
        mean_dense_teacher_decision_seconds: f64,
        teacher_decision_speedup: f64,
        legacy_decisions_per_sec: f64,
        dense_decisions_per_sec: f64,
    }

    /// Compares the legacy position-proposal + `select_candidates_fairly`
    /// candidate path against `dense_semantic_candidates` on the same
    /// sampled decision states, with the total candidate (and therefore
    /// rollout) count held equal between the two so the comparison isolates
    /// candidate-generation cost/quality from rollout-budget differences.
    ///
    /// - `legacy_materialized_build_tower_count`: `BuildTower` actions
    ///   actually allocated before truncation
    ///   (`semantic_legal_actions_with_position_limit`'s output) - the
    ///   thing `dense_semantic_candidates` is built to avoid materializing.
    /// - `dense_materialized_build_tower_count`: `BuildTower` actions
    ///   allocated by `dense_semantic_candidates`, which only ever builds
    ///   its top-K (plus the non-`BuildTower` set).
    /// - `*_achieves_oracle_best_score`: whether the candidate set's own
    ///   best `BuildTower` action, re-ranked by the same rollout-free
    ///   heuristic Phase 1 used, *scores* exactly as well as the unpruned
    ///   oracle's best (NOT a rollout/post-hoc best). Compared by score
    ///   equality, not action identity - see the report's `methodology`
    ///   string for why: exact ties at the top heuristic score are common
    ///   (e.g. a subset and its superset that reduce to the same
    ///   poker-hand pattern), so a candidate set can legitimately win by
    ///   holding a *different* member of the same tied score, not the
    ///   oracle's specific pick.
    /// - `*_teacher_decision_seconds`: `evaluate_semantic_candidate_set`
    ///   end to end (real rollouts, `scenario_count` scenarios x
    ///   `horizon_decisions` per candidate) - `legacy` and `dense` use the
    ///   same candidate *count*, so this isolates candidate-generation
    ///   overhead plus any oracle-best-inclusion effect on rollout value,
    ///   not a rollout-budget difference.
    ///
    /// Run with: cargo test --release -- --ignored dense_candidate_migration_benchmark --nocapture
    #[test]
    #[ignore = "manual release benchmark; writes artifacts/benchmarks/dense-candidate-migration-benchmark.json"]
    fn dense_candidate_migration_benchmark() {
        use crate::policy_runner::rank_build_tower_actions_by_heuristic;
        use std::time::Instant;

        // Kept small on purpose: each sample point runs
        // TARGET_CANDIDATE_COUNT * SCENARIO_COUNT real rollout forks per
        // candidate path (this is exactly the rollout cost candidate_limit
        // exists to bound), so this benchmark's cost scales with their
        // product, not with candidate-generation cost alone.
        const SEEDS: std::ops::Range<u64> = 0..3;
        const SAMPLES_PER_SEED: usize = 2;
        const MAX_DECISIONS_PER_SEED: usize = 40;
        const TARGET_CANDIDATE_COUNT: usize = 96;
        const SCENARIO_COUNT: usize = 1;
        const HORIZON_DECISIONS: usize = 1;

        let scenario_seeds = (0..SCENARIO_COUNT as u64).collect::<Vec<_>>();

        let mut sample_points = Vec::new();
        for seed in SEEDS {
            let mut environment = card_decision_environment(seed);
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
                    let oracle_best = oracle_ranking
                        .first()
                        .expect("non-empty oracle candidates rank at least one");
                    // The oracle score, not a specific action: many actions
                    // routinely tie for the top heuristic score (a card
                    // subset and the strict superset that reduces to the
                    // same poker-hand pattern always tie, for example), so
                    // "did this candidate set reach the achievable best" is
                    // the meaningful question, not "does it contain this
                    // exact one of the tied actions" - see the report's
                    // methodology string.
                    let oracle_best_score =
                        (oracle_best.covered_route, oracle_best.nearest_route, oracle_best.damage_raw);

                    let non_build_count = environment.semantic_non_build_actions().len();
                    let dense_build_tower_budget =
                        TARGET_CANDIDATE_COUNT.saturating_sub(non_build_count).max(1);

                    let legacy_gen_start = Instant::now();
                    let legacy_pre_truncation = environment
                        .semantic_legal_actions_with_position_limit(Some(
                            DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
                        ));
                    let legacy_materialized_build_tower_count = legacy_pre_truncation
                        .iter()
                        .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                        .count();
                    let legacy_candidates =
                        select_candidates_fairly(legacy_pre_truncation, TARGET_CANDIDATE_COUNT);
                    let legacy_candidate_generation_seconds =
                        legacy_gen_start.elapsed().as_secs_f64();
                    let legacy_build_tower_candidates = legacy_candidates
                        .iter()
                        .filter(|candidate| matches!(candidate.action, AgentAction::BuildTower { .. }))
                        .cloned()
                        .collect::<Vec<_>>();
                    let legacy_achieves_oracle_best_score = !legacy_build_tower_candidates.is_empty()
                        && rank_build_tower_actions_by_heuristic(
                            &observation,
                            &legacy_build_tower_candidates,
                        )
                        .first()
                        .is_some_and(|best| {
                            (best.covered_route, best.nearest_route, best.damage_raw)
                                == oracle_best_score
                        });

                    let dense_gen_start = Instant::now();
                    let dense_candidates = dense_semantic_candidates(
                        &environment,
                        Some(dense_build_tower_budget),
                    );
                    let dense_candidate_generation_seconds =
                        dense_gen_start.elapsed().as_secs_f64();
                    let dense_materialized_build_tower_count = dense_candidates
                        .iter()
                        .filter(|candidate| matches!(candidate.action, AgentAction::BuildTower { .. }))
                        .count();
                    // Can't reuse rank_build_tower_actions_by_heuristic here
                    // the way legacy does above: it matches an action's
                    // card_ids against observation.build_tower_candidates
                    // by exact (unsorted) Vec equality, but dense
                    // candidates' card_ids come from CardSubsetTable
                    // (id-sorted, by design - see joint_action's module
                    // docs) and would silently fail that lookup whenever
                    // sorted order differs from build_tower_candidates'
                    // hand-slot-order entries. Read the score straight from
                    // the dense table instead, which top_k_indices already
                    // ranks by the identical formula.
                    let table = DenseBuildTowerScoreTable::compute(&environment, &observation);
                    let dense_achieves_oracle_best_score = table
                        .top_k_indices(dense_build_tower_budget)
                        .first()
                        .and_then(|&(subset_index, hand_slot_index, position_index)| {
                            table.score(subset_index, hand_slot_index, position_index)
                        })
                        .is_some_and(|best| {
                            (best.covered_route, best.nearest_route, best.damage_raw)
                                == oracle_best_score
                        });

                    let teacher_config = RolloutTeacherConfig {
                        scenario_seeds: scenario_seeds.clone(),
                        horizon_decisions: HORIZON_DECISIONS,
                        build_tower_rollout_limit: None,
                    };
                    let legacy_decision_start = Instant::now();
                    evaluate_semantic_candidate_set(
                        &environment,
                        &legacy_candidates,
                        &teacher_config,
                    )
                    .expect("legacy candidate set should evaluate");
                    let legacy_teacher_decision_seconds =
                        legacy_decision_start.elapsed().as_secs_f64();

                    let dense_decision_start = Instant::now();
                    evaluate_semantic_candidate_set(
                        &environment,
                        &dense_candidates,
                        &teacher_config,
                    )
                    .expect("dense candidate set should evaluate");
                    let dense_teacher_decision_seconds =
                        dense_decision_start.elapsed().as_secs_f64();

                    sample_points.push(MigrationBenchmarkSamplePoint {
                        seed,
                        decision_index,
                        legacy_candidate_generation_seconds,
                        legacy_materialized_build_tower_count,
                        legacy_candidate_count: legacy_candidates.len(),
                        legacy_achieves_oracle_best_score,
                        legacy_teacher_decision_seconds,
                        dense_candidate_generation_seconds,
                        dense_materialized_build_tower_count,
                        dense_candidate_count: dense_candidates.len(),
                        dense_achieves_oracle_best_score,
                        dense_teacher_decision_seconds,
                    });
                    samples_collected += 1;
                }

                let legal_actions = environment.semantic_legal_actions();
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

        let count = sample_points.len().max(1) as f64;
        let mean = |values: Vec<f64>| values.iter().sum::<f64>() / count;
        let mean_legacy_candidate_generation_seconds = mean(
            sample_points
                .iter()
                .map(|s| s.legacy_candidate_generation_seconds)
                .collect(),
        );
        let mean_dense_candidate_generation_seconds = mean(
            sample_points
                .iter()
                .map(|s| s.dense_candidate_generation_seconds)
                .collect(),
        );
        let mean_legacy_materialized_build_tower_count = mean(
            sample_points
                .iter()
                .map(|s| s.legacy_materialized_build_tower_count as f64)
                .collect(),
        );
        let mean_dense_materialized_build_tower_count = mean(
            sample_points
                .iter()
                .map(|s| s.dense_materialized_build_tower_count as f64)
                .collect(),
        );
        let legacy_oracle_best_score_achievement_rate = mean(
            sample_points
                .iter()
                .map(|s| s.legacy_achieves_oracle_best_score as u8 as f64)
                .collect(),
        );
        let dense_oracle_best_score_achievement_rate = mean(
            sample_points
                .iter()
                .map(|s| s.dense_achieves_oracle_best_score as u8 as f64)
                .collect(),
        );
        let mean_legacy_teacher_decision_seconds = mean(
            sample_points
                .iter()
                .map(|s| s.legacy_teacher_decision_seconds)
                .collect(),
        );
        let mean_dense_teacher_decision_seconds = mean(
            sample_points
                .iter()
                .map(|s| s.dense_teacher_decision_seconds)
                .collect(),
        );
        let candidate_generation_speedup = mean_legacy_candidate_generation_seconds
            / mean_dense_candidate_generation_seconds.max(f64::EPSILON);
        let teacher_decision_speedup = mean_legacy_teacher_decision_seconds
            / mean_dense_teacher_decision_seconds.max(f64::EPSILON);
        let legacy_decisions_per_sec =
            1.0 / mean_legacy_teacher_decision_seconds.max(f64::EPSILON);
        let dense_decisions_per_sec = 1.0 / mean_dense_teacher_decision_seconds.max(f64::EPSILON);

        println!(
            "sample_count={} mean_legacy_candidate_generation_seconds={:.6} \
            mean_dense_candidate_generation_seconds={:.6} candidate_generation_speedup={:.2}x \
            mean_legacy_materialized_build_tower_count={:.1} \
            mean_dense_materialized_build_tower_count={:.1} \
            legacy_oracle_best_score_achievement_rate={:.4} dense_oracle_best_score_achievement_rate={:.4} \
            mean_legacy_teacher_decision_seconds={:.6} mean_dense_teacher_decision_seconds={:.6} \
            teacher_decision_speedup={:.2}x legacy_decisions_per_sec={:.2} dense_decisions_per_sec={:.2}",
            sample_points.len(),
            mean_legacy_candidate_generation_seconds,
            mean_dense_candidate_generation_seconds,
            candidate_generation_speedup,
            mean_legacy_materialized_build_tower_count,
            mean_dense_materialized_build_tower_count,
            legacy_oracle_best_score_achievement_rate,
            dense_oracle_best_score_achievement_rate,
            mean_legacy_teacher_decision_seconds,
            mean_dense_teacher_decision_seconds,
            teacher_decision_speedup,
            legacy_decisions_per_sec,
            dense_decisions_per_sec,
        );

        let report = MigrationBenchmarkReport {
            methodology: "legacy candidates come from \
                semantic_legal_actions_with_position_limit(DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT) \
                + select_candidates_fairly(TARGET_CANDIDATE_COUNT) (the pre-migration production \
                path); dense candidates come from dense_semantic_candidates with a BuildTower budget \
                sized so total candidate count matches TARGET_CANDIDATE_COUNT, keeping rollout count \
                comparable between the two. oracle_best_score is the rollout-free HEURISTIC oracle's \
                best score (rank_build_tower_actions_by_heuristic over the full, unpruned BuildTower \
                set) - not a rollout/post-hoc best. *_achieves_oracle_best_score compares the \
                candidate set's own best-scoring BuildTower action's SCORE against oracle_best_score, \
                not action identity: the top heuristic score is routinely a wide tie (a card subset \
                and any superset that reduces to the same poker-hand pattern always tie, for \
                instance - one sampled decision had 93 oracle actions tied for the top score), so \
                requiring the exact same tied action would conflate 'this budget doesn't reach the \
                achievable best' with 'this budget reached an equally-good but different tied \
                action', which dense_scorer_matches_exhaustive_heuristic_oracle and \
                dense_top_k_matches_brute_force_ranking already prove happens constantly by design \
                (deterministic tie-break on (subset_index, hand_slot_index, position_index), not \
                action_id()). dense_achieves_oracle_best_score is computed by reading the score \
                straight from DenseBuildTowerScoreTable (not by re-ranking dense_semantic_candidates' \
                output through rank_build_tower_actions_by_heuristic, which matches an action's \
                card_ids against observation.build_tower_candidates by exact unsorted-Vec equality \
                and would silently drop every dense candidate whose CardSubsetTable-sorted card_ids \
                don't happen to equal build_tower_candidates' hand-slot-order entries). \
                teacher_decision_seconds times evaluate_semantic_candidate_set end to end (real \
                rollouts) for each candidate set."
                .to_string(),
            target_candidate_count: TARGET_CANDIDATE_COUNT,
            scenario_count: SCENARIO_COUNT,
            horizon_decisions: HORIZON_DECISIONS,
            sample_points,
            mean_legacy_candidate_generation_seconds,
            mean_dense_candidate_generation_seconds,
            candidate_generation_speedup,
            mean_legacy_materialized_build_tower_count,
            mean_dense_materialized_build_tower_count,
            legacy_oracle_best_score_achievement_rate,
            dense_oracle_best_score_achievement_rate,
            mean_legacy_teacher_decision_seconds,
            mean_dense_teacher_decision_seconds,
            teacher_decision_speedup,
            legacy_decisions_per_sec,
            dense_decisions_per_sec,
        };

        let json = serde_json::to_string_pretty(&report).expect("report should serialize");
        std::fs::create_dir_all("../artifacts/benchmarks")
            .expect("benchmark artifact directory should be creatable");
        std::fs::write(
            "../artifacts/benchmarks/dense-candidate-migration-benchmark.json",
            &json,
        )
        .expect("benchmark report should be written");
    }
}
