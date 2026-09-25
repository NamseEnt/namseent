//! Phase 3 terminal evaluation: the terminal held-out gate (baseline and
//! teacher episodes played to the actual terminal state) and the post-hoc
//! terminal extension of the decision-count-truncated held-out run.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Result, bail};
use serde::Serialize;

use crate::config::GameConfig;
use crate::environment::{AgentAction, DecisionPoint, GameEnvironment};
use crate::policy_runner::canonical_scripted_semantic_action;
use crate::teacher::settle_forced_actions;
use crate::teacher_selection::{
    TeacherSelectionDecision, TeacherSelectionEpisode, TeacherSelectionPools,
    run_teacher_selection_episode,
};

/// Runaway guard for evaluation episodes. Reaching it is an invariant
/// failure, never a result.
pub const MAX_EPISODE_DECISIONS: usize = 512;

pub const DISCOVERY_ROLLOUTS_PER_CANDIDATE: usize = 32;
pub const VALIDATION_ROLLOUTS_PER_ARM: usize = 64;

#[derive(Debug, Serialize)]
pub struct TerminalEpisodeSummary {
    pub decision_count: usize,
    pub victory: bool,
    pub clear_rate: f32,
    pub final_stage: usize,
    pub final_state_hash: String,
}

fn summarize(environment: &GameEnvironment, decision_count: usize) -> TerminalEpisodeSummary {
    TerminalEpisodeSummary {
        decision_count,
        victory: environment.clear_rate() >= 100.0,
        clear_rate: environment.clear_rate(),
        final_stage: environment.snapshot().stage,
        final_state_hash: environment.state_hash(),
    }
}

fn step_and_settle(environment: &mut GameEnvironment, action: AgentAction) -> Result<bool> {
    let mut outcome = environment
        .semantic_step(action)
        .map_err(|error| anyhow::anyhow!("evaluation step failed: {error:?}"))?;
    settle_forced_actions(environment, &mut outcome)?;
    if outcome.truncated {
        bail!("evaluation episode hit the environment tick limit - invariant failure");
    }
    Ok(outcome.terminated)
}

/// Continues `environment` with the canonical policy until the actual
/// terminal state, using the same stepping as the canonical baseline
/// episode. Errors instead of returning a result if the safety cap is hit.
fn continue_canonical_to_terminal(
    environment: &mut GameEnvironment,
    mut decision_count: usize,
) -> Result<usize> {
    while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
        if decision_count >= MAX_EPISODE_DECISIONS {
            bail!(
                "episode reached the {MAX_EPISODE_DECISIONS}-decision safety cap without \
                 terminating - invariant failure"
            );
        }
        let action = canonical_scripted_semantic_action(environment)?;
        decision_count += 1;
        if step_and_settle(environment, action)? {
            break;
        }
    }
    Ok(decision_count)
}

pub fn run_canonical_terminal_episode(
    config: Arc<GameConfig>,
    seed: u64,
) -> Result<TerminalEpisodeSummary> {
    let mut environment = GameEnvironment::new(config, seed);
    let decision_count = continue_canonical_to_terminal(&mut environment, 0)?;
    Ok(summarize(&environment, decision_count))
}

#[derive(Debug, Serialize)]
pub struct TeacherCost {
    pub terminal_rollouts: usize,
    pub discovery_rollouts: usize,
    pub validation_rollouts: usize,
    pub override_count: usize,
    pub override_kinds: BTreeMap<String, usize>,
}

fn action_kind(action_id: &str) -> String {
    action_id.split(':').next().unwrap_or(action_id).to_string()
}

pub fn teacher_cost(decisions: &[TeacherSelectionDecision]) -> TeacherCost {
    let mut cost = TeacherCost {
        terminal_rollouts: 0,
        discovery_rollouts: 0,
        validation_rollouts: 0,
        override_count: 0,
        override_kinds: BTreeMap::new(),
    };
    for decision in decisions {
        if !decision.forced {
            cost.discovery_rollouts += decision.discovery.len() * DISCOVERY_ROLLOUTS_PER_CANDIDATE;
            cost.validation_rollouts +=
                (decision.discovery_top3.len() + 1) * VALIDATION_ROLLOUTS_PER_ARM;
        }
        if decision.selected_action_id != decision.baseline_action_id {
            cost.override_count += 1;
            *cost
                .override_kinds
                .entry(action_kind(&decision.selected_action_id))
                .or_insert(0) += 1;
        }
    }
    cost.terminal_rollouts = cost.discovery_rollouts + cost.validation_rollouts;
    cost
}

