//! Phase 3 teacher evaluation harness: canonical-baseline episode runner,
//! label/cost stability comparison across scenario/horizon/build-tower
//! rollout budget settings, and a paired full-game foundation comparing the
//! canonical heuristic baseline against the rollout teacher.
//!
//! This module does not itself decide whether the rollout teacher is
//! "strong enough" - the stability report and the paired full-game report
//! are diagnostics for a future held-out strength gate, not the gate
//! itself. See docs/game-ai/05-rollout-teacher.md and
//! docs/game-ai/08-evaluation.md.

use std::sync::Arc;
use std::time::Instant;

use anyhow::{Result, bail};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::GameConfig;
use crate::environment::{DecisionPoint, GameEnvironment};
use crate::policy_runner::canonical_scripted_semantic_action;
use crate::teacher::{
    RolloutTeacherConfig, TEACHER_SCORE_SCHEMA_VERSION, evaluate_semantic_candidate_set_with_baseline,
    prepare_semantic_candidates, run_semantic_teacher_episode, scenario_seed_digest,
    settle_forced_actions,
};

/// A "large regret" threshold fixed before any stability results are seen -
/// per docs/game-ai/05-rollout-teacher.md's score contract
/// (`stage_progress_v1`, terminal victory bonus 1,000), 0.05 is a fraction
/// of one stage's progress. Recorded in report metadata, not tuned post-hoc.
pub const LARGE_REGRET_THRESHOLD: f32 = 0.05;

// --- Canonical scripted-semantic episode/batch runner (item 9) --------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanonicalEpisodeResult {
    pub seed: u64,
    pub max_decisions: usize,
    pub decision_count: usize,
    pub terminated: bool,
    pub truncated: bool,
    pub victory: bool,
    pub clear_rate: f32,
    pub final_stage: usize,
    pub final_state_hash: String,
}

/// Runs one episode driven entirely by `canonical_scripted_semantic_action` -
/// the same canonical heuristic used as the teacher's baseline and
/// continuation policy (see `policy_runner::canonical_scripted_semantic_action`),
/// not `benchmark::run_semantic_batch`'s `DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT`
/// legacy path. Forced-action settling and termination/max_decisions
/// semantics match `teacher::run_semantic_teacher_episode`.
pub fn run_canonical_scripted_semantic_episode(
    game_config: Arc<GameConfig>,
    seed: u64,
    max_decisions: usize,
) -> Result<CanonicalEpisodeResult> {
    if max_decisions == 0 {
        bail!("canonical scripted semantic episode max decisions must be positive");
    }
    let mut environment = GameEnvironment::new(game_config, seed);
    let mut decision_count = 0;
    let mut terminated = false;
    let mut truncated = false;
    while decision_count < max_decisions {
        if matches!(environment.decision_point(), DecisionPoint::Terminal) {
            terminated = true;
            break;
        }
        let action = canonical_scripted_semantic_action(&environment)?;
        let mut outcome = environment
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("canonical episode action failed: {error:?}"))?;
        settle_forced_actions(&mut environment, &mut outcome)?;
        terminated = outcome.terminated;
        truncated = outcome.truncated;
        decision_count += 1;
        if terminated || truncated {
            break;
        }
    }
    if !terminated && !truncated && decision_count == max_decisions {
        truncated = true;
    }
    let observation = environment.snapshot();
    Ok(CanonicalEpisodeResult {
        seed,
        max_decisions,
        decision_count,
        terminated,
        truncated,
        victory: terminated && environment.clear_rate() >= 100.0,
        clear_rate: environment.clear_rate(),
        final_stage: observation.stage,
        final_state_hash: environment.state_hash(),
    })
}

pub fn run_canonical_scripted_semantic_batch(
    game_config: Arc<GameConfig>,
    seeds: &[u64],
    max_decisions: usize,
) -> Result<Vec<CanonicalEpisodeResult>> {
    let mut results = seeds
        .par_iter()
        .map(|&seed| run_canonical_scripted_semantic_episode(Arc::clone(&game_config), seed, max_decisions))
        .collect::<Result<Vec<_>>>()?;
    results.sort_by_key(|episode| episode.seed);
    Ok(results)
}

