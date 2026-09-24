use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Result, bail};
use rayon::prelude::*;
use serde::Serialize;
use td_core::diagnostics::{self, Counters};

use crate::config::GameConfig;
use crate::environment::{AgentAction, DecisionPoint, GameEnvironment};
use crate::policy_runner::canonical_scripted_semantic_action;
use crate::teacher::settle_forced_actions;
use crate::teacher_selection::{TeacherSelectionPools, build_s41_proposal};

const MAX_CONTINUATION_DECISIONS: usize = 512;

#[derive(Debug, Serialize)]
pub struct PrefixDecision {
    pub decision_index: usize,
    pub decision_point: String,
    pub state_hash: String,
    pub action_id: String,
}

#[derive(Debug, Serialize)]
pub struct BranchReport {
    pub candidate_id: String,
    pub is_baseline: bool,
    pub scenario_seed: u64,
    pub decisions: usize,
    pub hit_decision_cap: bool,
    pub hit_time_limit: bool,
    pub terminal: bool,
    pub clear_rate: f32,
    pub elapsed_seconds: f64,
    pub action_kind_counts: BTreeMap<String, usize>,
    pub decision_point_counts: BTreeMap<String, usize>,
    pub build_tower_count: usize,
    pub remove_tower_count: usize,
    pub tower_count_max: usize,
    pub tower_count_mean: f64,
    pub counters: Counters,
    pub can_place_at_seconds: f64,
    pub can_place_at_share: f64,
    pub full_placement_scans_per_decision_max: u64,
    pub full_placement_scans_per_decision_mean: f64,
    pub slowest_decision_seconds: f64,
    pub slowest_decision_point: String,
}

#[derive(Debug, Serialize)]
pub struct PlacementDiagReport {
    pub game_seed: u64,
    pub prefix_decisions: usize,
    pub prefix: Vec<PrefixDecision>,
    pub target_decision_point: String,
    pub target_state_hash: String,
    pub target_tower_count: usize,
    pub baseline_action_id: String,
    pub proposal_action_ids: Vec<String>,
    pub proposal_seconds: f64,
    pub proposal_counters: Counters,
    pub branches: Vec<BranchReport>,
}

fn action_kind(action: &AgentAction) -> String {
    let id = action.action_id();
    id.split(':').next().unwrap_or(&id).to_string()
}