#[derive(Debug, Serialize)]
pub struct TerminalGateSeed {
    pub game_seed: u64,
    pub baseline: TerminalEpisodeSummary,
    pub teacher: TeacherSelectionEpisode,
    pub teacher_cost: TeacherCost,
    pub paired_terminal_clear_rate_delta: f32,
    pub baseline_seconds: f64,
    pub teacher_seconds: f64,
}

/// One seed of the terminal gate: canonical baseline and frozen teacher, both
/// to the actual terminal state.
pub fn run_terminal_gate_seed(
    config: Arc<GameConfig>,
    game_seed: u64,
    pools: &TeacherSelectionPools,
) -> Result<TerminalGateSeed> {
    let started = Instant::now();
    let baseline = run_canonical_terminal_episode(Arc::clone(&config), game_seed)?;
    let baseline_seconds = started.elapsed().as_secs_f64();

    let started = Instant::now();
    let mut environment = GameEnvironment::new(config, game_seed);
    let teacher = run_teacher_selection_episode(&mut environment, pools, MAX_EPISODE_DECISIONS)?;
    let teacher_seconds = started.elapsed().as_secs_f64();
    if !teacher.terminated || teacher.truncated {
        bail!(
            "teacher episode for seed {game_seed} did not reach terminal within the \
             {MAX_EPISODE_DECISIONS}-decision safety cap (decisions {}, truncated {}) - \
             invariant failure",
            teacher.decision_count,
            teacher.truncated
        );
    }
    let teacher_cost = teacher_cost(&teacher.decisions);
    Ok(TerminalGateSeed {
        game_seed,
        paired_terminal_clear_rate_delta: teacher.clear_rate - baseline.clear_rate,
        baseline,
        teacher,
        teacher_cost,
        baseline_seconds,
        teacher_seconds,
    })
}

#[derive(Debug, Serialize)]
pub struct ExtendedArm {
    pub decisions_at_truncation: usize,
    pub stage_at_truncation: usize,
    pub clear_rate_at_truncation: f32,
    pub replayed_state_hash_matches: bool,
    pub terminal: TerminalEpisodeSummary,
}

#[derive(Debug, Serialize)]
pub struct TerminalExtensionSeed {
    pub game_seed: u64,
    pub baseline: ExtendedArm,
    pub teacher: ExtendedArm,
    pub delta_at_truncation: f32,
    pub terminalized_delta: f32,
}

