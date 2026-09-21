//! Diagnostic-only: one-step intervention evaluation of teacher overrides.
//!
//! For each recorded stability-artifact state where the reference teacher
//! selected an action of a given kind over the canonical baseline, applies
//! either the baseline action or the teacher action once and then continues
//! both branches with `canonical_scripted_semantic_action` to terminal.
//! No teacher call, no tick deadline. Production teacher semantics are
//! untouched.

use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use serde::Serialize;

use crate::config::GameConfig;
use crate::environment::{AgentAction, DecisionPoint, GameEnvironment};
use crate::policy_runner::canonical_scripted_semantic_action;
use crate::teacher::settle_forced_actions;

#[derive(Clone, Debug, Serialize)]
pub struct AfterFirstAction {
    pub sim_tick: u64,
    pub hp_raw: i64,
    pub gold: usize,
    pub left_dice: usize,
    pub rerolled_count: usize,
    pub hand: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct BranchSample {
    pub victory: bool,
    pub terminated: bool,
    pub truncated: bool,
    pub clear_rate: f32,
    pub final_stage: usize,
    pub decisions: usize,
    pub start_sim_tick: u64,
    pub first_defense_start_sim_tick: Option<u64>,
    pub terminal_sim_tick: u64,
    pub after_first_action: AfterFirstAction,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScenarioPair {
    pub scenario_seed: u64,
    pub baseline: BranchSample,
    pub teacher: BranchSample,
}

#[derive(Clone, Debug, Serialize)]
pub struct StateDiagnostic {
    pub game_seed: u64,
    pub decision_index: usize,
    pub decision_point: String,
    pub state_hash: String,
    pub baseline_action_id: String,
    pub teacher_action_id: String,
    pub short_horizon_baseline_mean: f64,
    pub short_horizon_teacher_mean: f64,
    pub short_horizon_regret: f64,
    pub pairs: Vec<ScenarioPair>,
}

fn run_branch(
    source: &GameEnvironment,
    first_action: &AgentAction,
    scenario_seed: u64,
    max_continuation_decisions: usize,
) -> Result<BranchSample> {
    let start_sim_tick = source.sim_tick();
    let mut rollout = source
        .fork_for_rollout_seed(scenario_seed)
        .map_err(|error| anyhow::anyhow!("fork failed: {error}"))?;
    let mut outcome = rollout
        .semantic_step(first_action.clone())
        .map_err(|error| anyhow::anyhow!("first action failed: {error:?}"))?;
    let observation = rollout.snapshot();
    let after_first_action = AfterFirstAction {
        sim_tick: observation.sim_tick,
        hp_raw: observation.hp_raw,
        gold: observation.gold,
        left_dice: observation.left_dice,
        rerolled_count: observation.rerolled_count,
        hand: serde_json::to_string(&observation.hand)?,
    };
    let mut decisions = 1usize;
    let mut first_defense_start_sim_tick = None;
    loop {
        if outcome.terminated
            || outcome.truncated
            || matches!(rollout.decision_point(), DecisionPoint::Terminal)
        {
            break;
        }
        if decisions >= max_continuation_decisions {
            bail!(
                "continuation hit the {max_continuation_decisions}-decision cap (seed {scenario_seed}, tick {})",
                rollout.sim_tick()
            );
        }
        let action = match rollout.forced_action() {
            Some(action) => action,
            None => canonical_scripted_semantic_action(&rollout)?,
        };
        let tick_before = rollout.sim_tick();
        outcome = rollout
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("continuation failed: {error:?}"))?;
        if first_defense_start_sim_tick.is_none() && rollout.sim_tick() > tick_before {
            first_defense_start_sim_tick = Some(tick_before);
        }
        decisions += 1;
    }
    let observation = rollout.snapshot();
    Ok(BranchSample {
        victory: outcome.terminated && rollout.clear_rate() >= 100.0,
        terminated: outcome.terminated,
        truncated: outcome.truncated,
        clear_rate: rollout.clear_rate(),
        final_stage: observation.stage,
        decisions,
        start_sim_tick,
        first_defense_start_sim_tick,
        terminal_sim_tick: observation.sim_tick,
        after_first_action,
    })
}

pub fn run_override_intervention(
    game_config: Arc<GameConfig>,
    artifact: &Path,
    action_kind: &str,
    scenario_seeds: &[u64],
    max_continuation_decisions: usize,
) -> Result<Vec<StateDiagnostic>> {
    let report: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(artifact).with_context(|| format!("read {}", artifact.display()))?,
    )?;
    let stability = &report["stability"];
    let (ref_scenarios, ref_horizon, ref_limit) = (
        stability["reference_scenario_count"].as_u64().context("reference_scenario_count")?,
        stability["reference_horizon_sim_ticks"].as_u64().context("reference_horizon")?,
        stability["reference_build_tower_rollout_limit"].as_u64().context("reference_limit")?,
    );
    let mut targets = stability["records"]
        .as_array()
        .context("records")?
        .iter()
        .filter(|r| {
            r["scenario_count"].as_u64() == Some(ref_scenarios)
                && r["horizon_sim_ticks"].as_u64() == Some(ref_horizon)
                && r["build_tower_rollout_limit"].as_u64() == Some(ref_limit)
                && r["selected_action_id"] != r["baseline_action_id"]
                && r["selected_action_id"]
                    .as_str()
                    .is_some_and(|id| id.split(':').next() == Some(action_kind))
        })
        .cloned()
        .collect::<Vec<_>>();
    targets.sort_by_key(|r| (r["seed"].as_u64(), r["decision_index"].as_u64()));