fn run_branch(
    source: &GameEnvironment,
    candidate: &AgentAction,
    is_baseline: bool,
    scenario_seed: u64,
    time_limit_seconds: f64,
) -> Result<BranchReport> {
    let started = Instant::now();
    let counters_before = diagnostics::snapshot();
    let mut rollout = source
        .fork_for_rollout_seed(scenario_seed)
        .map_err(|error| anyhow::anyhow!("fork failed: {error}"))?;

    let mut action_kind_counts = BTreeMap::new();
    let mut decision_point_counts = BTreeMap::new();
    let mut tower_counts = Vec::new();
    let mut scans_per_decision = Vec::new();
    let mut slowest_decision_seconds = 0.0f64;
    let mut slowest_decision_point = String::new();
    let mut decisions = 0usize;
    let mut next_action = Some(candidate.clone());
    let mut terminal = false;
    let mut hit_decision_cap = false;
    let mut hit_time_limit = false;

    loop {
        if matches!(rollout.decision_point(), DecisionPoint::Terminal) {
            terminal = true;
            break;
        }
        if decisions >= MAX_CONTINUATION_DECISIONS {
            hit_decision_cap = true;
            break;
        }
        if started.elapsed().as_secs_f64() >= time_limit_seconds {
            hit_time_limit = true;
            break;
        }
        let decision_started = Instant::now();
        let decision_counters = diagnostics::snapshot();
        let decision_point = format!("{:?}", rollout.decision_point());
        let action = match next_action.take() {
            Some(action) => action,
            None => match rollout.forced_action() {
                Some(action) => action,
                None => canonical_scripted_semantic_action(&rollout)?,
            },
        };
        let policy_seconds = decision_started.elapsed().as_secs_f64();
        let policy_counters = diagnostics::snapshot().delta(&decision_counters);
        let action_id = action.action_id();
        *action_kind_counts.entry(action_kind(&action)).or_insert(0) += 1;
        *decision_point_counts
            .entry(decision_point.clone())
            .or_insert(0) += 1;
        let outcome = rollout
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("branch step failed: {error:?}"))?;
        decisions += 1;
        tower_counts.push(rollout.tower_count());
        scans_per_decision.push(
            diagnostics::snapshot()
                .delta(&decision_counters)
                .full_placement_scans(),
        );
        let decision_seconds = decision_started.elapsed().as_secs_f64();
        if decision_seconds >= 5.0 {
            let step_counters = diagnostics::snapshot().delta(&decision_counters);
            eprintln!(
                "    slow decision seed={scenario_seed} #{decisions} [{decision_point}] action={action_id} towers={} total={decision_seconds:.1}s policy={policy_seconds:.1}s policy_counters={policy_counters:?} total_counters={step_counters:?}",
                rollout.tower_count(),
            );
        }
        if decision_seconds > slowest_decision_seconds {
            slowest_decision_seconds = decision_seconds;
            slowest_decision_point = decision_point;
        }
        if outcome.terminated || outcome.truncated {
            terminal = true;
            break;
        }
    }

    let elapsed_seconds = started.elapsed().as_secs_f64();
    let counters = diagnostics::snapshot().delta(&counters_before);
    let can_place_at_seconds = counters.can_place_at_nanos as f64 / 1e9;
    Ok(BranchReport {
        candidate_id: candidate.action_id(),
        is_baseline,
        scenario_seed,
        decisions,
        hit_decision_cap,
        hit_time_limit,
        terminal,
        clear_rate: rollout.clear_rate(),
        elapsed_seconds,
        build_tower_count: action_kind_counts.get("build_tower").copied().unwrap_or(0),
        remove_tower_count: action_kind_counts.get("remove_tower").copied().unwrap_or(0),
        action_kind_counts,
        decision_point_counts,
        tower_count_max: tower_counts.iter().copied().max().unwrap_or(0),
        tower_count_mean: if tower_counts.is_empty() {
            0.0
        } else {
            tower_counts.iter().sum::<usize>() as f64 / tower_counts.len() as f64
        },
        counters,
        can_place_at_seconds,
        can_place_at_share: if elapsed_seconds > 0.0 {
            can_place_at_seconds / elapsed_seconds
        } else {
            0.0
        },
        full_placement_scans_per_decision_max: scans_per_decision
            .iter()
            .copied()
            .max()
            .unwrap_or(0),
        full_placement_scans_per_decision_mean: if scans_per_decision.is_empty() {
            0.0
        } else {
            scans_per_decision.iter().sum::<u64>() as f64 / scans_per_decision.len() as f64
        },
        slowest_decision_seconds,
        slowest_decision_point,
    })
}