/// Post-hoc diagnostic for one seed of the decision-count-truncated
/// held-out: replays both recorded episodes exactly (every recorded state
/// hash must match), then continues each with the canonical policy to the
/// actual terminal state. The teacher is not applied after the truncation
/// point.
pub fn extend_truncated_seed(
    config: Arc<GameConfig>,
    record: &serde_json::Value,
) -> Result<TerminalExtensionSeed> {
    let game_seed = record["game_seed"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("record missing game_seed"))?;

    let baseline_record = &record["baseline"];
    let baseline_decisions = baseline_record["decision_count"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("baseline missing decision_count"))?
        as usize;
    let mut baseline_environment = GameEnvironment::new(Arc::clone(&config), game_seed);
    for _ in 0..baseline_decisions {
        if matches!(
            baseline_environment.decision_point(),
            DecisionPoint::Terminal
        ) {
            bail!("seed {game_seed}: baseline replay terminated early");
        }
        let action = canonical_scripted_semantic_action(&baseline_environment)?;
        if step_and_settle(&mut baseline_environment, action)? {
            break;
        }
    }
    let baseline_hash_matches = baseline_environment.state_hash()
        == baseline_record["final_state_hash"]
            .as_str()
            .unwrap_or_default();
    if !baseline_hash_matches {
        bail!("seed {game_seed}: baseline replay state hash does not match the artifact");
    }
    let baseline_stage = baseline_environment.snapshot().stage;
    let baseline_clear_rate = baseline_environment.clear_rate();
    let baseline_terminal_decisions =
        continue_canonical_to_terminal(&mut baseline_environment, baseline_decisions)?;

    let teacher_record = &record["teacher"];
    let decisions: Vec<TeacherSelectionDecision> =
        serde_json::from_value(teacher_record["decisions"].clone())?;
    let mut teacher_environment = GameEnvironment::new(Arc::clone(&config), game_seed);
    for decision in &decisions {
        if teacher_environment.state_hash() != decision.state_hash {
            bail!(
                "seed {game_seed}: teacher replay diverged at decision {}",
                decision.decision_index
            );
        }
        let action = if decision.selected_action_id == decision.baseline_action_id {
            canonical_scripted_semantic_action(&teacher_environment)?
        } else {
            teacher_environment
                .semantic_legal_actions()
                .into_iter()
                .chain(teacher_environment.legal_actions())
                .find(|legal| legal.id == decision.selected_action_id)
                .map(|legal| legal.action)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "seed {game_seed}: recorded action {} is not legal on replay",
                        decision.selected_action_id
                    )
                })?
        };
        if action.action_id() != decision.selected_action_id {
            bail!("seed {game_seed}: replayed action id differs from the recorded one");
        }
        if step_and_settle(&mut teacher_environment, action)? {
            break;
        }
    }
    let teacher_hash_matches = teacher_environment.state_hash()
        == teacher_record["final_state_hash"]
            .as_str()
            .unwrap_or_default();
    if !teacher_hash_matches {
        bail!("seed {game_seed}: teacher replay state hash does not match the artifact");
    }
    let teacher_stage = teacher_environment.snapshot().stage;
    let teacher_clear_rate = teacher_environment.clear_rate();
    let teacher_terminal_decisions =
        continue_canonical_to_terminal(&mut teacher_environment, decisions.len())?;

    let baseline_terminal = summarize(&baseline_environment, baseline_terminal_decisions);
    let teacher_terminal = summarize(&teacher_environment, teacher_terminal_decisions);
    Ok(TerminalExtensionSeed {
        game_seed,
        delta_at_truncation: teacher_clear_rate - baseline_clear_rate,
        terminalized_delta: teacher_terminal.clear_rate - baseline_terminal.clear_rate,
        baseline: ExtendedArm {
            decisions_at_truncation: baseline_decisions,
            stage_at_truncation: baseline_stage,
            clear_rate_at_truncation: baseline_clear_rate,
            replayed_state_hash_matches: baseline_hash_matches,
            terminal: baseline_terminal,
        },
        teacher: ExtendedArm {
            decisions_at_truncation: decisions.len(),
            stage_at_truncation: teacher_stage,
            clear_rate_at_truncation: teacher_clear_rate,
            replayed_state_hash_matches: teacher_hash_matches,
            terminal: teacher_terminal,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::teacher_eval::run_canonical_scripted_semantic_episode;

    #[test]
    fn canonical_terminal_episode_matches_canonical_episode_runner() {
        let config = Arc::new(GameConfig::default_config());
        for seed in 0..3u64 {
            let terminal = run_canonical_terminal_episode(Arc::clone(&config), seed).unwrap();
            let reference = run_canonical_scripted_semantic_episode(
                Arc::clone(&config),
                seed,
                MAX_EPISODE_DECISIONS,
            )
            .unwrap();
            assert!(reference.terminated && !reference.truncated);
            assert_eq!(terminal.decision_count, reference.decision_count);
            assert_eq!(terminal.clear_rate, reference.clear_rate);
            assert_eq!(terminal.final_state_hash, reference.final_state_hash);
            assert_eq!(terminal.final_stage, reference.final_stage);
        }
    }

    fn decision(
        forced: bool,
        candidates: usize,
        top3: usize,
        baseline: &str,
        selected: &str,
    ) -> TeacherSelectionDecision {
        TeacherSelectionDecision {
            schema_version: 2,
            decision_index: 0,
            decision_point: String::new(),
            sim_tick: 0,
            state_hash: String::new(),
            baseline_action_id: baseline.to_string(),
            forced,
            proposal_action_ids: vec![String::new(); candidates],
            discovery: vec![(String::new(), 0.0); candidates],
            discovery_top3: vec![String::new(); top3],
            validation: Vec::new(),
            selected_action_id: selected.to_string(),
            elapsed_seconds: 0.0,
        }
    }

    #[test]
    fn teacher_cost_counts_rollouts_and_overrides() {
        let decisions = vec![
            decision(true, 0, 0, "continue", "continue"),
            decision(false, 9, 3, "purchase_shop_item:1", "reroll:4,5"),
            decision(false, 2, 2, "start_defense", "start_defense"),
        ];
        let cost = teacher_cost(&decisions);
        assert_eq!(cost.discovery_rollouts, (9 + 2) * 32);
        assert_eq!(cost.validation_rollouts, (4 + 3) * 64);
        assert_eq!(
            cost.terminal_rollouts,
            cost.discovery_rollouts + cost.validation_rollouts
        );
        assert_eq!(cost.override_count, 1);
        assert_eq!(cost.override_kinds.get("reroll"), Some(&1));
    }
}