    let mut results = Vec::new();
    let mut seeds = targets.iter().filter_map(|r| r["seed"].as_u64()).collect::<Vec<_>>();
    seeds.dedup();
    for seed in seeds {
        let wanted = targets
            .iter()
            .filter(|r| r["seed"].as_u64() == Some(seed))
            .collect::<Vec<_>>();
        let last_index = wanted
            .iter()
            .filter_map(|r| r["decision_index"].as_u64())
            .max()
            .unwrap() as usize;
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
        for decision_index in 0..=last_index {
            if matches!(environment.decision_point(), DecisionPoint::Terminal) {
                bail!("seed {seed} reached terminal before decision {decision_index}");
            }
            if let Some(record) = wanted
                .iter()
                .find(|r| r["decision_index"].as_u64() == Some(decision_index as u64))
            {
                let state_hash = environment.state_hash();
                let expected_hash = record["state_hash"].as_str().unwrap_or_default();
                if state_hash != expected_hash {
                    bail!(
                        "state hash mismatch seed {seed} index {decision_index}: replay {state_hash} vs artifact {expected_hash}"
                    );
                }
                let baseline_action = canonical_scripted_semantic_action(&environment)?;
                let baseline_id = baseline_action.action_id();
                let artifact_baseline_id = record["baseline_action_id"].as_str().unwrap_or_default();
                if baseline_id != artifact_baseline_id {
                    bail!("baseline action mismatch seed {seed} index {decision_index}: {baseline_id} vs {artifact_baseline_id}");
                }
                let teacher_id = record["selected_action_id"].as_str().unwrap_or_default().to_string();
                let teacher_action = environment
                    .semantic_legal_actions()
                    .into_iter()
                    .find(|legal| legal.id == teacher_id)
                    .with_context(|| format!("teacher action {teacher_id} not legal"))?
                    .action;
                let pairs = scenario_seeds
                    .par_iter()
                    .map(|&scenario_seed| {
                        Ok(ScenarioPair {
                            scenario_seed,
                            baseline: run_branch(&environment, &baseline_action, scenario_seed, max_continuation_decisions)?,
                            teacher: run_branch(&environment, &teacher_action, scenario_seed, max_continuation_decisions)?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                let baseline_mean = record["baseline_mean_score"].as_f64().unwrap_or_default();
                let teacher_mean = record["selected_mean_score"].as_f64().unwrap_or_default();
                results.push(StateDiagnostic {
                    game_seed: seed,
                    decision_index,
                    decision_point: record["decision_point"].as_str().unwrap_or_default().to_string(),
                    state_hash,
                    baseline_action_id: baseline_id,
                    teacher_action_id: teacher_id,
                    short_horizon_baseline_mean: baseline_mean,
                    short_horizon_teacher_mean: teacher_mean,
                    short_horizon_regret: teacher_mean - baseline_mean,
                    pairs,
                });
            }
            let action = canonical_scripted_semantic_action(&environment)?;
            let mut outcome = environment
                .semantic_step(action)
                .map_err(|error| anyhow::anyhow!("corpus replay step failed: {error:?}"))?;
            settle_forced_actions(&mut environment, &mut outcome)?;
        }
    }
    Ok(results)
}

#[derive(Clone, Debug, Serialize)]
pub struct CandidateStat {
    pub action_id: String,
    pub mean_score: f32,
    pub standard_error: f32,
    pub variance: f32,
    pub wins: usize,
    pub mean_clear_rate: f32,
    pub mean_final_stage: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScenarioCountEvaluation {
    pub scenario_count: usize,
    pub selected_action_id: String,
    pub baseline_action_id: String,
    pub baseline_mean_score: f32,
    pub expert_regret: f32,
    pub candidate_count: usize,
    pub candidates: Vec<CandidateStat>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StateSensitivity {
    pub game_seed: u64,
    pub decision_index: usize,
    pub decision_point: String,
    pub state_hash: String,
    pub reference_selected_action_id: String,
    pub evaluations: Vec<ScenarioCountEvaluation>,
}

/// Diagnostic-only: re-evaluates the full production candidate set of the
/// artifact's reference-config states whose selected action kind is
/// `action_kind`, once per scenario count (nested seed prefixes), and keeps
/// every candidate's mean/SE.
pub fn run_state_scenario_sensitivity(
    game_config: Arc<GameConfig>,
    artifact: &Path,
    action_kind: &str,
    scenario_seed_start: u64,
    scenario_counts: &[usize],
    horizon_sim_ticks: u64,
    build_tower_rollout_limit: usize,
) -> Result<Vec<StateSensitivity>> {
    use crate::teacher::{
        RolloutTeacherConfig, evaluate_semantic_candidate_set_with_baseline, prepare_semantic_candidates,
    };
    let report: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(artifact).with_context(|| format!("read {}", artifact.display()))?,
    )?;
    let stability = &report["stability"];
    let mut targets = stability["records"]
        .as_array()
        .context("records")?
        .iter()
        .filter(|r| {
            r["selected_action_id"]
                .as_str()
                .is_some_and(|id| id.split(':').next() == Some(action_kind))
        })
        .cloned()
        .collect::<Vec<_>>();
    targets.sort_by_key(|r| (r["seed"].as_u64(), r["decision_index"].as_u64()));
    let mut results = Vec::new();
    let mut seeds = targets.iter().filter_map(|r| r["seed"].as_u64()).collect::<Vec<_>>();
    seeds.dedup();
    for seed in seeds {
        let wanted = targets
            .iter()
            .filter(|r| r["seed"].as_u64() == Some(seed))
            .collect::<Vec<_>>();
        let last_index = wanted.iter().filter_map(|r| r["decision_index"].as_u64()).max().unwrap() as usize;
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
        for decision_index in 0..=last_index {
            if matches!(environment.decision_point(), DecisionPoint::Terminal) {
                bail!("seed {seed} reached terminal before decision {decision_index}");
            }
            if let Some(record) = wanted
                .iter()
                .find(|r| r["decision_index"].as_u64() == Some(decision_index as u64))
            {
                let state_hash = environment.state_hash();
                let expected_hash = record["state_hash"].as_str().unwrap_or_default();
                if state_hash != expected_hash {
                    bail!("state hash mismatch seed {seed} index {decision_index}: replay {state_hash} vs artifact {expected_hash}");
                }
                let prepared = prepare_semantic_candidates(&environment, Some(build_tower_rollout_limit))?;
                let candidates = prepared.candidates_for_limit(Some(build_tower_rollout_limit));
                let mut evaluations = Vec::new();
                for &scenario_count in scenario_counts {
                    let config = RolloutTeacherConfig {
                        scenario_seeds: (scenario_seed_start..scenario_seed_start + scenario_count as u64).collect(),
                        horizon_sim_ticks,
                        build_tower_rollout_limit: Some(build_tower_rollout_limit),
                    };
                    let decision = evaluate_semantic_candidate_set_with_baseline(
                        &environment,
                        &candidates,
                        prepared.baseline_action.clone(),
                        &config,
                    )?;
                    evaluations.push(ScenarioCountEvaluation {
                        scenario_count,
                        selected_action_id: decision.selected_action_id.clone(),
                        baseline_action_id: decision.baseline_action_id.clone(),
                        baseline_mean_score: decision.baseline_mean_score,
                        expert_regret: decision.expert_regret,
                        candidate_count: decision.candidate_count,
                        candidates: decision
                            .candidates
                            .iter()
                            .map(|c| CandidateStat {
                                action_id: c.action_id.clone(),
                                mean_score: c.mean_score,
                                standard_error: c.standard_error,
                                variance: c.variance,
                                wins: c.wins,
                                mean_clear_rate: c.mean_clear_rate,
                                mean_final_stage: c.mean_final_stage,
                            })
                            .collect(),
                    });
                }
                results.push(StateSensitivity {
                    game_seed: seed,
                    decision_index,
                    decision_point: record["decision_point"].as_str().unwrap_or_default().to_string(),
                    state_hash,
                    reference_selected_action_id: record["selected_action_id"].as_str().unwrap_or_default().to_string(),
                    evaluations,
                });
            }
            let action = canonical_scripted_semantic_action(&environment)?;
            let mut outcome = environment
                .semantic_step(action)
                .map_err(|error| anyhow::anyhow!("corpus replay step failed: {error:?}"))?;
            settle_forced_actions(&mut environment, &mut outcome)?;
        }
    }
    Ok(results)
}

#[derive(Clone, Debug, Serialize)]
pub struct ValidationScenario {
    pub scenario_seed: u64,
    pub baseline_score: f32,
    pub selected_score: f32,
    pub baseline_full: BranchSample,
    pub selected_full: BranchSample,
}

#[derive(Clone, Debug, Serialize)]
pub struct SelectionValidation {
    pub game_seed: u64,
    pub decision_index: usize,
    pub decision_point: String,
    pub state_hash: String,
    pub baseline_action_id: String,
    pub selection_selected_action_id: String,
    pub selection_selected_mean: f32,
    pub selection_selected_se: f32,
    pub selection_baseline_mean: f32,
    pub validation: Vec<ValidationScenario>,
}

/// Diagnostic-only: selects with `selection_seeds` over the full production
/// candidate set (baseline-conservative), freezes the winner, then compares
/// only {baseline, winner} on independent `validation_seeds` (CRN pairs) for
/// both the short-horizon teacher score and a one-step full-game
/// intervention. `expected_selected` (from a prior artifact) must match.
#[allow(clippy::too_many_arguments)]
pub fn run_selection_validation(
    game_config: Arc<GameConfig>,
    artifact: &Path,
    expected_artifact: &Path,
    action_kind: &str,
    selection_seeds: &[u64],
    validation_seeds: &[u64],
    horizon_sim_ticks: u64,
    build_tower_rollout_limit: usize,
    max_continuation_decisions: usize,
) -> Result<Vec<SelectionValidation>> {
    use crate::environment::LegalAction;
    use crate::teacher::{
        RolloutTeacherConfig, candidate_scenario_score, evaluate_semantic_candidate_set_with_baseline,
        prepare_semantic_candidates,
    };
    let report: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(artifact)?)?;
    let expected: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(expected_artifact)?)?;
    let mut targets = report["stability"]["records"]
        .as_array()
        .context("records")?
        .iter()
        .filter(|r| {
            r["selected_action_id"]
                .as_str()
                .is_some_and(|id| id.split(':').next() == Some(action_kind))
        })
        .cloned()
        .collect::<Vec<_>>();
    targets.sort_by_key(|r| (r["seed"].as_u64(), r["decision_index"].as_u64()));
    let mut results = Vec::new();
    let mut seeds = targets.iter().filter_map(|r| r["seed"].as_u64()).collect::<Vec<_>>();
    seeds.dedup();
    for seed in seeds {
        let wanted = targets.iter().filter(|r| r["seed"].as_u64() == Some(seed)).collect::<Vec<_>>();
        let last_index = wanted.iter().filter_map(|r| r["decision_index"].as_u64()).max().unwrap() as usize;
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
        for decision_index in 0..=last_index {
            if let Some(record) = wanted.iter().find(|r| r["decision_index"].as_u64() == Some(decision_index as u64)) {
                let state_hash = environment.state_hash();
                if state_hash != record["state_hash"].as_str().unwrap_or_default() {
                    bail!("state hash mismatch seed {seed} index {decision_index}");
                }
                let prepared = prepare_semantic_candidates(&environment, Some(build_tower_rollout_limit))?;
                let candidates = prepared.candidates_for_limit(Some(build_tower_rollout_limit));
                let selection_config = RolloutTeacherConfig {
                    scenario_seeds: selection_seeds.to_vec(),
                    horizon_sim_ticks,
                    build_tower_rollout_limit: Some(build_tower_rollout_limit),
                };
                let decision = evaluate_semantic_candidate_set_with_baseline(
                    &environment,
                    &candidates,
                    prepared.baseline_action.clone(),
                    &selection_config,
                )?;
                let expected_id = expected
                    .as_array()
                    .context("expected artifact")?
                    .iter()
                    .find(|s| s["game_seed"].as_u64() == Some(seed) && s["decision_index"].as_u64() == Some(decision_index as u64))
                    .and_then(|s| {
                        s["evaluations"].as_array()?.iter().find(|e| e["scenario_count"].as_u64() == Some(selection_seeds.len() as u64))
                    })
                    .and_then(|e| e["selected_action_id"].as_str().map(str::to_string))
                    .context("expected selection missing")?;
                if decision.selected_action_id != expected_id {
                    bail!(
                        "selection not reproduced seed {seed} index {decision_index}: {} vs {expected_id}",
                        decision.selected_action_id
                    );
                }
                let selected_estimate = decision
                    .candidates
                    .iter()
                    .find(|c| c.action_id == decision.selected_action_id)
                    .context("selected candidate")?;
                let baseline_id = decision.baseline_action_id.clone();
                let selected_legal = LegalAction {
                    id: selected_estimate.action_id.clone(),
                    action: selected_estimate.action.clone(),
                };
                let baseline_legal = LegalAction {
                    id: baseline_id.clone(),
                    action: prepared.baseline_action.clone(),
                };
                let validation_config = RolloutTeacherConfig {
                    scenario_seeds: validation_seeds.to_vec(),
                    horizon_sim_ticks,
                    build_tower_rollout_limit: Some(build_tower_rollout_limit),
                };
                let validation = validation_seeds
                    .par_iter()
                    .map(|&scenario_seed| {
                        Ok(ValidationScenario {
                            scenario_seed,
                            baseline_score: candidate_scenario_score(&environment, &baseline_legal, scenario_seed, &validation_config)?,
                            selected_score: candidate_scenario_score(&environment, &selected_legal, scenario_seed, &validation_config)?,
                            baseline_full: run_branch(&environment, &baseline_legal.action, scenario_seed, max_continuation_decisions)?,
                            selected_full: run_branch(&environment, &selected_legal.action, scenario_seed, max_continuation_decisions)?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                results.push(SelectionValidation {
                    game_seed: seed,
                    decision_index,
                    decision_point: record["decision_point"].as_str().unwrap_or_default().to_string(),
                    state_hash,
                    baseline_action_id: baseline_id,
                    selection_selected_action_id: decision.selected_action_id.clone(),
                    selection_selected_mean: decision.selected_mean_score,
                    selection_selected_se: selected_estimate.standard_error,
                    selection_baseline_mean: decision.baseline_mean_score,
                    validation,
                });
            }
            let action = canonical_scripted_semantic_action(&environment)?;
            let mut outcome = environment
                .semantic_step(action)
                .map_err(|error| anyhow::anyhow!("corpus replay step failed: {error:?}"))?;
            settle_forced_actions(&mut environment, &mut outcome)?;
        }
    }
    Ok(results)
}

#[derive(Clone, Debug, Serialize)]
pub struct HorizonScores {
    pub horizon_sim_ticks: u64,
    pub baseline_scores: Vec<f32>,
    pub selected_scores: Vec<f32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FrozenHorizonSweep {
    pub game_seed: u64,
    pub decision_index: usize,
    pub state_hash: String,
    pub baseline_action_id: String,
    pub selected_action_id: String,
    pub validation_seeds: Vec<u64>,
    pub horizons: Vec<HorizonScores>,
}

/// Diagnostic-only: re-scores a frozen {baseline, selected} action pair from
/// a prior selection-validation artifact at several exact tick horizons on
/// the same validation scenarios. Never re-selects.
pub fn run_frozen_horizon_sweep(
    game_config: Arc<GameConfig>,
    frozen_artifact: &Path,
    validation_seeds: &[u64],
    horizons: &[u64],
) -> Result<Vec<FrozenHorizonSweep>> {
    use crate::environment::LegalAction;
    use crate::teacher::{RolloutTeacherConfig, candidate_scenario_score};
    let frozen: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(frozen_artifact)?)?;
    let mut states = frozen.as_array().context("frozen artifact")?.clone();
    states.sort_by_key(|s| (s["game_seed"].as_u64(), s["decision_index"].as_u64()));
    let mut results = Vec::new();
    let mut seeds = states.iter().filter_map(|s| s["game_seed"].as_u64()).collect::<Vec<_>>();
    seeds.dedup();
    for seed in seeds {
        let wanted = states.iter().filter(|s| s["game_seed"].as_u64() == Some(seed)).collect::<Vec<_>>();
        let last_index = wanted.iter().filter_map(|s| s["decision_index"].as_u64()).max().unwrap() as usize;
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
        for decision_index in 0..=last_index {
            if let Some(state) = wanted.iter().find(|s| s["decision_index"].as_u64() == Some(decision_index as u64)) {
                let state_hash = environment.state_hash();
                if state_hash != state["state_hash"].as_str().unwrap_or_default() {
                    bail!("state hash mismatch seed {seed} index {decision_index}");
                }
                let baseline_action = canonical_scripted_semantic_action(&environment)?;
                let baseline_id = baseline_action.action_id();
                if baseline_id != state["baseline_action_id"].as_str().unwrap_or_default() {
                    bail!("baseline action mismatch seed {seed} index {decision_index}");
                }
                let selected_id = state["selection_selected_action_id"].as_str().unwrap_or_default().to_string();
                let selected_action = environment
                    .semantic_legal_actions()
                    .into_iter()
                    .find(|legal| legal.id == selected_id)
                    .with_context(|| format!("frozen action {selected_id} not legal"))?
                    .action;
                let baseline_legal = LegalAction { id: baseline_id.clone(), action: baseline_action };
                let selected_legal = LegalAction { id: selected_id.clone(), action: selected_action };
                let mut horizon_results = Vec::new();
                for &horizon in horizons {
                    let config = RolloutTeacherConfig {
                        scenario_seeds: validation_seeds.to_vec(),
                        horizon_sim_ticks: horizon,
                        build_tower_rollout_limit: None,
                    };
                    let scored = validation_seeds
                        .par_iter()
                        .map(|&scenario_seed| {
                            Ok((
                                candidate_scenario_score(&environment, &baseline_legal, scenario_seed, &config)?,
                                candidate_scenario_score(&environment, &selected_legal, scenario_seed, &config)?,
                            ))
                        })
                        .collect::<Result<Vec<_>>>()?;
                    horizon_results.push(HorizonScores {
                        horizon_sim_ticks: horizon,
                        baseline_scores: scored.iter().map(|s| s.0).collect(),
                        selected_scores: scored.iter().map(|s| s.1).collect(),
                    });
                }
                results.push(FrozenHorizonSweep {
                    game_seed: seed,
                    decision_index,
                    state_hash,
                    baseline_action_id: baseline_id,
                    selected_action_id: selected_id,
                    validation_seeds: validation_seeds.to_vec(),
                    horizons: horizon_results,
                });
            }
            let action = canonical_scripted_semantic_action(&environment)?;
            let mut outcome = environment
                .semantic_step(action)
                .map_err(|error| anyhow::anyhow!("corpus replay step failed: {error:?}"))?;
            settle_forced_actions(&mut environment, &mut outcome)?;
        }
    }
    Ok(results)
}

#[derive(Clone, Debug, Serialize)]
pub struct TerminalOutcome {
    pub clear_rate: f32,
    pub final_stage: usize,
    pub victory: bool,
    pub decisions: usize,
}

impl From<&BranchSample> for TerminalOutcome {
    fn from(sample: &BranchSample) -> Self {
        Self {
            clear_rate: sample.clear_rate,
            final_stage: sample.final_stage,
            victory: sample.victory,
            decisions: sample.decisions,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LowFidelityEntry {
    pub action_id: String,
    pub mean_score: f32,
    pub standard_error: f32,
    pub mean_minus_baseline: f32,
    pub rank_among_all: usize,
    pub is_baseline: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct TerminalSeries {
    pub action_id: String,
    pub low_fidelity_rank: Option<usize>,
    pub outcomes: Vec<TerminalOutcome>,
}

#[derive(Clone, Debug, Serialize)]
pub struct KWinner {
    pub k: usize,
    pub winner_action_id: String,
    pub winner_is_baseline: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct MultiFidelityState {
    pub game_seed: u64,
    pub decision_index: usize,
    pub decision_point: String,
    pub state_hash: String,
    pub baseline_action_id: String,
    pub frozen_phase3h_action_id: String,
    pub low_fidelity: Vec<LowFidelityEntry>,
    pub discovery_seeds: Vec<u64>,
    pub discovery: Vec<TerminalSeries>,
    pub k_winners: Vec<KWinner>,
    pub validation_seeds: Vec<u64>,
    pub validation: Vec<TerminalSeries>,
    pub low_fidelity_seconds: f64,
    pub discovery_seconds: f64,
    pub validation_seconds: f64,
    pub terminal_rollouts: usize,
}

fn mean_clear(series: &TerminalSeries) -> f32 {
    series.outcomes.iter().map(|o| o.clear_rate).sum::<f32>() / series.outcomes.len() as f32
}

/// Diagnostic-only top-K multi-fidelity check: low-fidelity (production
/// score, 8 scenarios) ranks the full candidate set; baseline + top-8 are
/// run to terminal on fresh discovery seeds; each K's winner is frozen and
/// validated on further fresh seeds. No teacher call in any continuation.
#[allow(clippy::too_many_arguments)]
pub fn run_multifidelity_topk(
    game_config: Arc<GameConfig>,
    frozen_artifact: &Path,
    states: &[(u64, usize)],
    low_seeds: &[u64],
    discovery_seeds: &[u64],
    validation_seeds: &[u64],
    ks: &[usize],
    horizon_sim_ticks: u64,
    build_tower_rollout_limit: usize,
    max_continuation_decisions: usize,
) -> Result<Vec<MultiFidelityState>> {
    use crate::teacher::{
        RolloutTeacherConfig, evaluate_semantic_candidate_set_with_baseline, prepare_semantic_candidates,
    };
    let frozen: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(frozen_artifact)?)?;
    let frozen = frozen.as_array().context("frozen artifact")?;
    let max_k = *ks.iter().max().context("ks")?;
    let mut wanted = states.to_vec();
    wanted.sort();
    let mut results = Vec::new();
    let mut seeds = wanted.iter().map(|s| s.0).collect::<Vec<_>>();
    seeds.dedup();
    for seed in seeds {
        let mine = wanted.iter().filter(|s| s.0 == seed).collect::<Vec<_>>();
        let last_index = mine.iter().map(|s| s.1).max().unwrap();
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
        for decision_index in 0..=last_index {
            if mine.iter().any(|s| s.1 == decision_index) {
                let record = frozen
                    .iter()
                    .find(|f| f["game_seed"].as_u64() == Some(seed) && f["decision_index"].as_u64() == Some(decision_index as u64))
                    .context("state missing in frozen artifact")?;
                let state_hash = environment.state_hash();
                if state_hash != record["state_hash"].as_str().unwrap_or_default() {
                    bail!("state hash mismatch seed {seed} index {decision_index}");
                }
                let baseline_action = canonical_scripted_semantic_action(&environment)?;
                let baseline_id = baseline_action.action_id();
                if baseline_id != record["baseline_action_id"].as_str().unwrap_or_default() {
                    bail!("baseline action mismatch seed {seed} index {decision_index}");
                }
                let frozen_id = record["selection_selected_action_id"].as_str().unwrap_or_default().to_string();

                let started = std::time::Instant::now();
                let prepared = prepare_semantic_candidates(&environment, Some(build_tower_rollout_limit))?;
                let candidates = prepared.candidates_for_limit(Some(build_tower_rollout_limit));
                let config = RolloutTeacherConfig {
                    scenario_seeds: low_seeds.to_vec(),
                    horizon_sim_ticks,
                    build_tower_rollout_limit: Some(build_tower_rollout_limit),
                };
                let decision = evaluate_semantic_candidate_set_with_baseline(
                    &environment,
                    &candidates,
                    prepared.baseline_action.clone(),
                    &config,
                )?;
                let low_fidelity_seconds = started.elapsed().as_secs_f64();
                let mut ranked = decision.candidates.iter().collect::<Vec<_>>();
                ranked.sort_by(|a, b| b.mean_score.total_cmp(&a.mean_score).then_with(|| a.action_id.cmp(&b.action_id)));
                let baseline_low = ranked.iter().find(|c| c.action_id == baseline_id).context("baseline estimate")?.mean_score;
                let low_fidelity = ranked
                    .iter()
                    .enumerate()
                    .map(|(index, c)| LowFidelityEntry {
                        action_id: c.action_id.clone(),
                        mean_score: c.mean_score,
                        standard_error: c.standard_error,
                        mean_minus_baseline: c.mean_score - baseline_low,
                        rank_among_all: index + 1,
                        is_baseline: c.action_id == baseline_id,
                    })
                    .collect::<Vec<_>>();
                let action_of = |id: &str| -> Result<AgentAction> {
                    Ok(decision
                        .candidates
                        .iter()
                        .find(|c| c.action_id == id)
                        .with_context(|| format!("candidate {id} missing"))?
                        .action
                        .clone())
                };
                let rank_of = |id: &str| low_fidelity.iter().find(|e| e.action_id == id).map(|e| e.rank_among_all);
                let shortlist = low_fidelity
                    .iter()
                    .filter(|e| !e.is_baseline)
                    .take(max_k)
                    .map(|e| e.action_id.clone())
                    .collect::<Vec<_>>();

                let run_series = |id: &str, seeds: &[u64]| -> Result<TerminalSeries> {
                    let action = if id == baseline_id { baseline_action.clone() } else { action_of(id)? };
                    let outcomes = seeds
                        .par_iter()
                        .map(|&s| run_branch(&environment, &action, s, max_continuation_decisions).map(|b| TerminalOutcome::from(&b)))
                        .collect::<Result<Vec<_>>>()?;
                    Ok(TerminalSeries { action_id: id.to_string(), low_fidelity_rank: rank_of(id), outcomes })
                };

                let started = std::time::Instant::now();
                let mut discovery_ids = vec![baseline_id.clone()];
                discovery_ids.extend(shortlist.iter().cloned());
                if !discovery_ids.contains(&frozen_id) {
                    discovery_ids.push(frozen_id.clone());
                }
                let discovery = discovery_ids
                    .iter()
                    .map(|id| run_series(id, discovery_seeds))
                    .collect::<Result<Vec<_>>>()?;
                let discovery_seconds = started.elapsed().as_secs_f64();
                let baseline_series = discovery.iter().find(|s| s.action_id == baseline_id).unwrap();
                let baseline_mean = mean_clear(baseline_series);
                let k_winners = ks
                    .iter()
                    .map(|&k| {
                        let mut best_id = baseline_id.clone();
                        let mut best_mean = baseline_mean;
                        for id in shortlist.iter().take(k) {
                            let series = discovery.iter().find(|s| &s.action_id == id).unwrap();
                            let mean = mean_clear(series);
                            if mean > best_mean || (mean == best_mean && best_id != baseline_id && *id < best_id) {
                                best_id = id.clone();
                                best_mean = mean;
                            }
                        }
                        KWinner { k, winner_is_baseline: best_id == baseline_id, winner_action_id: best_id }
                    })
                    .collect::<Vec<_>>();

                let started = std::time::Instant::now();
                let mut validation_ids = vec![baseline_id.clone()];
                for winner in &k_winners {
                    if !validation_ids.contains(&winner.winner_action_id) {
                        validation_ids.push(winner.winner_action_id.clone());
                    }
                }
                if !validation_ids.contains(&frozen_id) {
                    validation_ids.push(frozen_id.clone());
                }
                let validation = validation_ids
                    .iter()
                    .map(|id| run_series(id, validation_seeds))
                    .collect::<Result<Vec<_>>>()?;
                let validation_seconds = started.elapsed().as_secs_f64();
                let terminal_rollouts = discovery.len() * discovery_seeds.len() + validation.len() * validation_seeds.len();
                results.push(MultiFidelityState {
                    game_seed: seed,
                    decision_index,
                    decision_point: record["decision_point"].as_str().unwrap_or_default().to_string(),
                    state_hash,
                    baseline_action_id: baseline_id,
                    frozen_phase3h_action_id: frozen_id,
                    low_fidelity,
                    discovery_seeds: discovery_seeds.to_vec(),
                    discovery,
                    k_winners,
                    validation_seeds: validation_seeds.to_vec(),
                    validation,
                    low_fidelity_seconds,
                    discovery_seconds,
                    validation_seconds,
                    terminal_rollouts,
                });
            }
            let action = canonical_scripted_semantic_action(&environment)?;
            let mut outcome = environment
                .semantic_step(action)
                .map_err(|error| anyhow::anyhow!("corpus replay step failed: {error:?}"))?;
            settle_forced_actions(&mut environment, &mut outcome)?;
        }
    }
    Ok(results)
}

#[derive(Clone, Debug, Serialize)]
pub struct ExhaustiveTerminalResult {
    pub game_seed: u64,
    pub decision_index: usize,
    pub decision_point: String,
    pub state_hash: String,
    pub baseline_action_id: String,
    pub candidate_count: usize,
    pub low_fidelity: Vec<LowFidelityEntry>,
    pub discovery_seeds: Vec<u64>,
    pub discovery: Vec<TerminalSeries>,
    pub validation_action_ids: Vec<String>,
    pub validation_seeds: Vec<u64>,
    pub validation: Vec<TerminalSeries>,
    pub low_fidelity_seconds: f64,
    pub discovery_seconds: f64,
    pub validation_seconds: f64,
    pub discovery_rollouts: usize,
    pub validation_rollouts: usize,
}

/// Diagnostic-only: terminal oracle over the *entire* production candidate
/// set of one state on `discovery_seeds`, plus fresh validation of the
/// baseline, the discovery top-`validation_top` and the low-fidelity rank-1
/// candidate on `validation_seeds`. Nothing here feeds back into selection.
#[allow(clippy::too_many_arguments)]
pub fn run_exhaustive_terminal(
    game_config: Arc<GameConfig>,
    frozen_artifact: &Path,
    state: (u64, usize),
    low_seeds: &[u64],
    discovery_seeds: &[u64],
    validation_seeds: &[u64],
    validation_top: usize,
    horizon_sim_ticks: u64,
    build_tower_rollout_limit: usize,
    max_continuation_decisions: usize,
) -> Result<ExhaustiveTerminalResult> {
    use crate::teacher::{
        RolloutTeacherConfig, evaluate_semantic_candidate_set_with_baseline, prepare_semantic_candidates,
    };
    let frozen: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(frozen_artifact)?)?;
    let record = frozen
        .as_array()
        .context("frozen artifact")?
        .iter()
        .find(|f| f["game_seed"].as_u64() == Some(state.0) && f["decision_index"].as_u64() == Some(state.1 as u64))
        .context("state missing in frozen artifact")?
        .clone();
    let mut environment = GameEnvironment::new(Arc::clone(&game_config), state.0);
    for decision_index in 0..state.1 {
        let action = canonical_scripted_semantic_action(&environment)?;
        let mut outcome = environment
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("corpus replay step failed: {error:?}"))?;
        settle_forced_actions(&mut environment, &mut outcome)?;
        if matches!(environment.decision_point(), DecisionPoint::Terminal) {
            bail!("terminal before decision {}", decision_index + 1);
        }
    }
    let state_hash = environment.state_hash();
    if state_hash != record["state_hash"].as_str().unwrap_or_default() {
        bail!("state hash mismatch");
    }
    let baseline_action = canonical_scripted_semantic_action(&environment)?;
    let baseline_id = baseline_action.action_id();
    if baseline_id != record["baseline_action_id"].as_str().unwrap_or_default() {
        bail!("baseline action mismatch");
    }

    let started = std::time::Instant::now();
    let prepared = prepare_semantic_candidates(&environment, Some(build_tower_rollout_limit))?;
    let candidates = prepared.candidates_for_limit(Some(build_tower_rollout_limit));
    let config = RolloutTeacherConfig {
        scenario_seeds: low_seeds.to_vec(),
        horizon_sim_ticks,
        build_tower_rollout_limit: Some(build_tower_rollout_limit),
    };
    let decision = evaluate_semantic_candidate_set_with_baseline(
        &environment,
        &candidates,
        prepared.baseline_action.clone(),
        &config,
    )?;
    let low_fidelity_seconds = started.elapsed().as_secs_f64();
    let mut ranked = decision.candidates.iter().collect::<Vec<_>>();
    ranked.sort_by(|a, b| b.mean_score.total_cmp(&a.mean_score).then_with(|| a.action_id.cmp(&b.action_id)));
    let baseline_low = ranked.iter().find(|c| c.action_id == baseline_id).context("baseline estimate")?.mean_score;
    let low_fidelity = ranked
        .iter()
        .enumerate()
        .map(|(index, c)| LowFidelityEntry {
            action_id: c.action_id.clone(),
            mean_score: c.mean_score,
            standard_error: c.standard_error,
            mean_minus_baseline: c.mean_score - baseline_low,
            rank_among_all: index + 1,
            is_baseline: c.action_id == baseline_id,
        })
        .collect::<Vec<_>>();
    let rank_of = |id: &str| low_fidelity.iter().find(|e| e.action_id == id).map(|e| e.rank_among_all);
    let all = decision.candidates.iter().map(|c| (c.action_id.clone(), c.action.clone())).collect::<Vec<_>>();

    let run_grid = |actions: &[(String, AgentAction)], seeds: &[u64]| -> Result<Vec<TerminalSeries>> {
        let flat = actions
            .iter()
            .enumerate()
            .flat_map(|(a, _)| seeds.iter().map(move |&s| (a, s)))
            .collect::<Vec<_>>();
        let outcomes = flat
            .par_iter()
            .map(|&(a, s)| run_branch(&environment, &actions[a].1, s, max_continuation_decisions).map(|b| TerminalOutcome::from(&b)))
            .collect::<Result<Vec<_>>>()?;
        Ok(actions
            .iter()
            .enumerate()
            .map(|(a, (id, _))| TerminalSeries {
                action_id: id.clone(),
                low_fidelity_rank: rank_of(id),
                outcomes: outcomes[a * seeds.len()..(a + 1) * seeds.len()].to_vec(),
            })
            .collect())
    };

    let started = std::time::Instant::now();
    let discovery = run_grid(&all, discovery_seeds)?;
    let discovery_seconds = started.elapsed().as_secs_f64();
    let baseline_mean = mean_clear(discovery.iter().find(|s| s.action_id == baseline_id).unwrap());
    let _ = baseline_mean;
    let mut by_terminal = discovery.iter().filter(|s| s.action_id != baseline_id).collect::<Vec<_>>();
    by_terminal.sort_by(|a, b| mean_clear(b).total_cmp(&mean_clear(a)).then_with(|| a.action_id.cmp(&b.action_id)));
    let mut validation_ids = vec![baseline_id.clone()];
    validation_ids.extend(by_terminal.iter().take(validation_top).map(|s| s.action_id.clone()));
    let low_top = low_fidelity.iter().find(|e| !e.is_baseline).context("low-fidelity top")?.action_id.clone();
    if !validation_ids.contains(&low_top) {
        validation_ids.push(low_top);
    }
    let validation_actions = validation_ids
        .iter()
        .map(|id| Ok((id.clone(), all.iter().find(|(a, _)| a == id).context("validation action")?.1.clone())))
        .collect::<Result<Vec<_>>>()?;
    let started = std::time::Instant::now();
    let validation = run_grid(&validation_actions, validation_seeds)?;
    let validation_seconds = started.elapsed().as_secs_f64();
    Ok(ExhaustiveTerminalResult {
        game_seed: state.0,
        decision_index: state.1,
        decision_point: record["decision_point"].as_str().unwrap_or_default().to_string(),
        state_hash,
        baseline_action_id: baseline_id,
        candidate_count: all.len(),
        low_fidelity,
        discovery_seeds: discovery_seeds.to_vec(),
        discovery_rollouts: all.len() * discovery_seeds.len(),
        discovery,
        validation_rollouts: validation_actions.len() * validation_seeds.len(),
        validation_action_ids: validation_ids,
        validation_seeds: validation_seeds.to_vec(),
        validation,
        low_fidelity_seconds,
        discovery_seconds,
        validation_seconds,
    })
}

/// Diagnostic-only: production candidate order (non-build actions, then
/// dense-ranked `BuildTower` top-K) for states of a frozen artifact.
pub fn candidate_orders(
    game_config: Arc<GameConfig>,
    frozen_artifact: &Path,
    states: &[(u64, usize)],
    build_tower_rollout_limit: usize,
) -> Result<Vec<serde_json::Value>> {
    use crate::teacher::prepare_semantic_candidates;
    let frozen: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(frozen_artifact)?)?;
    let mut out = Vec::new();
    for &(seed, index) in states {
        let record = frozen
            .as_array()
            .context("frozen artifact")?
            .iter()
            .find(|f| f["game_seed"].as_u64() == Some(seed) && f["decision_index"].as_u64() == Some(index as u64))
            .context("state missing in frozen artifact")?;
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), seed);
        for _ in 0..index {
            let action = canonical_scripted_semantic_action(&environment)?;
            let mut outcome = environment
                .semantic_step(action)
                .map_err(|error| anyhow::anyhow!("corpus replay step failed: {error:?}"))?;
            settle_forced_actions(&mut environment, &mut outcome)?;
        }
        if environment.state_hash() != record["state_hash"].as_str().unwrap_or_default() {
            bail!("state hash mismatch seed {seed} index {index}");
        }
        let prepared = prepare_semantic_candidates(&environment, Some(build_tower_rollout_limit))?;
        let ids = prepared
            .candidates_for_limit(Some(build_tower_rollout_limit))
            .into_iter()
            .map(|c| c.id)
            .collect::<Vec<_>>();
        out.push(serde_json::json!({
            "game_seed": seed,
            "decision_index": index,
            "baseline_action_id": prepared.baseline_action.action_id(),
            "candidate_order": ids,
        }));
    }
    Ok(out)
}