// --- Stability experiment grid contract (items 10-12) ------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilityGridConfig {
    pub seed_start: u64,
    pub seed_end: u64,
    pub max_decisions: usize,
    pub state_limit_per_seed: usize,
    pub scenario_seed_start: u64,
    pub scenario_counts: Vec<usize>,
    pub horizon_decisions: Vec<usize>,
    pub build_tower_rollout_limits: Vec<Option<usize>>,
}

impl StabilityGridConfig {
    fn validate(&self) -> Result<()> {
        if self.seed_end < self.seed_start {
            bail!("seed_end must be >= seed_start");
        }
        if self.max_decisions == 0 {
            bail!("max_decisions must be positive");
        }
        if self.state_limit_per_seed == 0 {
            bail!("state_limit_per_seed must be positive");
        }
        if self.scenario_counts.is_empty() || self.scenario_counts.iter().any(|&count| count == 0) {
            bail!("scenario_counts must be non-empty and positive");
        }
        if self.horizon_decisions.is_empty() || self.horizon_decisions.iter().any(|&h| h == 0) {
            bail!("horizon_decisions must be non-empty and positive");
        }
        if self.build_tower_rollout_limits.is_empty()
            || self
                .build_tower_rollout_limits
                .iter()
                .any(|limit| *limit == Some(0))
        {
            bail!("build_tower_rollout_limits must be non-empty and positive when set");
        }
        Ok(())
    }

