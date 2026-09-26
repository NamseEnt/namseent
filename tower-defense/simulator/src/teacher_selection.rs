//! Production rollout teacher selection algorithm ("S4/1 + discovery top-3 +
//! independent validation + Holm-Bonferroni gate"), frozen after the Phase
//! 3E-3N diagnostics in docs/game-ai/05-rollout-teacher.md.
//!
//! Replaces the short-horizon `stage_progress_v1`-at-fixed-tick-horizon
//! selection (`evaluate_semantic_candidate_set_with_baseline`'s
//! `select_conservatively`) as the *final* decision rule. That short score
//! is still used, unchanged, for exactly one narrower purpose here: ranking
//! `Reroll` candidates against each other during proposal construction (see
//! [`rank_best_reroll`]).
//!
//! Pipeline per decision:
//! 1. Proposal (`build_s41_proposal`): baseline (always) + every legal
//!    discrete action (not Reroll, BuildTower or PlaceTower) + the first 4
//!    `BuildTower` candidates in the existing dense-build order + the first 4
//!    `PlaceTower` candidates in the canonical placement order + the single
//!    best `Reroll` by low-fidelity `stage_progress_v1` score (8 scenarios).
//! 2. Discovery (`terminal_clear_rate` over `discovery_seeds`): every
//!    non-baseline proposal candidate is applied once, then continued with
//!    `canonical_scripted_semantic_action` only (never the teacher itself)
//!    to terminal, and ranked by mean `clear_rate`. Top 3 are frozen.
//! 3. Validation (`terminal_clear_rate` over `validation_seeds`, disjoint
//!    from both other pools): baseline and the frozen top 3 are terminal-
//!    evaluated on fresh scenarios; a one-sided paired t-test per candidate
//!    (H1: candidate > baseline) is Holm-Bonferroni corrected at family-wise
//!    alpha 0.05.
//! 4. Gate: no candidate passes -> baseline; otherwise the passing candidate
//!    with the largest validation mean paired delta (ties by action id).

use std::time::Instant;

use anyhow::{Result, bail};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::environment::{AgentAction, DecisionPoint, GameEnvironment, LegalAction};
use crate::policy_runner::{canonical_scripted_semantic_action, rank_place_tower_actions};
use crate::teacher::{
    RolloutTeacherConfig, evaluate_semantic_candidate_set_with_baseline, prepare_semantic_candidates,
    settle_forced_actions,
};

/// Schema for [`TeacherSelectionDecision`]/[`TeacherSelectionEpisode`] - a
/// wholly new production decision algorithm and output shape, distinct from
/// `TEACHER_SCORE_SCHEMA_VERSION` (which versions the `stage_progress_v1`
/// rollout-score primitive itself; that primitive's computation is
/// unchanged here, only repurposed as the Reroll proposal's ranking score -
/// see the module doc comment).
pub const TEACHER_SELECTION_SCHEMA_VERSION: u32 = 2;

/// Low-fidelity Reroll-proposal-ranking horizon (unchanged from the Phase 3
/// production default).
const PROPOSAL_HORIZON_SIM_TICKS: u64 = 3_266;
/// Safety guard identical in purpose to `teacher::MAX_TEACHER_CONTINUATION_DECISIONS`,
/// applied to discovery/validation's canonical-only terminal continuations,
/// which run to actual terminal rather than a tick horizon and so need a
/// bound on total decisions, not on stuck-tick decisions.
const MAX_TERMINAL_CONTINUATION_DECISIONS: usize = 512;
const FWER_ALPHA: f64 = 0.05;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeacherSelectionPools {
    pub proposal_seeds: Vec<u64>,
    pub discovery_seeds: Vec<u64>,
    pub validation_seeds: Vec<u64>,
}

impl TeacherSelectionPools {
    /// The three pools are disjoint teacher-internal future-sample ranges,
    /// separate from the held-out *game* seeds an episode runs under and
    /// from every earlier diagnostic's scenario ranges (see
    /// docs/game-ai/05-rollout-teacher.md).
    pub fn production() -> Self {
        Self {
            proposal_seeds: (10_000..10_008).collect(),
            discovery_seeds: (20_000..20_032).collect(),
            validation_seeds: (30_000..30_064).collect(),
        }
    }

