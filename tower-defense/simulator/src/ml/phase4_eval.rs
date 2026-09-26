//! Phase 4A paired terminal evaluation of the canonical scripted baseline and
//! search-free learned policies.

use super::phase4_dataset::{MAX_EPISODE_DECISIONS, step_to_next_decision};
use super::semantic_bc::SemanticPolicy;
use crate::config::GameConfig;
use crate::environment::{DecisionPoint, GameEnvironment};
use crate::policy_runner::canonical_scripted_semantic_action;
use anyhow::{Result, bail};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub enum EvalPolicy {
    Canonical,
    Learned {
        name: String,
        policy: SemanticPolicy,
    },
}

impl EvalPolicy {
    pub fn name(&self) -> &str {
        match self {
            Self::Canonical => "canonical",
            Self::Learned { name, .. } => name,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PolicyEpisode {
    pub seed: u64,
    pub policy: String,
    pub terminal_clear_rate: f32,
    pub final_stage: usize,
    pub victory: bool,
    pub decisions: usize,
    pub illegal_actions: usize,
    pub fallback_actions: usize,
    pub guard_interventions: usize,
    pub canonical_agreement: usize,
    pub chosen_kind_counts: BTreeMap<String, usize>,
    pub decision_seconds: f64,
    pub forward_seconds: f64,
    pub wall_seconds: f64,
    pub final_state_hash: String,
}

pub fn run_policy_episode(
    config: Arc<GameConfig>,
    seed: u64,
    policy: &EvalPolicy,
) -> Result<PolicyEpisode> {
    let started = Instant::now();
    let mut environment = GameEnvironment::new(config, seed);
    let mut episode = PolicyEpisode {
        seed,
        policy: policy.name().to_string(),
        terminal_clear_rate: 0.0,
        final_stage: 0,
        victory: false,
        decisions: 0,
        illegal_actions: 0,
        fallback_actions: 0,
        guard_interventions: 0,
        canonical_agreement: 0,
        chosen_kind_counts: BTreeMap::new(),
        decision_seconds: 0.0,
        forward_seconds: 0.0,
        wall_seconds: 0.0,
        final_state_hash: String::new(),
    };
    let mut recent = Vec::new();
    while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
        if episode.decisions >= MAX_EPISODE_DECISIONS {
            bail!(
                "seed {seed} policy {}: reached the {MAX_EPISODE_DECISIONS}-decision safety cap \
                 - invariant failure (stage {}, last actions {:?})",
                policy.name(),
                environment.snapshot().stage,
                recent
            );
        }
        let decision_started = Instant::now();
        let action = match policy {
            EvalPolicy::Canonical => {
                let action = canonical_scripted_semantic_action(&environment)?;
                episode.canonical_agreement += 1;
                action
            }
            EvalPolicy::Learned { policy, .. } => match policy.choose(&environment) {
                Ok(choice) => {
                    episode.forward_seconds += choice.forward_seconds;
                    episode.guard_interventions += choice.guard_intervention as usize;
                    let candidate = &choice.candidates.candidates[choice.index];
                    if !choice.legal_mask[choice.index]
                        || !environment.semantic_action_is_legal(&candidate.action)
                    {
                        episode.illegal_actions += 1;
                        episode.fallback_actions += 1;
                        canonical_scripted_semantic_action(&environment)?
                    } else {
                        if choice.candidates.canonical_index() == Some(choice.index) {
                            episode.canonical_agreement += 1;
                        }
                        candidate.action.clone()
                    }
                }
                Err(error) => {
                    eprintln!("seed {seed}: policy failed ({error}); canonical fallback");
                    episode.fallback_actions += 1;
                    canonical_scripted_semantic_action(&environment)?
                }
            },
        };
        episode.decision_seconds += decision_started.elapsed().as_secs_f64();
        recent.push(format!(
            "{:?}:{}",
            environment.decision_point(),
            action.action_id()
        ));
        if recent.len() > 12 {
            recent.remove(0);
        }
        *episode
            .chosen_kind_counts
            .entry(action.kind().wire_name().to_string())
            .or_insert(0) += 1;
        episode.decisions += 1;
        if step_to_next_decision(&mut environment, action)? {
            break;
        }
    }
    episode.terminal_clear_rate = environment.clear_rate();
    episode.victory = episode.terminal_clear_rate >= 100.0;
    episode.final_stage = environment.snapshot().stage;
    episode.final_state_hash = environment.state_hash();
    episode.wall_seconds = started.elapsed().as_secs_f64();
    Ok(episode)
}

#[derive(Clone, Debug, Serialize)]
pub struct PolicySummary {
    pub policy: String,
    pub episodes: usize,
    pub mean_terminal_clear_rate: f64,
    pub median_terminal_clear_rate: f64,
    pub mean_final_stage: f64,
    pub victories: usize,
    pub mean_decisions: f64,
    pub illegal_actions: usize,
    pub fallback_actions: usize,
    pub guard_interventions: usize,
    pub canonical_agreement_rate: f64,
    pub chosen_kind_counts: BTreeMap<String, usize>,
    pub mean_decision_ms: f64,
    pub mean_forward_ms: f64,
    pub decisions_per_second: f64,
    pub mean_episode_wall_seconds: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct PairedComparison {
    pub policy: String,
    pub reference: String,
    pub deltas: Vec<f32>,
    pub mean: f64,
    pub se: f64,
    pub ci95: (f64, f64),
    pub median: f64,
    pub better: usize,
    pub worse: usize,
    pub tie: usize,
}

fn median(values: &mut [f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

pub fn summarize_policy(policy: &str, episodes: &[&PolicyEpisode]) -> PolicySummary {
    let count = episodes.len().max(1) as f64;
    let decisions = episodes
        .iter()
        .map(|episode| episode.decisions)
        .sum::<usize>();
    let decision_seconds = episodes
        .iter()
        .map(|episode| episode.decision_seconds)
        .sum::<f64>();
    let mut chosen_kind_counts = BTreeMap::new();
    for episode in episodes {
        for (kind, value) in &episode.chosen_kind_counts {
            *chosen_kind_counts.entry(kind.clone()).or_insert(0) += value;
        }
    }
    PolicySummary {
        policy: policy.to_string(),
        episodes: episodes.len(),
        mean_terminal_clear_rate: episodes
            .iter()
            .map(|episode| episode.terminal_clear_rate as f64)
            .sum::<f64>()
            / count,
        median_terminal_clear_rate: median(
            &mut episodes
                .iter()
                .map(|episode| episode.terminal_clear_rate as f64)
                .collect::<Vec<_>>(),
        ),
        mean_final_stage: episodes
            .iter()
            .map(|episode| episode.final_stage as f64)
            .sum::<f64>()
            / count,
        victories: episodes.iter().filter(|episode| episode.victory).count(),
        mean_decisions: decisions as f64 / count,
        illegal_actions: episodes.iter().map(|episode| episode.illegal_actions).sum(),
        fallback_actions: episodes
            .iter()
            .map(|episode| episode.fallback_actions)
            .sum(),
        guard_interventions: episodes
            .iter()
            .map(|episode| episode.guard_interventions)
            .sum(),
        canonical_agreement_rate: episodes
            .iter()
            .map(|episode| episode.canonical_agreement)
            .sum::<usize>() as f64
            / decisions.max(1) as f64,
        chosen_kind_counts,
        mean_decision_ms: decision_seconds * 1_000.0 / decisions.max(1) as f64,
        mean_forward_ms: episodes
            .iter()
            .map(|episode| episode.forward_seconds)
            .sum::<f64>()
            * 1_000.0
            / decisions.max(1) as f64,
        decisions_per_second: decisions as f64 / decision_seconds.max(f64::MIN_POSITIVE),
        mean_episode_wall_seconds: episodes
            .iter()
            .map(|episode| episode.wall_seconds)
            .sum::<f64>()
            / count,
    }
}

pub fn paired(
    policy: &str,
    reference: &str,
    by_seed: &BTreeMap<u64, BTreeMap<String, PolicyEpisode>>,
) -> PairedComparison {
    let deltas = by_seed
        .values()
        .map(|episodes| {
            episodes[policy].terminal_clear_rate - episodes[reference].terminal_clear_rate
        })
        .collect::<Vec<_>>();
    let n = deltas.len() as f64;
    let mean = deltas.iter().map(|delta| *delta as f64).sum::<f64>() / n.max(1.0);
    let variance = if deltas.len() > 1 {
        deltas
            .iter()
            .map(|delta| (*delta as f64 - mean).powi(2))
            .sum::<f64>()
            / (n - 1.0)
    } else {
        0.0
    };
    let se = (variance / n.max(1.0)).sqrt();
    PairedComparison {
        policy: policy.to_string(),
        reference: reference.to_string(),
        mean,
        se,
        ci95: (mean - 1.96 * se, mean + 1.96 * se),
        median: median(&mut deltas.iter().map(|delta| *delta as f64).collect::<Vec<_>>()),
        better: deltas.iter().filter(|delta| **delta > 0.0).count(),
        worse: deltas.iter().filter(|delta| **delta < 0.0).count(),
        tie: deltas.iter().filter(|delta| **delta == 0.0).count(),
        deltas,
    }
}

#[derive(Debug, Serialize)]
pub struct TerminalEvaluationReport {
    pub split: String,
    pub seeds: Vec<u64>,
    pub policies: Vec<String>,
    pub summaries: Vec<PolicySummary>,
    pub comparisons: Vec<PairedComparison>,
    pub episodes: Vec<PolicyEpisode>,
    pub wall_seconds: f64,
}

/// Runs every policy on every seed to the actual terminal state. Seeds run
/// in parallel; each episode is independent and deterministic.
pub fn evaluate_policies(
    config: Arc<GameConfig>,
    split: &str,
    seeds: &[u64],
    policies: &[EvalPolicy],
    comparisons: &[(String, String)],
) -> Result<TerminalEvaluationReport> {
    let started = Instant::now();
    let jobs = seeds
        .iter()
        .flat_map(|seed| policies.iter().map(move |policy| (*seed, policy)))
        .collect::<Vec<_>>();
    let episodes = jobs
        .par_iter()
        .map(|(seed, policy)| run_policy_episode(Arc::clone(&config), *seed, policy))
        .collect::<Result<Vec<_>>>()?;
    let mut by_seed: BTreeMap<u64, BTreeMap<String, PolicyEpisode>> = BTreeMap::new();
    for episode in &episodes {
        by_seed
            .entry(episode.seed)
            .or_default()
            .insert(episode.policy.clone(), episode.clone());
    }
    let names = policies
        .iter()
        .map(|policy| policy.name().to_string())
        .collect::<Vec<_>>();
    let summaries = names
        .iter()
        .map(|name| {
            summarize_policy(
                name,
                &episodes
                    .iter()
                    .filter(|episode| &episode.policy == name)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let comparisons = comparisons
        .iter()
        .map(|(policy, reference)| paired(policy, reference, &by_seed))
        .collect();
    Ok(TerminalEvaluationReport {
        split: split.to_string(),
        seeds: seeds.to_vec(),
        policies: names,
        summaries,
        comparisons,
        episodes,
        wall_seconds: started.elapsed().as_secs_f64(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::model::{
        DeepSetsActorCritic, InferenceBackend, ModelConfig, default_policy_device,
    };
    use crate::teacher_terminal_gate::run_canonical_terminal_episode;

    #[test]
    fn canonical_policy_episode_matches_terminal_gate_baseline() {
        let config = Arc::new(GameConfig::default_config());
        let episode = run_policy_episode(Arc::clone(&config), 4, &EvalPolicy::Canonical).unwrap();
        let reference = run_canonical_terminal_episode(config, 4).unwrap();
        assert_eq!(episode.terminal_clear_rate, reference.clear_rate);
        assert_eq!(episode.decisions, reference.decision_count);
        assert_eq!(episode.final_state_hash, reference.final_state_hash);
    }

    #[test]
    fn learned_policy_terminal_evaluation_is_reproducible_and_legal() {
        let config = Arc::new(GameConfig::default_config());
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let policy = EvalPolicy::Learned {
            name: "untrained".to_string(),
            policy: SemanticPolicy::new(model),
        };
        let first = run_policy_episode(Arc::clone(&config), 6, &policy).unwrap();
        let second = run_policy_episode(config, 6, &policy).unwrap();
        assert_eq!(first.final_state_hash, second.final_state_hash);
        assert_eq!(first.terminal_clear_rate, second.terminal_clear_rate);
        assert_eq!(first.decisions, second.decisions);
        assert_eq!(first.illegal_actions, 0);
        assert_eq!(first.fallback_actions, 0);
    }
}