    /// The reference setting: largest `scenario_count`, largest
    /// `horizon_decisions`, largest `build_tower_rollout_limit` (`None`,
    /// i.e. unlimited, counts as the largest). Not "ground truth" - just the
    /// most expensive combination this grid evaluates, used as a stability
    /// comparison anchor.
    fn reference_setting(&self) -> (usize, usize, Option<usize>) {
        let scenario_count = *self.scenario_counts.iter().max().expect("non-empty");
        let horizon = *self.horizon_decisions.iter().max().expect("non-empty");
        let build_tower_rollout_limit = self
            .build_tower_rollout_limits
            .iter()
            .copied()
            .max_by_key(|limit| limit.unwrap_or(usize::MAX))
            .expect("non-empty");
        (scenario_count, horizon, build_tower_rollout_limit)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StabilityStateConfigRecord {
    pub seed: u64,
    pub decision_index: usize,
    pub state_hash: String,
    pub decision_point: String,
    pub scenario_count: usize,
    pub horizon_decisions: usize,
    pub build_tower_rollout_limit: Option<usize>,
    pub candidate_count: usize,
    pub selected_action_id: String,
    pub baseline_action_id: String,
    pub selected_mean_score: f32,
    pub baseline_mean_score: f32,
    pub expert_regret: f32,
    pub selected_standard_error: f32,
    pub baseline_standard_error: f32,
    pub score_margin: Option<f32>,
    pub elapsed_seconds: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StabilityAggregate {
    pub sample_count: usize,
    pub agreement_rate_vs_reference: f64,
    pub agreement_rate_by_decision_point: std::collections::BTreeMap<String, f64>,
    pub mean_expert_regret: f64,
    pub median_expert_regret: f64,
    pub fraction_positive_regret: f64,
    pub fraction_large_regret: f64,
    pub mean_candidate_count: f64,
    pub mean_wall_time_seconds: f64,
    pub scenario_rollout_count: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StabilityReport {
    pub grid: StabilityGridConfig,
    pub large_regret_threshold: f32,
    pub reference_scenario_count: usize,
    pub reference_horizon_decisions: usize,
    pub reference_build_tower_rollout_limit: Option<usize>,
    pub records: Vec<StabilityStateConfigRecord>,
    pub aggregate: StabilityAggregate,
}

/// Builds the development state corpus by following the canonical baseline
/// trajectory (never the teacher's own choice - see this module's doc
/// comment) and, at each visited state, evaluates every
/// (scenario_count, horizon_decisions, build_tower_rollout_limit)
/// combination in `grid` without mutating the environment. Only after every
/// config has been evaluated for a state does the harness advance the real
/// environment by the canonical baseline action.
pub fn run_stability_grid(
    game_config: Arc<GameConfig>,
    grid: &StabilityGridConfig,
) -> Result<StabilityReport> {
    grid.validate()?;
    let (reference_scenario_count, reference_horizon, reference_build_tower_rollout_limit) =
        grid.reference_setting();

    let mut records = Vec::new();
    for seed in grid.seed_start..=grid.seed_end {
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
        let mut decision_index = 0usize;
        let mut states_recorded = 0usize;
        while states_recorded < grid.state_limit_per_seed && decision_index < grid.max_decisions {
            if matches!(environment.decision_point(), DecisionPoint::Terminal) {
                break;
            }
            let state_hash = environment.state_hash();
            let decision_point = format!("{:?}", environment.decision_point());
            // Dense `BuildTower` ranking and the canonical baseline action
            // depend only on this (unmutated) state, never on
            // scenario_count/horizon_decisions/build_tower_rollout_limit -
            // prepared once per state and reused (sliced per
            // build_tower_rollout_limit) across every config combination
            // below, instead of recomputing a full
            // `DenseBuildTowerScoreTable` per combination.
            let prepared =
                prepare_semantic_candidates(&environment, reference_build_tower_rollout_limit)?;
            for &scenario_count in &grid.scenario_counts {
                let scenario_seeds = (grid.scenario_seed_start
                    ..grid.scenario_seed_start.saturating_add(scenario_count as u64))
                    .collect::<Vec<_>>();
                for &horizon_decisions in &grid.horizon_decisions {
                    for &build_tower_rollout_limit in &grid.build_tower_rollout_limits {
                        let config = RolloutTeacherConfig {
                            scenario_seeds: scenario_seeds.clone(),
                            horizon_decisions,
                            build_tower_rollout_limit,
                        };
                        let candidates = prepared.candidates_for_limit(build_tower_rollout_limit);
                        let start = Instant::now();
                        let decision = evaluate_semantic_candidate_set_with_baseline(
                            &environment,
                            &candidates,
                            prepared.baseline_action.clone(),
                            &config,
                        )?;
                        let elapsed_seconds = start.elapsed().as_secs_f64();
                        let mut sorted_scores = decision
                            .candidates
                            .iter()
                            .map(|candidate| candidate.mean_score)
                            .collect::<Vec<_>>();
                        sorted_scores.sort_by(|a, b| b.total_cmp(a));
                        let score_margin = if sorted_scores.len() >= 2 {
                            Some(sorted_scores[0] - sorted_scores[1])
                        } else {
                            None
                        };
                        let selected_standard_error = decision
                            .candidates
                            .iter()
                            .find(|candidate| candidate.action_id == decision.selected_action_id)
                            .map(|candidate| candidate.standard_error)
                            .unwrap_or(0.0);
                        let baseline_standard_error = decision
                            .candidates
                            .iter()
                            .find(|candidate| candidate.action_id == decision.baseline_action_id)
                            .map(|candidate| candidate.standard_error)
                            .unwrap_or(0.0);
                        records.push(StabilityStateConfigRecord {
                            seed,
                            decision_index,
                            state_hash: state_hash.clone(),
                            decision_point: decision_point.clone(),
                            scenario_count,
                            horizon_decisions,
                            build_tower_rollout_limit,
                            candidate_count: decision.candidate_count,
                            selected_action_id: decision.selected_action_id,
                            baseline_action_id: decision.baseline_action_id,
                            selected_mean_score: decision.selected_mean_score,
                            baseline_mean_score: decision.baseline_mean_score,
                            expert_regret: decision.expert_regret,
                            selected_standard_error,
                            baseline_standard_error,
                            score_margin,
                            elapsed_seconds,
                        });
                    }
                }
            }
            states_recorded += 1;
            decision_index += 1;
            let action = canonical_scripted_semantic_action(&environment)?;
            let mut outcome = environment
                .semantic_step(action)
                .map_err(|error| anyhow::anyhow!("stability corpus step failed: {error:?}"))?;
            settle_forced_actions(&mut environment, &mut outcome)?;
            if outcome.terminated || outcome.truncated {
                break;
            }
        }
    }

    let aggregate = aggregate_stability_records(
        &records,
        reference_scenario_count,
        reference_horizon,
        reference_build_tower_rollout_limit,
    );

    Ok(StabilityReport {
        grid: grid.clone(),
        large_regret_threshold: LARGE_REGRET_THRESHOLD,
        reference_scenario_count,
        reference_horizon_decisions: reference_horizon,
        reference_build_tower_rollout_limit,
        records,
        aggregate,
    })
}

fn aggregate_stability_records(
    records: &[StabilityStateConfigRecord],
    reference_scenario_count: usize,
    reference_horizon: usize,
    reference_build_tower_rollout_limit: Option<usize>,
) -> StabilityAggregate {
    let sample_count = records.len();
    if sample_count == 0 {
        return StabilityAggregate {
            sample_count: 0,
            agreement_rate_vs_reference: 0.0,
            agreement_rate_by_decision_point: std::collections::BTreeMap::new(),
            mean_expert_regret: 0.0,
            median_expert_regret: 0.0,
            fraction_positive_regret: 0.0,
            fraction_large_regret: 0.0,
            mean_candidate_count: 0.0,
            mean_wall_time_seconds: 0.0,
            scenario_rollout_count: 0,
        };
    }
    // A reference selected_action_id per (seed, decision_index), from the
    // reference setting's record at that state.
    let mut reference_selection: std::collections::HashMap<(u64, usize), &str> =
        std::collections::HashMap::new();
    for record in records {
        if record.scenario_count == reference_scenario_count
            && record.horizon_decisions == reference_horizon
            && record.build_tower_rollout_limit == reference_build_tower_rollout_limit
        {
            reference_selection.insert(
                (record.seed, record.decision_index),
                record.selected_action_id.as_str(),
            );
        }
    }
    let mut agreements = 0usize;
    let mut agreements_by_point: std::collections::BTreeMap<String, (usize, usize)> =
        std::collections::BTreeMap::new();
    for record in records {
        if let Some(&reference_action_id) = reference_selection.get(&(record.seed, record.decision_index)) {
            let agrees = record.selected_action_id == reference_action_id;
            agreements += agrees as usize;
            let entry = agreements_by_point
                .entry(record.decision_point.clone())
                .or_insert((0, 0));
            entry.0 += agrees as usize;
            entry.1 += 1;
        }
    }
    let agreement_rate_vs_reference = agreements as f64 / sample_count as f64;
    let agreement_rate_by_decision_point = agreements_by_point
        .into_iter()
        .map(|(point, (agree, total))| (point, agree as f64 / total.max(1) as f64))
        .collect();

    let mut regrets = records.iter().map(|r| r.expert_regret as f64).collect::<Vec<_>>();
    regrets.sort_by(|a, b| a.total_cmp(b));
    let mean_expert_regret = regrets.iter().sum::<f64>() / sample_count as f64;
    let median_expert_regret = if sample_count % 2 == 1 {
        regrets[sample_count / 2]
    } else {
        (regrets[sample_count / 2 - 1] + regrets[sample_count / 2]) / 2.0
    };
    let fraction_positive_regret =
        records.iter().filter(|r| r.expert_regret > 0.0).count() as f64 / sample_count as f64;
    let fraction_large_regret = records
        .iter()
        .filter(|r| r.expert_regret >= LARGE_REGRET_THRESHOLD)
        .count() as f64
        / sample_count as f64;
    let mean_candidate_count =
        records.iter().map(|r| r.candidate_count as f64).sum::<f64>() / sample_count as f64;
    let mean_wall_time_seconds =
        records.iter().map(|r| r.elapsed_seconds).sum::<f64>() / sample_count as f64;
    let scenario_rollout_count = records
        .iter()
        .map(|r| (r.candidate_count as u64) * (r.scenario_count as u64))
        .sum::<u64>();

    StabilityAggregate {
        sample_count,
        agreement_rate_vs_reference,
        agreement_rate_by_decision_point,
        mean_expert_regret,
        median_expert_regret,
        fraction_positive_regret,
        fraction_large_regret,
        mean_candidate_count,
        mean_wall_time_seconds,
        scenario_rollout_count,
    }
}

// --- Paired full-game evaluation foundation (item 13) -------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedFullGameSeedResult {
    pub seed: u64,
    pub baseline_victory: bool,
    pub teacher_victory: bool,
    pub baseline_clear_rate: f32,
    pub teacher_clear_rate: f32,
    pub baseline_final_stage: usize,
    pub teacher_final_stage: usize,
    pub baseline_decision_count: usize,
    pub teacher_decision_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedFullGameAggregate {
    pub seed_count: usize,
    pub baseline_full_clear_rate: f64,
    pub teacher_full_clear_rate: f64,
    pub teacher_win_baseline_loss_count: usize,
    pub baseline_win_teacher_loss_count: usize,
    pub mean_clear_rate_delta: f64,
    pub mean_final_stage_delta: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedFullGameReport {
    pub seeds: Vec<u64>,
    pub max_decisions: usize,
    pub teacher_config: RolloutTeacherConfig,
    pub results: Vec<PairedFullGameSeedResult>,
    pub aggregate: PairedFullGameAggregate,
}

/// Paired baseline (`canonical_scripted_semantic_action`) vs. rollout
/// teacher (`run_semantic_teacher_episode` with `teacher_config`) full-game
/// comparison, on an identical gameplay seed list for both sides. This is a
/// foundation for the real held-out strength gate, not the gate itself -
/// callers must not treat a smoke-sized `seeds` list as approving the
/// teacher.
pub fn run_paired_full_game_evaluation(
    game_config: Arc<GameConfig>,
    seeds: &[u64],
    max_decisions: usize,
    teacher_config: &RolloutTeacherConfig,
) -> Result<PairedFullGameReport> {
    if max_decisions == 0 {
        bail!("paired full-game max decisions must be positive");
    }
    let results = seeds
        .par_iter()
        .map(|&seed| -> Result<PairedFullGameSeedResult> {
            let baseline = run_canonical_scripted_semantic_episode(
                Arc::clone(&game_config),
                seed,
                max_decisions,
            )?;
            let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
            let teacher_episode =
                run_semantic_teacher_episode(&mut environment, teacher_config, max_decisions)?;
            Ok(PairedFullGameSeedResult {
                seed,
                baseline_victory: baseline.victory,
                teacher_victory: teacher_episode.victory,
                baseline_clear_rate: baseline.clear_rate,
                teacher_clear_rate: environment.clear_rate(),
                baseline_final_stage: baseline.final_stage,
                teacher_final_stage: environment.snapshot().stage,
                baseline_decision_count: baseline.decision_count,
                teacher_decision_count: teacher_episode.decision_count,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let seed_count = results.len();
    let divisor = seed_count.max(1);
    let baseline_full_clear_rate =
        results.iter().filter(|r| r.baseline_victory).count() as f64 / divisor as f64;
    let teacher_full_clear_rate =
        results.iter().filter(|r| r.teacher_victory).count() as f64 / divisor as f64;
    let teacher_win_baseline_loss_count = results
        .iter()
        .filter(|r| r.teacher_victory && !r.baseline_victory)
        .count();
    let baseline_win_teacher_loss_count = results
        .iter()
        .filter(|r| r.baseline_victory && !r.teacher_victory)
        .count();
    let mean_clear_rate_delta = results
        .iter()
        .map(|r| (r.teacher_clear_rate - r.baseline_clear_rate) as f64)
        .sum::<f64>()
        / divisor as f64;
    let mean_final_stage_delta = results
        .iter()
        .map(|r| r.teacher_final_stage as f64 - r.baseline_final_stage as f64)
        .sum::<f64>()
        / divisor as f64;

    Ok(PairedFullGameReport {
        seeds: seeds.to_vec(),
        max_decisions,
        teacher_config: teacher_config.clone(),
        results,
        aggregate: PairedFullGameAggregate {
            seed_count,
            baseline_full_clear_rate,
            teacher_full_clear_rate,
            teacher_win_baseline_loss_count,
            baseline_win_teacher_loss_count,
            mean_clear_rate_delta,
            mean_final_stage_delta,
        },
    })
}

pub fn teacher_score_schema_version() -> u32 {
    TEACHER_SCORE_SCHEMA_VERSION
}

pub fn scenario_schedule_digest(scenario_seed_start: u64, scenario_count: usize) -> String {
    let seeds = (scenario_seed_start..scenario_seed_start.saturating_add(scenario_count as u64))
        .collect::<Vec<_>>();
    scenario_seed_digest(&seeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Arc<GameConfig> {
        Arc::new(GameConfig::default_config())
    }

    #[test]
    fn canonical_episode_is_deterministic() {
        let first = run_canonical_scripted_semantic_episode(config(), 3, 16)
            .expect("canonical episode should run");
        let second = run_canonical_scripted_semantic_episode(config(), 3, 16)
            .expect("canonical episode should repeat");
        assert_eq!(first, second);
    }

    /// Increasing scenario count must preserve the nested seed prefix: the
    /// N-scenario schedule is exactly the first N seeds of the 2N-scenario
    /// schedule (common-random-number nesting), not an independently drawn
    /// set.
    #[test]
    fn scenario_count_seed_schedule_is_a_nested_prefix() {
        let small = (1000u64..1000 + 2).collect::<Vec<_>>();
        let large = (1000u64..1000 + 4).collect::<Vec<_>>();
        assert_eq!(large[..small.len()], small[..]);
        assert_ne!(
            scenario_schedule_digest(1000, 2),
            scenario_schedule_digest(1000, 4),
            "different scenario counts must produce different digests"
        );
    }

    /// `run_stability_grid` now shares one `DenseBuildTowerScoreTable`
    /// (`prepare_semantic_candidates`) across every scenario/horizon/limit
    /// combination at a given state, but each candidate's rollout scenarios
    /// still run real fixed-horizon simulation - a full stability grid over
    /// a meaningful seed/state range is still a release-mode
    /// `teacher-eval` CLI run, not a unit test. Kept `#[ignore]`, matching
    /// this crate's existing convention for heavy diagnostics (see
    /// `teacher::tests::dense_candidate_migration_benchmark`) - see
    /// docs/game-ai/05-rollout-teacher.md's "Stability evaluation harness".
    #[test]
    #[ignore = "real fixed-horizon rollout simulation per candidate; run in release mode: cargo test --release -- --ignored stability_grid_is_deterministic_and_covers_every_config"]
    fn stability_grid_is_deterministic_and_covers_every_config() {
        let grid = StabilityGridConfig {
            seed_start: 0,
            seed_end: 0,
            max_decisions: 2,
            state_limit_per_seed: 1,
            scenario_seed_start: 1000,
            scenario_counts: vec![1, 2],
            horizon_decisions: vec![1],
            build_tower_rollout_limits: vec![Some(2), Some(4)],
        };
        let first = run_stability_grid(config(), &grid).expect("grid should run");
        let second = run_stability_grid(config(), &grid).expect("grid should repeat");
        assert_eq!(first, second);
        // Every state should produce exactly one record per grid combination.
        let combos = grid.scenario_counts.len()
            * grid.horizon_decisions.len()
            * grid.build_tower_rollout_limits.len();
        assert_eq!(first.records.len() % combos, 0);
        // State hash for a given (seed, decision_index) must be identical
        // across every config - the corpus is fixed by the canonical
        // baseline trajectory, not by which teacher config is evaluated.
        let mut hashes: std::collections::HashMap<(u64, usize), std::collections::HashSet<String>> =
            std::collections::HashMap::new();
        for record in &first.records {
            hashes
                .entry((record.seed, record.decision_index))
                .or_default()
                .insert(record.state_hash.clone());
        }
        for (_, set) in hashes {
            assert_eq!(set.len(), 1, "state hash must be config-independent");
        }
        for record in &first.records {
            assert!(!record.baseline_action_id.is_empty());
        }
    }

    /// A rollout-teacher episode's decisions vary widely in cost - some
    /// early-game states have dozens of legal `Reroll` card-subset actions
    /// (independent of `build_tower_rollout_limit`, which only bounds the
    /// `BuildTower` portion), each requiring its own fixed-horizon rollout -
    /// so a handful of real teacher decisions can cost tens of seconds even
    /// in an optimized debug build. Manual/release-mode check only, matching
    /// this crate's `#[ignore]` convention for heavy diagnostics (see
    /// `stability_grid_is_deterministic_and_covers_every_config`); smoke-run
    /// via the `teacher-eval --run-paired-full-game` CLI instead.
    #[test]
    #[ignore = "real rollout-teacher episode decisions can each cost tens of seconds in debug; run in release mode: cargo test --release -- --ignored paired_full_game_uses_identical_seed_lists"]
    fn paired_full_game_uses_identical_seed_lists() {
        let seeds = vec![0u64, 1];
        let teacher_config = RolloutTeacherConfig {
            scenario_seeds: vec![2000, 2001],
            horizon_decisions: 2,
            build_tower_rollout_limit: Some(4),
        };
        let report = run_paired_full_game_evaluation(config(), &seeds, 8, &teacher_config)
            .expect("paired evaluation should run");
        assert_eq!(report.seeds, seeds);
        assert_eq!(
            report.results.iter().map(|r| r.seed).collect::<Vec<_>>(),
            seeds
        );
    }
}