    /// Same pipeline, a fraction of the scenario budget - for tests that
    /// check *pipeline logic* (determinism, wiring) rather than statistical
    /// power. Never used outside `#[cfg(test)]`; production always uses
    /// [`Self::production`]. Ranges are disjoint from `production()`'s and
    /// from every diagnostic scenario pool used during Phase 3 development.
    #[cfg(test)]
    fn small_for_tests() -> Self {
        Self {
            proposal_seeds: (90_000..90_002).collect(),
            discovery_seeds: (90_010..90_014).collect(),
            validation_seeds: (90_020..90_028).collect(),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.proposal_seeds.is_empty() {
            bail!("teacher selection proposal_seeds must be non-empty");
        }
        if self.discovery_seeds.is_empty() {
            bail!("teacher selection discovery_seeds must be non-empty");
        }
        if self.validation_seeds.is_empty() {
            bail!("teacher selection validation_seeds must be non-empty");
        }
        let mut all = self.proposal_seeds.clone();
        all.extend(&self.discovery_seeds);
        all.extend(&self.validation_seeds);
        let total = all.len();
        all.sort_unstable();
        all.dedup();
        if all.len() != total {
            bail!("teacher selection scenario pools must be pairwise disjoint");
        }
        Ok(())
    }
}

fn action_kind(action_id: &str) -> &str {
    action_id.split(':').next().unwrap_or("")
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CandidateValidationStat {
    pub action_id: String,
    pub mean_delta: f64,
    pub sd_delta: f64,
    pub se_delta: f64,
    pub t_statistic: f64,
    pub p_value: f64,
    pub p_holm: f64,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeacherSelectionDecision {
    pub schema_version: u32,
    #[serde(default)]
    pub decision_index: usize,
    #[serde(default)]
    pub decision_point: String,
    #[serde(default)]
    pub sim_tick: u64,
    pub state_hash: String,
    pub baseline_action_id: String,
    /// True when the S4/1 non-baseline proposal set was empty, so discovery
    /// and validation were skipped entirely and the baseline was returned
    /// directly - a cost shortcut, not a semantics change (an empty
    /// candidate set can only ever resolve to the baseline).
    pub forced: bool,
    /// The non-baseline proposal set (S4/1 minus the baseline), in
    /// construction order: non-reroll/non-build, then dense-order top-4
    /// BuildTower, then the single best Reroll.
    pub proposal_action_ids: Vec<String>,
    /// Every proposal candidate's discovery-stage mean terminal clear_rate.
    pub discovery: Vec<(String, f32)>,
    /// The frozen top 3 (by discovery mean, ties by action id), at most 3.
    pub discovery_top3: Vec<String>,
    /// Validation stats for exactly `discovery_top3`, in that order.
    pub validation: Vec<CandidateValidationStat>,
    pub selected_action_id: String,
    #[serde(default)]
    pub elapsed_seconds: f64,
}

/// Regularized incomplete beta function `I_x(a, b)`, used for the Student-t
/// CDF. Numerical-Recipes-style continued fraction (`betacf`); deterministic
/// and side-effect-free, so results reproduce bit-for-bit across runs.
fn regularized_incomplete_beta(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    fn betacf(a: f64, b: f64, x: f64) -> f64 {
        const MAXIT: usize = 200;
        const EPS: f64 = 3e-12;
        const FPMIN: f64 = 1e-300;
        let qab = a + b;
        let qap = a + 1.0;
        let qam = a - 1.0;
        let mut c = 1.0;
        let mut d = 1.0 - qab * x / qap;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        d = 1.0 / d;
        let mut h = d;
        for m in 1..=MAXIT {
            let m_f = m as f64;
            let m2 = 2.0 * m_f;
            let aa = m_f * (b - m_f) * x / ((qam + m2) * (a + m2));
            d = 1.0 + aa * d;
            if d.abs() < FPMIN {
                d = FPMIN;
            }
            c = 1.0 + aa / c;
            if c.abs() < FPMIN {
                c = FPMIN;
            }
            d = 1.0 / d;
            h *= d * c;
            let aa = -(a + m_f) * (qab + m_f) * x / ((a + m2) * (qap + m2));
            d = 1.0 + aa * d;
            if d.abs() < FPMIN {
                d = FPMIN;
            }
            c = 1.0 + aa / c;
            if c.abs() < FPMIN {
                c = FPMIN;
            }
            d = 1.0 / d;
            let delta = d * c;
            h *= delta;
            if (delta - 1.0).abs() < EPS {
                break;
            }
        }
        h
    }
    let bt = (lgamma(a + b) - lgamma(a) - lgamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

/// Lanczos approximation of `ln(Gamma(x))`, `x > 0`. Only ever called here
/// with `a = df/2 >= 0.5` and `b = 0.5`, both well inside its domain.
fn lgamma(x: f64) -> f64 {
    const G: f64 = 7.0;
    const COEFFICIENTS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        // Reflection formula; unused in practice (a, b here are >= 0.5) but
        // kept so the function is correct for any positive x.
        (std::f64::consts::PI / (std::f64::consts::PI * x).sin()).ln() - lgamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = COEFFICIENTS[0];
        let t = x + G + 0.5;
        for (i, coefficient) in COEFFICIENTS.iter().enumerate().skip(1) {
            a += coefficient / (x + i as f64);
        }
        0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

/// One-sided p-value `P(T_df >= t)` for the Student-t distribution, i.e. the
/// upper-tail probability under `H0: mean == 0` against `H1: mean > 0`.
fn one_sided_p_greater(t: f64, df: f64) -> f64 {
    let x = df / (df + t * t);
    let p_two_tail_lower = regularized_incomplete_beta(df / 2.0, 0.5, x);
    if t > 0.0 {
        p_two_tail_lower / 2.0
    } else {
        1.0 - p_two_tail_lower / 2.0
    }
}

/// Paired one-sided t-test of `H0: E[delta] <= 0` against `H1: E[delta] > 0`,
/// with an explicit, NaN-free definition for zero-variance samples (rather
/// than falling through to a `0/0` t-statistic): a nonzero constant delta is
/// treated as if perfectly separated from zero (`p = 0` or `p = 1`), and an
/// exactly-zero constant delta as a certain non-rejection (`p = 1`).
fn paired_one_sided_t_test(deltas: &[f64]) -> (f64, f64, f64, f64, f64) {
    let n = deltas.len();
    assert!(n >= 2, "paired t-test requires at least two pairs");
    let mean = deltas.iter().sum::<f64>() / n as f64;
    let variance =
        deltas.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / (n as f64 - 1.0);
    let sd = variance.max(0.0).sqrt();
    if sd == 0.0 {
        let (t, p) = if mean > 0.0 {
            (f64::INFINITY, 0.0)
        } else if mean < 0.0 {
            (f64::NEG_INFINITY, 1.0)
        } else {
            (0.0, 1.0)
        };
        return (mean, sd, 0.0, t, p);
    }
    let se = sd / (n as f64).sqrt();
    let t = mean / se;
    let p = one_sided_p_greater(t, (n - 1) as f64);
    (mean, sd, se, t, p)
}

/// Holm-Bonferroni step-down adjustment at family-wise `alpha`, returning
/// `(adjusted_p, passed)` for each input p-value in its original order.
fn holm_bonferroni(p_values: &[f64], alpha: f64) -> Vec<(f64, bool)> {
    let m = p_values.len();
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|&a, &b| {
        p_values[a]
            .partial_cmp(&p_values[b])
            .expect("p-values must never be NaN")
    });
    let mut adjusted = vec![0.0; m];
    let mut running_max = 0.0f64;
    for (rank, &index) in order.iter().enumerate() {
        let candidate = p_values[index] * (m - rank) as f64;
        running_max = running_max.max(candidate);
        adjusted[index] = running_max.min(1.0);
    }
    adjusted.into_iter().map(|p| (p, p < alpha)).collect()
}

/// Applies `action` once on a scenario-resampled fork of `source`, then
/// continues with `canonical_scripted_semantic_action` (never the teacher)
/// until terminal, and returns the terminal `clear_rate`. No tick deadline:
/// this runs to the actual end of the episode.
pub(crate) fn terminal_clear_rate(
    source: &GameEnvironment,
    action: &AgentAction,
    scenario_seed: u64,
) -> Result<f32> {
    let mut rollout = source
        .fork_for_rollout_seed(scenario_seed)
        .map_err(|error| anyhow::anyhow!("teacher selection fork failed: {error}"))?;
    let mut outcome = rollout
        .rollout_step_trusted(action.clone())
        .map_err(|error| anyhow::anyhow!("teacher selection candidate action failed: {error:?}"))?;
    let mut decisions = 1usize;
    loop {
        if outcome.terminated
            || outcome.truncated
            || matches!(rollout.decision_point(), DecisionPoint::Terminal)
        {
            break;
        }
        if decisions >= MAX_TERMINAL_CONTINUATION_DECISIONS {
            bail!(
                "terminal continuation hit the {MAX_TERMINAL_CONTINUATION_DECISIONS}-decision cap \
                 (scenario {scenario_seed}) - this is a safety-guard invariant violation, not a \
                 normal terminal outcome"
            );
        }
        let action = match rollout.forced_action() {
            Some(action) => action,
            None => canonical_scripted_semantic_action(&rollout)?,
        };
        outcome = rollout
            .rollout_step_trusted(action)
            .map_err(|error| anyhow::anyhow!("teacher selection continuation failed: {error:?}"))?;
        decisions += 1;
    }
    #[cfg(feature = "diagnostics")]
    td_core::diagnostics::record(|counters| counters.rollout_decisions += decisions as u64);
    Ok(rollout.clear_rate())
}

/// Ranks every non-`Reroll` proposal candidate, plus the dense-order top-4
/// `BuildTower` and the low-fidelity best `Reroll`, into the S4/1 set.
/// Returns `(non_reroll_non_build, build_top4, reroll_all)`, each already
/// filtered to legal, still-deduplicated production candidates.
pub const BUILD_TOWER_PROPOSAL_LIMIT: usize = 4;
pub const PLACE_TOWER_PROPOSAL_LIMIT: usize = 4;

#[derive(Default)]
struct PartitionedCandidates {
    discrete: Vec<LegalAction>,
    build_top4: Vec<LegalAction>,
    place_top4: Vec<LegalAction>,
    reroll_all: Vec<LegalAction>,
}

fn partition_candidates(environment: &GameEnvironment) -> Result<PartitionedCandidates> {
    let prepared = prepare_semantic_candidates(environment, Some(BUILD_TOWER_PROPOSAL_LIMIT))?;
    let with_build_top4 = prepared.candidates_for_limit(Some(BUILD_TOWER_PROPOSAL_LIMIT));
    let mut partitioned = PartitionedCandidates::default();
    let mut place_all = Vec::new();
    for candidate in with_build_top4 {
        match action_kind(&candidate.id) {
            "reroll" => partitioned.reroll_all.push(candidate),
            "build_tower" => partitioned.build_top4.push(candidate),
            "place_tower" => place_all.push(candidate),
            _ => partitioned.discrete.push(candidate),
        }
    }
    partitioned.place_top4 = rank_place_tower_actions(&environment.snapshot(), &place_all)
        .into_iter()
        .take(PLACE_TOWER_PROPOSAL_LIMIT)
        .collect();
    Ok(partitioned)
}

/// Ranks `reroll_candidates` by the existing low-fidelity
/// `stage_progress_v1` rollout score (`PROPOSAL_HORIZON_SIM_TICKS`,
/// `pools.proposal_seeds`) and returns the best one (ties by ascending
/// action id), or `None` if there are no Reroll candidates.
#[cfg(test)]
fn rank_best_reroll(
    environment: &GameEnvironment,
    baseline_action: &AgentAction,
    reroll_candidates: &[LegalAction],
    pools: &TeacherSelectionPools,
) -> Result<Option<LegalAction>> {
    rank_rerolls(environment, baseline_action, reroll_candidates, pools).map(|(best, _)| best)
}

/// [`rank_best_reroll`] plus every Reroll candidate's low-fidelity mean
/// score, in evaluation order.
fn rank_rerolls(
    environment: &GameEnvironment,
    baseline_action: &AgentAction,
    reroll_candidates: &[LegalAction],
    pools: &TeacherSelectionPools,
) -> Result<(Option<LegalAction>, Vec<(String, f32)>)> {
    if reroll_candidates.is_empty() {
        return Ok((None, Vec::new()));
    }
    let config = RolloutTeacherConfig {
        scenario_seeds: pools.proposal_seeds.clone(),
        horizon_sim_ticks: PROPOSAL_HORIZON_SIM_TICKS,
        build_tower_rollout_limit: None,
    };
    let decision = evaluate_semantic_candidate_set_with_baseline(
        environment,
        reroll_candidates,
        baseline_action.clone(),
        &config,
    )?;
    let baseline_id = baseline_action.action_id();
    let best = decision
        .candidates
        .iter()
        .filter(|candidate| candidate.action_id != baseline_id)
        .max_by(|a, b| {
            a.mean_score
                .total_cmp(&b.mean_score)
                .then_with(|| b.action_id.cmp(&a.action_id))
        })
        .expect("reroll_candidates is non-empty and excludes the baseline id");
    let scores = decision
        .candidates
        .iter()
        .filter(|candidate| candidate.action_id != baseline_id)
        .map(|candidate| (candidate.action_id.clone(), candidate.mean_score))
        .collect();
    Ok((
        Some(LegalAction {
            id: best.action_id.clone(),
            action: best.action.clone(),
        }),
        scores,
    ))
}

#[cfg(any(test, feature = "diagnostics"))]
pub(crate) fn build_s41_proposal(
    environment: &GameEnvironment,
    baseline_action: &AgentAction,
    pools: &TeacherSelectionPools,
) -> Result<Vec<LegalAction>> {
    build_s41_proposal_with_reroll_scores(environment, baseline_action, pools)
        .map(|(proposal, _)| proposal)
}

fn build_s41_proposal_with_reroll_scores(
    environment: &GameEnvironment,
    baseline_action: &AgentAction,
    pools: &TeacherSelectionPools,
) -> Result<(Vec<LegalAction>, Vec<(String, f32)>)> {
    let baseline_id = baseline_action.action_id();
    let partitioned = partition_candidates(environment)?;
    let (best_reroll, reroll_scores) =
        rank_rerolls(environment, baseline_action, &partitioned.reroll_all, pools)?;

    let mut proposal = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for candidate in partitioned
        .discrete
        .into_iter()
        .chain(partitioned.build_top4)
        .chain(partitioned.place_top4)
        .chain(best_reroll)
    {
        if candidate.id == baseline_id {
            continue;
        }
        if seen.insert(candidate.id.clone()) {
            proposal.push(candidate);
        }
    }
    Ok((proposal, reroll_scores))
}

/// Evaluates one decision: builds the S4/1 proposal, runs terminal discovery
/// and independent validation, and applies the Holm gate. Returns the
/// decision record plus the resolved `AgentAction` to actually apply.
pub fn select_teacher_action(
    environment: &GameEnvironment,
    pools: &TeacherSelectionPools,
) -> Result<(TeacherSelectionDecision, AgentAction)> {
    select_teacher_action_with_raw_outcomes(environment, pools)
        .map(|(decision, action, _)| (decision, action))
}

/// Raw per-scenario terminal outcomes behind one [`select_teacher_action`]
/// decision, in the order of `pools`' seed lists. Candidate vectors follow
/// `proposal_action_ids` (discovery) and `discovery_top3` (validation).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeacherRawOutcomes {
    pub reroll_proposal_scores: Vec<(String, f32)>,
    pub discovery_outcomes: Vec<Vec<f32>>,
    pub validation_baseline_outcomes: Vec<f32>,
    pub validation_candidate_outcomes: Vec<Vec<f32>>,
}

/// Same decision as [`select_teacher_action`], also returning the raw
/// scenario outcomes the decision was computed from.
pub fn select_teacher_action_with_raw_outcomes(
    environment: &GameEnvironment,
    pools: &TeacherSelectionPools,
) -> Result<(TeacherSelectionDecision, AgentAction, TeacherRawOutcomes)> {
    pools.validate()?;
    let state_hash = environment.state_hash();
    let baseline_action = canonical_scripted_semantic_action(environment)?;
    let baseline_id = baseline_action.action_id();
    let (proposal, reroll_proposal_scores) =
        build_s41_proposal_with_reroll_scores(environment, &baseline_action, pools)?;

    if proposal.is_empty() {
        return Ok((
            TeacherSelectionDecision {
                schema_version: TEACHER_SELECTION_SCHEMA_VERSION,
                decision_index: 0,
                decision_point: String::new(),
                sim_tick: environment.sim_tick(),
                state_hash,
                baseline_action_id: baseline_id.clone(),
                forced: true,
                proposal_action_ids: Vec::new(),
                discovery: Vec::new(),
                discovery_top3: Vec::new(),
                validation: Vec::new(),
                selected_action_id: baseline_id,
                elapsed_seconds: 0.0,
            },
            baseline_action,
            TeacherRawOutcomes {
                reroll_proposal_scores,
                ..TeacherRawOutcomes::default()
            },
        ));
    }

    // Discovery: every proposal candidate, once per discovery seed, ranked
    // by mean terminal clear_rate. Independent of the baseline (ranking is
    // by absolute mean, never by delta vs. baseline - see the module doc
    // comment: a candidate scoring below the baseline here must still be
    // eligible for the top-3 freeze).
    let discovery_pairs = proposal
        .iter()
        .enumerate()
        .flat_map(|(index, _)| pools.discovery_seeds.iter().map(move |&seed| (index, seed)))
        .collect::<Vec<_>>();
    let discovery_outcomes = discovery_pairs
        .par_iter()
        .map(|&(index, seed)| terminal_clear_rate(environment, &proposal[index].action, seed))
        .collect::<Result<Vec<_>>>()?;
    let discovery: Vec<(String, f32)> = proposal
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            let start = index * pools.discovery_seeds.len();
            let end = start + pools.discovery_seeds.len();
            let mean = discovery_outcomes[start..end].iter().sum::<f32>()
                / pools.discovery_seeds.len() as f32;
            (candidate.id.clone(), mean)
        })
        .collect();

    let mut ranked = discovery.clone();
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let discovery_top3: Vec<String> = ranked.into_iter().take(3).map(|(id, _)| id).collect();
    let frozen: Vec<&LegalAction> = discovery_top3
        .iter()
        .map(|id| {
            proposal
                .iter()
                .find(|candidate| &candidate.id == id)
                .expect("discovery_top3 ids come from proposal")
        })
        .collect();

    // Validation: baseline once per validation seed, reused across every
    // frozen candidate; each frozen candidate once per validation seed.
    let baseline_outcomes = pools
        .validation_seeds
        .par_iter()
        .map(|&seed| terminal_clear_rate(environment, &baseline_action, seed))
        .collect::<Result<Vec<_>>>()?;
    let candidate_pairs = frozen
        .iter()
        .enumerate()
        .flat_map(|(index, _)| pools.validation_seeds.iter().map(move |&seed| (index, seed)))
        .collect::<Vec<_>>();
    let candidate_outcomes = candidate_pairs
        .par_iter()
        .map(|&(index, seed)| terminal_clear_rate(environment, &frozen[index].action, seed))
        .collect::<Result<Vec<_>>>()?;

    let mut validation = Vec::with_capacity(frozen.len());
    for (index, candidate) in frozen.iter().enumerate() {
        let start = index * pools.validation_seeds.len();
        let end = start + pools.validation_seeds.len();
        let deltas: Vec<f64> = candidate_outcomes[start..end]
            .iter()
            .zip(&baseline_outcomes)
            .map(|(&candidate_clear, &baseline_clear)| {
                (candidate_clear - baseline_clear) as f64
            })
            .collect();
        let (mean, sd, se, t, p) = paired_one_sided_t_test(&deltas);
        validation.push(CandidateValidationStat {
            action_id: candidate.id.clone(),
            mean_delta: mean,
            sd_delta: sd,
            se_delta: se,
            t_statistic: t,
            p_value: p,
            p_holm: 0.0,
            passed: false,
        });
    }
    let p_values: Vec<f64> = validation.iter().map(|stat| stat.p_value).collect();
    for (stat, (adjusted, passed)) in validation.iter_mut().zip(holm_bonferroni(&p_values, FWER_ALPHA)) {
        stat.p_holm = adjusted;
        stat.passed = passed;
    }

    let selected = validation
        .iter()
        .filter(|stat| stat.passed)
        .max_by(|a, b| {
            a.mean_delta
                .total_cmp(&b.mean_delta)
                .then_with(|| b.action_id.cmp(&a.action_id))
        })
        .map(|stat| stat.action_id.clone());
    let selected_action_id = selected.clone().unwrap_or_else(|| baseline_id.clone());
    let selected_action = match &selected {
        Some(id) => frozen
            .iter()
            .find(|candidate| &candidate.id == id)
            .expect("selected id comes from frozen")
            .action
            .clone(),
        None => baseline_action.clone(),
    };

    Ok((
        TeacherSelectionDecision {
            schema_version: TEACHER_SELECTION_SCHEMA_VERSION,
            decision_index: 0,
            decision_point: String::new(),
            sim_tick: environment.sim_tick(),
            state_hash,
            baseline_action_id: baseline_id,
            forced: false,
            proposal_action_ids: proposal.iter().map(|candidate| candidate.id.clone()).collect(),
            discovery,
            discovery_top3,
            validation,
            selected_action_id,
            elapsed_seconds: 0.0,
        },
        selected_action,
        TeacherRawOutcomes {
            reroll_proposal_scores,
            discovery_outcomes: discovery_outcomes
                .chunks(pools.discovery_seeds.len())
                .map(<[f32]>::to_vec)
                .collect(),
            validation_baseline_outcomes: baseline_outcomes,
            validation_candidate_outcomes: candidate_outcomes
                .chunks(pools.validation_seeds.len())
                .map(<[f32]>::to_vec)
                .collect(),
        },
    ))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeacherSelectionEpisode {
    pub seed: u64,
    pub max_decisions: usize,
    pub decision_count: usize,
    pub terminated: bool,
    pub truncated: bool,
    pub victory: bool,
    pub clear_rate: f32,
    pub final_stage: usize,
    pub final_state_hash: String,
    pub decisions: Vec<TeacherSelectionDecision>,
}

/// Runs one episode driven entirely by [`select_teacher_action`]: every
/// decision re-evaluates the current real state (never the hidden future of
/// a rollout), and only the real environment advances between decisions.
pub fn run_teacher_selection_episode(
    environment: &mut GameEnvironment,
    pools: &TeacherSelectionPools,
    max_decisions: usize,
) -> Result<TeacherSelectionEpisode> {
    pools.validate()?;
    if max_decisions == 0 {
        bail!("teacher selection episode max decisions must be positive");
    }
    let mut decisions = Vec::new();
    let mut terminated = false;
    let mut truncated = false;
    while decisions.len() < max_decisions {
        if matches!(environment.decision_point(), DecisionPoint::Terminal) {
            terminated = true;
            break;
        }
        let decision_index = decisions.len();
        let decision_point = format!("{:?}", environment.decision_point());
        let started = Instant::now();
        let (mut decision, selected_action) = select_teacher_action(environment, pools)?;
        decision.decision_index = decision_index;
        decision.decision_point = decision_point;
        decision.elapsed_seconds = started.elapsed().as_secs_f64();
        eprintln!(
            "  decision {decision_index} [{}] {}: proposal={} forced={} selected={}{} ({:.1}s)",
            decision.decision_point,
            &decision.state_hash[..10.min(decision.state_hash.len())],
            decision.proposal_action_ids.len(),
            decision.forced,
            decision.selected_action_id,
            if decision.selected_action_id == decision.baseline_action_id {
                " (baseline)"
            } else {
                " (override)"
            },
            decision.elapsed_seconds
        );
        let mut outcome = environment
            .semantic_step(selected_action)
            .map_err(|error| anyhow::anyhow!("teacher selection episode action failed: {error:?}"))?;
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
    let observation = environment.snapshot();
    Ok(TeacherSelectionEpisode {
        seed: environment.seed(),
        max_decisions,
        decision_count: decisions.len(),
        terminated,
        truncated,
        victory: terminated && environment.clear_rate() >= 100.0,
        clear_rate: environment.clear_rate(),
        final_stage: observation.stage,
        final_state_hash: environment.state_hash(),
        decisions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::DecisionPoint;
    use std::collections::HashSet;
    use std::sync::Arc;

    fn config() -> Arc<GameConfig> {
        Arc::new(GameConfig::default_config())
    }

    fn shop_environment(seed: u64) -> Option<GameEnvironment> {
        let environment = GameEnvironment::new(config(), seed);
        (environment.decision_point() == DecisionPoint::Shop).then_some(environment)
    }

    fn find_shop_seed() -> (u64, GameEnvironment) {
        for seed in 0..32 {
            if let Some(environment) = shop_environment(seed) {
                return (seed, environment);
            }
        }
        panic!("expected a seed with an opening Shop decision");
    }

    fn legal_ids(environment: &GameEnvironment) -> (Vec<String>, Vec<String>) {
        (
            environment
                .legal_actions()
                .into_iter()
                .map(|legal| legal.id)
                .collect(),
            environment
                .semantic_legal_actions()
                .into_iter()
                .map(|legal| legal.id)
                .collect(),
        )
    }

    /// Steps two forks of the same scenario in lockstep, one through the
    /// public `semantic_step` and one through `rollout_step_trusted`, and
    /// requires identical state, context, legal actions, canonical choices,
    /// metrics and terminal outcome at every decision.
    #[test]
    fn trusted_rollout_step_matches_semantic_step_at_every_decision() {
        let mut decisions_checked = 0usize;
        for game_seed in [0u64, 3, 109] {
            for prefix in [0usize, 14, 33] {
                let mut source = GameEnvironment::new(config(), game_seed);
                for _ in 0..prefix {
                    let action = canonical_scripted_semantic_action(&source).unwrap();
                    let mut outcome = source.semantic_step(action).unwrap();
                    crate::teacher::settle_forced_actions(&mut source, &mut outcome).unwrap();
                }
                for scenario_seed in [20000u64, 20001] {
                    let mut reference = source.fork_for_rollout_seed(scenario_seed).unwrap();
                    let mut trusted = source.fork_for_rollout_seed(scenario_seed).unwrap();
                    loop {
                        assert_eq!(
                            reference.progress_fingerprint(),
                            trusted.progress_fingerprint(),
                            "seed {game_seed} prefix {prefix} scenario {scenario_seed}"
                        );
                        assert_eq!(legal_ids(&reference), legal_ids(&trusted));
                        assert_eq!(reference.metrics(), trusted.metrics());
                        if matches!(reference.decision_point(), DecisionPoint::Terminal) {
                            break;
                        }
                        let action = match reference.forced_action() {
                            Some(action) => action,
                            None => canonical_scripted_semantic_action(&reference).unwrap(),
                        };
                        let trusted_action = match trusted.forced_action() {
                            Some(action) => action,
                            None => canonical_scripted_semantic_action(&trusted).unwrap(),
                        };
                        assert_eq!(action, trusted_action);
                        let expected = reference.semantic_step(action).unwrap();
                        let actual = trusted.rollout_step_trusted(trusted_action).unwrap();
                        assert_eq!(expected.terminated, actual.terminated);
                        assert_eq!(expected.truncated, actual.truncated);
                        decisions_checked += 1;
                        if expected.terminated || expected.truncated {
                            break;
                        }
                    }
                    assert_eq!(reference.clear_rate(), trusted.clear_rate());
                    assert_eq!(reference.state_hash(), trusted.state_hash());
                }
            }
        }
        assert!(decisions_checked > 1000, "checked {decisions_checked}");
    }

    // --- proposal ------------------------------------------------------

    #[test]
    fn proposal_always_excludes_baseline_and_has_no_duplicates() {
        let (_, environment) = find_shop_seed();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let pools = TeacherSelectionPools::production();
        let proposal = build_s41_proposal(&environment, &baseline, &pools).unwrap();
        assert!(!proposal.iter().any(|candidate| candidate.id == baseline.action_id()));
        let unique: HashSet<&String> = proposal.iter().map(|candidate| &candidate.id).collect();
        assert_eq!(unique.len(), proposal.len());
    }

    #[test]
    fn proposal_includes_every_non_reroll_non_build_legal_action() {
        let (_, environment) = find_shop_seed();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let discrete = partition_candidates(&environment).unwrap().discrete;
        let pools = TeacherSelectionPools::production();
        let proposal = build_s41_proposal(&environment, &baseline, &pools).unwrap();
        for candidate in &discrete {
            if candidate.id == baseline.action_id() {
                continue;
            }
            assert!(
                proposal.iter().any(|p| p.id == candidate.id),
                "missing non-reroll/non-build candidate {}",
                candidate.id
            );
        }
    }

    #[test]
    fn proposal_build_tower_is_exactly_dense_order_top_4() {
        let (_, environment) = find_shop_seed();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let build_top4 = partition_candidates(&environment).unwrap().build_top4;
        assert_eq!(build_top4.len(), 4, "fixture must offer >=4 BuildTower candidates");
        let pools = TeacherSelectionPools::production();
        let proposal = build_s41_proposal(&environment, &baseline, &pools).unwrap();
        let proposal_builds: Vec<&String> = proposal
            .iter()
            .filter(|candidate| action_kind(&candidate.id) == "build_tower")
            .map(|candidate| &candidate.id)
            .collect();
        let expected: Vec<&String> = build_top4
            .iter()
            .map(|candidate| &candidate.id)
            .filter(|id| **id != baseline.action_id())
            .collect();
        assert_eq!(proposal_builds, expected);
    }

    fn tower_placement_with_hand_tower() -> GameEnvironment {
        let mut environment = GameEnvironment::new(config(), 109);
        for _ in 0..12 {
            let action = canonical_scripted_semantic_action(&environment).unwrap();
            let mut outcome = environment.semantic_step(action).unwrap();
            crate::teacher::settle_forced_actions(&mut environment, &mut outcome).unwrap();
        }
        assert_eq!(environment.decision_point(), DecisionPoint::TowerPlacement);
        environment
    }

    #[test]
    fn proposal_place_tower_is_exactly_canonical_order_top_4() {
        let environment = tower_placement_with_hand_tower();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let place_all = environment
            .semantic_legal_actions()
            .into_iter()
            .filter(|candidate| action_kind(&candidate.id) == "place_tower")
            .collect::<Vec<_>>();
        assert!(place_all.len() > PLACE_TOWER_PROPOSAL_LIMIT);
        let ranked = rank_place_tower_actions(&environment.snapshot(), &place_all);
        assert_eq!(ranked[0].id, baseline.action_id());
        let pools = TeacherSelectionPools::production();
        let proposal = build_s41_proposal(&environment, &baseline, &pools).unwrap();
        let proposal_places: Vec<&String> = proposal
            .iter()
            .filter(|candidate| action_kind(&candidate.id) == "place_tower")
            .map(|candidate| &candidate.id)
            .collect();
        let expected: Vec<&String> = ranked
            .iter()
            .take(PLACE_TOWER_PROPOSAL_LIMIT)
            .map(|candidate| &candidate.id)
            .filter(|id| **id != baseline.action_id())
            .collect();
        assert_eq!(proposal_places, expected);
    }

    #[test]
    fn proposal_reroll_is_exactly_one_best_candidate() {
        let (_, environment) = find_shop_seed();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let pools = TeacherSelectionPools::production();
        let proposal = build_s41_proposal(&environment, &baseline, &pools).unwrap();
        let rerolls = proposal
            .iter()
            .filter(|candidate| action_kind(&candidate.id) == "reroll")
            .count();
        assert_eq!(rerolls, 1);
    }

    /// Reroll candidates are scored independently per candidate (CRN, no
    /// interaction between candidates in the same rollout config), so
    /// reroll-only proposal scoring must pick exactly the same best Reroll
    /// as the full-candidate low-fidelity ranking used elsewhere in the
    /// teacher.
    #[test]
    fn reroll_only_scoring_matches_full_candidate_low_fidelity_ranking() {
        let test_started = Instant::now();
        let (_, environment) = find_shop_seed();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let pools = TeacherSelectionPools::production();
        let reroll_all = partition_candidates(&environment).unwrap().reroll_all;
        eprintln!(
            "[timing] proposal/partition phase: {:.3}s, reroll_all={}",
            test_started.elapsed().as_secs_f64(),
            reroll_all.len()
        );

        // `build_tower_rollout_limit: Some(4)` here, not `None`: per-candidate
        // scoring is independent of what else is in the evaluated set (no
        // cross-candidate interaction, CRN across scenarios), so this
        // invariant holds regardless of candidate-set size - and `None`
        // would rank every legal full-map `BuildTower` position/card-subset
        // combination (potentially hundreds to thousands of candidates),
        // making this test pathologically slow for no additional coverage.
        let full_config = RolloutTeacherConfig {
            scenario_seeds: pools.proposal_seeds.clone(),
            horizon_sim_ticks: PROPOSAL_HORIZON_SIM_TICKS,
            build_tower_rollout_limit: Some(4),
        };
        let prepared = prepare_semantic_candidates(&environment, Some(4)).unwrap();
        let full_candidates = prepared.candidates_for_limit(Some(4));
        eprintln!(
            "[timing] full_candidates size={} scenario_count={} (candidate x scenario branches = {})",
            full_candidates.len(),
            pools.proposal_seeds.len(),
            full_candidates.len() * pools.proposal_seeds.len()
        );
        let full_phase_started = Instant::now();
        let full_decision = evaluate_semantic_candidate_set_with_baseline(
            &environment,
            &full_candidates,
            baseline.clone(),
            &full_config,
        )
        .unwrap();
        eprintln!(
            "[timing] full-candidate low-fidelity ranking phase: {:.3}s",
            full_phase_started.elapsed().as_secs_f64()
        );
        let best_full_reroll = full_decision
            .candidates
            .iter()
            .filter(|candidate| action_kind(&candidate.action_id) == "reroll")
            .max_by(|a, b| {
                a.mean_score
                    .total_cmp(&b.mean_score)
                    .then_with(|| b.action_id.cmp(&a.action_id))
            })
            .unwrap();

        let reroll_phase_started = Instant::now();
        let best_reroll_only = rank_best_reroll(&environment, &baseline, &reroll_all, &pools)
            .unwrap()
            .unwrap();
        eprintln!(
            "[timing] reroll-only low-fidelity ranking phase: {:.3}s (reroll_candidates={} scenario_count={}, branches={})",
            reroll_phase_started.elapsed().as_secs_f64(),
            reroll_all.len(),
            pools.proposal_seeds.len(),
            reroll_all.len() * pools.proposal_seeds.len()
        );
        eprintln!(
            "[timing] TOTAL test elapsed: {:.3}s",
            test_started.elapsed().as_secs_f64()
        );
        let matching_estimate = full_decision
            .candidates
            .iter()
            .find(|candidate| candidate.action_id == best_reroll_only.id)
            .unwrap();
        assert_eq!(best_reroll_only.id, best_full_reroll.action_id);
        assert_eq!(matching_estimate.mean_score, best_full_reroll.mean_score);
    }

    fn tower_placement_environment() -> GameEnvironment {
        // TowerPlacement never offers a semantic Reroll or BuildTower macro
        // action (those only exist at Shop/CardSelection), so it exercises
        // both the "fewer than 4 builds" and "no reroll" degenerate cases.
        let mut environment = GameEnvironment::new(config(), 0);
        while environment.decision_point() != DecisionPoint::TowerPlacement {
            let action = canonical_scripted_semantic_action(&environment).unwrap();
            let mut outcome = environment.semantic_step(action).unwrap();
            settle_forced_actions(&mut environment, &mut outcome).unwrap();
        }
        environment
    }

    #[test]
    fn no_reroll_candidates_omit_the_reroll_slot() {
        let environment = tower_placement_environment();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let pools = TeacherSelectionPools::production();
        let proposal = build_s41_proposal(&environment, &baseline, &pools).unwrap();
        assert!(!proposal.iter().any(|candidate| action_kind(&candidate.id) == "reroll"));
    }

    #[test]
    fn fewer_than_four_build_candidates_still_works() {
        let environment = tower_placement_environment();
        let baseline = canonical_scripted_semantic_action(&environment).unwrap();
        let pools = TeacherSelectionPools::production();
        let proposal = build_s41_proposal(&environment, &baseline, &pools).unwrap();
        assert!(!proposal.iter().any(|candidate| action_kind(&candidate.id) == "build_tower"));
    }

    // --- statistics ------------------------------------------------------

    #[test]
    fn paired_t_test_known_values() {
        // n=5, mean 1.0, sd exactly computable by hand.
        let deltas = vec![0.0, 1.0, 1.0, 1.0, 2.0];
        let (mean, sd, se, t, p) = paired_one_sided_t_test(&deltas);
        assert!((mean - 1.0).abs() < 1e-9);
        assert!((sd - 0.707_106_78).abs() < 1e-6);
        assert!((se - 0.316_227_77).abs() < 1e-6);
        assert!((t - 3.162_277_66).abs() < 1e-5);
        // one-sided p for t=3.1623, df=4 is about 0.01704 (R: pt(3.1623,4,lower.tail=FALSE))
        assert!((p - 0.01704).abs() < 2e-4, "p={p}");
    }

    #[test]
    fn paired_t_test_zero_at_t_zero() {
        let deltas = vec![-1.0, 1.0, -1.0, 1.0];
        let (mean, _, _, t, p) = paired_one_sided_t_test(&deltas);
        assert_eq!(mean, 0.0);
        assert_eq!(t, 0.0);
        assert!((p - 0.5).abs() < 1e-9);
    }

    #[test]
    fn paired_t_test_zero_variance_cases() {
        let (_, sd, se, t, p) = paired_one_sided_t_test(&[2.0, 2.0, 2.0]);
        assert_eq!(sd, 0.0);
        assert_eq!(se, 0.0);
        assert_eq!(t, f64::INFINITY);
        assert_eq!(p, 0.0);

        let (_, sd, _, t, p) = paired_one_sided_t_test(&[0.0, 0.0, 0.0]);
        assert_eq!(sd, 0.0);
        assert_eq!(t, 0.0);
        assert_eq!(p, 1.0);

        let (_, sd, _, t, p) = paired_one_sided_t_test(&[-3.0, -3.0]);
        assert_eq!(sd, 0.0);
        assert_eq!(t, f64::NEG_INFINITY);
        assert_eq!(p, 1.0);
    }

    #[test]
    fn holm_bonferroni_single_hypothesis_matches_raw_p() {
        let result = holm_bonferroni(&[0.03], 0.05);
        assert_eq!(result, vec![(0.03, true)]);
        let result = holm_bonferroni(&[0.10], 0.05);
        assert_eq!(result, vec![(0.10, false)]);
    }

    #[test]
    fn holm_bonferroni_two_hypotheses() {
        // p = [0.01, 0.04]: smallest gets *2 = 0.02 (< .05, pass), largest
        // gets max(0.02, 0.04*1) = 0.04 (< .05, pass).
        let result = holm_bonferroni(&[0.01, 0.04], 0.05);
        assert!((result[0].0 - 0.02).abs() < 1e-12);
        assert!(result[0].1);
        assert!((result[1].0 - 0.04).abs() < 1e-12);
        assert!(result[1].1);
    }

    #[test]
    fn holm_bonferroni_three_hypotheses_step_down() {
        // p = [0.001, 0.02, 0.03]: adjusted = [0.003, 0.04, 0.04(monotone)].
        let result = holm_bonferroni(&[0.001, 0.02, 0.03], 0.05);
        assert!((result[0].0 - 0.003).abs() < 1e-12);
        assert!(result[0].1);
        assert!((result[1].0 - 0.04).abs() < 1e-12);
        assert!(result[1].1);
        // 0.03 * 1 = 0.03, but monotonicity forces max(0.03, running_max=0.04) = 0.04.
        assert!((result[2].0 - 0.04).abs() < 1e-12);
        assert!(result[2].1);
    }

    #[test]
    fn holm_bonferroni_rejects_when_correction_pushes_over_alpha() {
        // p = [0.02, 0.03]: smallest *2 = 0.04 (pass), largest *1 = 0.03 but
        // monotone floor is 0.04 (still pass) - use a case that fails: p =
        // [0.03, 0.04] -> adjusted [0.06, 0.06], both fail.
        let result = holm_bonferroni(&[0.03, 0.04], 0.05);
        assert!(!result[0].1);
        assert!(!result[1].1);
    }

    // --- gate selection logic (synthetic, no rollouts) --------------------

    fn synthetic_gate(deltas: &[Vec<f64>]) -> (Vec<CandidateValidationStat>, Option<String>) {
        let mut validation: Vec<CandidateValidationStat> = deltas
            .iter()
            .enumerate()
            .map(|(index, d)| {
                let (mean, sd, se, t, p) = paired_one_sided_t_test(d);
                CandidateValidationStat {
                    action_id: format!("candidate:{index}"),
                    mean_delta: mean,
                    sd_delta: sd,
                    se_delta: se,
                    t_statistic: t,
                    p_value: p,
                    p_holm: 0.0,
                    passed: false,
                }
            })
            .collect();
        let p_values: Vec<f64> = validation.iter().map(|s| s.p_value).collect();
        for (stat, (adjusted, passed)) in validation.iter_mut().zip(holm_bonferroni(&p_values, FWER_ALPHA)) {
            stat.p_holm = adjusted;
            stat.passed = passed;
        }
        let selected = validation
            .iter()
            .filter(|stat| stat.passed)
            .max_by(|a, b| {
                a.mean_delta
                    .total_cmp(&b.mean_delta)
                    .then_with(|| b.action_id.cmp(&a.action_id))
            })
            .map(|stat| stat.action_id.clone());
        (validation, selected)
    }

    #[test]
    fn gate_keeps_baseline_when_no_candidate_passes() {
        let noisy = vec![-1.0, 1.0, -1.0, 1.0, -1.0, 1.0];
        let (_, selected) = synthetic_gate(&[noisy.clone(), noisy]);
        assert_eq!(selected, None);
    }

    #[test]
    fn gate_picks_max_mean_delta_among_passing_candidates() {
        let strong = vec![5.0; 10];
        let weaker_but_still_passing = vec![1.0; 10];
        let (_, selected) = synthetic_gate(&[weaker_but_still_passing, strong]);
        assert_eq!(selected.as_deref(), Some("candidate:1"));
    }

    #[test]
    fn discovery_top1_can_lose_to_top2_after_validation() {
        // Discovery top-1 has a large, noisy discovery-stage mean that does
        // not replicate on independent validation seeds (sd is large enough
        // that its one-sided p-value fails Holm); top-2's smaller but
        // consistent delta passes.
        let bad_top1 = vec![10.0, -8.0, 9.0, -9.0, 8.0, -10.0, 9.0, -8.0];
        let good_top2 = vec![1.0, 1.2, 0.9, 1.1, 1.0, 0.95, 1.05, 1.0];
        let (validation, selected) = synthetic_gate(&[bad_top1, good_top2]);
        assert!(!validation[0].passed, "noisy top-1 must fail the gate");
        assert!(validation[1].passed, "consistent top-2 must pass the gate");
        assert_eq!(selected.as_deref(), Some("candidate:1"));
    }

    // --- end-to-end determinism -------------------------------------------

    #[test]
    fn same_state_and_pools_reproduce_the_selection_exactly() {
        let (_, environment) = find_shop_seed();
        let pools = TeacherSelectionPools::small_for_tests();
        let (first, first_action) = select_teacher_action(&environment, &pools).unwrap();
        let (second, second_action) = select_teacher_action(&environment, &pools).unwrap();
        assert_eq!(first, second);
        assert_eq!(first_action, second_action);
    }

    /// Same check, full production scenario budget (8/32/64) - expensive
    /// (~15 minutes), so not part of the default `cargo test` run. Run
    /// explicitly (`cargo test -- --ignored teacher_selection`) before
    /// relying on a held-out or production run.
    #[test]
    #[ignore = "full production scenario budget; run explicitly, not on every `cargo test`"]
    fn same_state_and_pools_reproduce_the_selection_exactly_at_production_budget() {
        let (_, environment) = find_shop_seed();
        let pools = TeacherSelectionPools::production();
        let (first, first_action) = select_teacher_action(&environment, &pools).unwrap();
        let (second, second_action) = select_teacher_action(&environment, &pools).unwrap();
        assert_eq!(first, second);
        assert_eq!(first_action, second_action);
    }

    #[test]
    fn disjoint_pools_are_enforced() {
        let mut pools = TeacherSelectionPools::production();
        pools.validation_seeds[0] = pools.discovery_seeds[0];
        assert!(pools.validate().is_err());
    }

    // --- Phase 3N regression -------------------------------------------

    /// Feeds the exact raw paired deltas recorded by the Phase 3N diagnostic
    /// (baseline + S4/1 discovery top-3, independent validation seeds
    /// 9000..9063 - see docs/game-ai/05-rollout-teacher.md) through the
    /// production statistics/gate implementation and checks it reproduces
    /// the decision that diagnostic reported. No rollouts here - this is a
    /// pure statistics-layer regression against real recorded data, so it
    /// runs in milliseconds and is part of the default test run.
    #[test]
    fn reproduces_phase_3n_diagnostic_gate_decisions() {
        for state in crate::teacher_selection_phase3n_fixture::PHASE_3N_STATES {
            let mut validation: Vec<CandidateValidationStat> = state
                .candidates
                .iter()
                .map(|candidate| {
                    let (mean, sd, se, t, p) = paired_one_sided_t_test(candidate.deltas);
                    CandidateValidationStat {
                        action_id: candidate.action_id.to_string(),
                        mean_delta: mean,
                        sd_delta: sd,
                        se_delta: se,
                        t_statistic: t,
                        p_value: p,
                        p_holm: 0.0,
                        passed: false,
                    }
                })
                .collect();
            let p_values: Vec<f64> = validation.iter().map(|stat| stat.p_value).collect();
            for (stat, (adjusted, passed)) in
                validation.iter_mut().zip(holm_bonferroni(&p_values, FWER_ALPHA))
            {
                stat.p_holm = adjusted;
                stat.passed = passed;
            }
            let selected = validation
                .iter()
                .filter(|stat| stat.passed)
                .max_by(|a, b| {
                    a.mean_delta
                        .total_cmp(&b.mean_delta)
                        .then_with(|| b.action_id.cmp(&a.action_id))
                })
                .map(|stat| stat.action_id.clone())
                .unwrap_or_else(|| state.baseline_action_id.to_string());
            assert_eq!(
                selected, state.expected_selected,
                "state {}: expected {}, got {} (validation stats: {:?})",
                state.label, state.expected_selected, selected, validation
            );
        }
    }
}