pub fn run_placement_diag(
    config: Arc<GameConfig>,
    game_seed: u64,
    prefix_decisions: usize,
    scenario_seeds: &[u64],
    baseline_only: bool,
    time_limit_seconds: f64,
) -> Result<PlacementDiagReport> {
    let mut environment = GameEnvironment::new(config, game_seed);
    let mut prefix = Vec::new();
    for decision_index in 0..prefix_decisions {
        if matches!(environment.decision_point(), DecisionPoint::Terminal) {
            bail!("episode terminated before decision {decision_index}");
        }
        let action = canonical_scripted_semantic_action(&environment)?;
        prefix.push(PrefixDecision {
            decision_index,
            decision_point: format!("{:?}", environment.decision_point()),
            state_hash: environment.state_hash(),
            action_id: action.action_id(),
        });
        let mut outcome = environment
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("prefix step failed: {error:?}"))?;
        settle_forced_actions(&mut environment, &mut outcome)?;
    }

    let baseline = canonical_scripted_semantic_action(&environment)?;
    let proposal_started = Instant::now();
    let proposal_counters_before = diagnostics::snapshot();
    let proposal = build_s41_proposal(
        &environment,
        &baseline,
        &TeacherSelectionPools::production(),
    )?;
    let proposal_counters = diagnostics::snapshot().delta(&proposal_counters_before);
    let proposal_seconds = proposal_started.elapsed().as_secs_f64();

    let mut candidates = vec![(baseline.clone(), true)];
    if !baseline_only {
        candidates.extend(
            proposal
                .iter()
                .map(|candidate| (candidate.action.clone(), false)),
        );
    }
    let jobs: Vec<(AgentAction, bool, u64)> = candidates
        .iter()
        .flat_map(|(action, is_baseline)| {
            scenario_seeds
                .iter()
                .map(move |&seed| (action.clone(), *is_baseline, seed))
        })
        .collect();
    let branches = jobs
        .par_iter()
        .map(|(action, is_baseline, seed)| {
            let report = run_branch(&environment, action, *is_baseline, *seed, time_limit_seconds)?;
            eprintln!(
                "  {} seed {}: decisions={} cap={} timeout={} {:.1}s can_place_at={:.0}% towers_max={} scans/decision max={} mean={:.1}",
                report.candidate_id,
                report.scenario_seed,
                report.decisions,
                report.hit_decision_cap,
                report.hit_time_limit,
                report.elapsed_seconds,
                report.can_place_at_share * 100.0,
                report.tower_count_max,
                report.full_placement_scans_per_decision_max,
                report.full_placement_scans_per_decision_mean,
            );
            Ok(report)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(PlacementDiagReport {
        game_seed,
        prefix_decisions,
        prefix,
        target_decision_point: format!("{:?}", environment.decision_point()),
        target_state_hash: environment.state_hash(),
        target_tower_count: environment.tower_count(),
        baseline_action_id: baseline.action_id(),
        proposal_action_ids: proposal
            .iter()
            .map(|candidate| candidate.id.clone())
            .collect(),
        proposal_seconds,
        proposal_counters,
        branches,
    })
}

#[derive(Debug, Serialize)]
pub struct FingerprintDecision {
    pub decision_index: usize,
    pub decision_point: String,
    pub state_hash: String,
    pub legal_action_ids: Vec<String>,
    pub semantic_legal_action_ids: Vec<String>,
    pub proposal_action_ids: Vec<String>,
    pub action_id: String,
    pub clear_rate: f32,
}

#[derive(Debug, Serialize)]
pub struct TrajectoryFingerprint {
    pub game_seed: u64,
    pub decisions: Vec<FingerprintDecision>,
    pub final_state_hash: String,
    pub final_clear_rate: f32,
}

pub fn trajectory_fingerprint(
    config: Arc<GameConfig>,
    game_seed: u64,
    max_decisions: usize,
) -> Result<TrajectoryFingerprint> {
    let pools = TeacherSelectionPools::production();
    let mut environment = GameEnvironment::new(config, game_seed);
    let mut decisions = Vec::new();
    while decisions.len() < max_decisions
        && !matches!(environment.decision_point(), DecisionPoint::Terminal)
    {
        let action = canonical_scripted_semantic_action(&environment)?;
        let proposal = build_s41_proposal(&environment, &action, &pools)?;
        decisions.push(FingerprintDecision {
            decision_index: decisions.len(),
            decision_point: format!("{:?}", environment.decision_point()),
            state_hash: environment.state_hash(),
            legal_action_ids: environment
                .legal_actions()
                .into_iter()
                .map(|legal| legal.id)
                .collect(),
            semantic_legal_action_ids: environment
                .semantic_legal_actions()
                .into_iter()
                .map(|legal| legal.id)
                .collect(),
            proposal_action_ids: proposal.into_iter().map(|legal| legal.id).collect(),
            action_id: action.action_id(),
            clear_rate: environment.clear_rate(),
        });
        let mut outcome = environment
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("fingerprint step failed: {error:?}"))?;
        settle_forced_actions(&mut environment, &mut outcome)?;
        if outcome.terminated || outcome.truncated {
            break;
        }
    }
    Ok(TrajectoryFingerprint {
        game_seed,
        decisions,
        final_state_hash: environment.state_hash(),
        final_clear_rate: environment.clear_rate(),
    })
}
