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
