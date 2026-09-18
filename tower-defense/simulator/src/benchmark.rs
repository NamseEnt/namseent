use crate::config::{self, GameConfig};
use crate::environment::{AgentAction, LegalAction, Observation, RewardConfig};
use crate::policy_runner::{EnvironmentPolicy, PolicyRunnerConfig, run_batch, run_semantic_batch};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::BTreeMap;
use std::sync::Arc;

pub const BENCHMARK_SCHEMA_VERSION: u32 = 3;
pub const BENCHMARK_SEED_SCHEDULE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkProvenance {
    pub benchmark_schema_version: u32,
    pub policy: String,
    pub action_mode: String,
    pub config_digest: String,
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub seed_start: u64,
    pub seed_end: u64,
    pub seed_digest: String,
    pub max_decisions: usize,
    pub threads: usize,
    pub record_steps: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub provenance: BenchmarkProvenance,
    pub episodes: usize,
    pub victories: usize,
    pub full_clear_rate: f64,
    pub decisions: usize,
    pub ticks_advanced: u64,
    pub candidate_evaluations: usize,
    pub placement_position_checks: usize,
    pub mean_final_stage: f64,
    pub elapsed_seconds: f64,
    pub episodes_per_second: f64,
    pub decisions_per_second: f64,
    pub ticks_per_second: f64,
    pub candidate_evaluations_per_second: f64,
    pub placement_position_checks_per_second: f64,
    pub mean_candidates_per_decision: f64,
    pub termination_reasons: BTreeMap<String, usize>,
}

pub fn run_policy<P, F>(
    config: Arc<GameConfig>,
    seeds: &[u64],
    policy_name: impl Into<String>,
    max_decisions: usize,
    threads: usize,
    policy_factory: F,
) -> Result<BenchmarkReport>
where
    P: EnvironmentPolicy,
    F: Fn(u64) -> P + Sync,
{
    run_policy_with_mode(
        config,
        seeds,
        policy_name,
        max_decisions,
        threads,
        policy_factory,
        false,
    )
}

pub fn run_semantic_policy<P, F>(
    config: Arc<GameConfig>,
    seeds: &[u64],
    policy_name: impl Into<String>,
    max_decisions: usize,
    threads: usize,
    policy_factory: F,
) -> Result<BenchmarkReport>
where
    P: EnvironmentPolicy,
    F: Fn(u64) -> P + Sync,
{
    run_policy_with_mode(
        config,
        seeds,
        policy_name,
        max_decisions,
        threads,
        policy_factory,
        true,
    )
}

fn run_policy_with_mode<P, F>(
    config: Arc<GameConfig>,
    seeds: &[u64],
    policy_name: impl Into<String>,
    max_decisions: usize,
    threads: usize,
    policy_factory: F,
    semantic_actions: bool,
) -> Result<BenchmarkReport>
where
    P: EnvironmentPolicy,
    F: Fn(u64) -> P + Sync,
{
    if seeds.is_empty() {
        anyhow::bail!("benchmark requires at least one seed");
    }
    if seeds.windows(2).any(|window| window[0] >= window[1]) {
        anyhow::bail!("benchmark seeds must be strictly increasing");
    }
    if max_decisions == 0 {
        anyhow::bail!("benchmark requires a positive max decision count");
    }
    let runner_config = PolicyRunnerConfig {
        max_decisions_per_episode: max_decisions,
        record_steps: false,
        max_stage: None,
        reward_config: RewardConfig::default(),
    };
    let started = std::time::Instant::now();
    let batch = if semantic_actions {
        run_semantic_batch(Arc::clone(&config), seeds, &runner_config, policy_factory)?
    } else {
        run_batch(Arc::clone(&config), seeds, &runner_config, policy_factory)?
    };
    let elapsed_seconds = started.elapsed().as_secs_f64();
    let episodes = batch.episodes.len();
    let victories = batch
        .episodes
        .iter()
        .filter(|episode| episode.victory)
        .count();
    let decisions = batch
        .episodes
        .iter()
        .map(|episode| episode.decision_count)
        .sum::<usize>();
    let ticks_advanced = batch
        .episodes
        .iter()
        .map(|episode| episode.ticks_advanced)
        .sum::<u64>();
    let candidate_evaluations = batch
        .episodes
        .iter()
        .map(|episode| episode.candidate_evaluations)
        .sum::<usize>();
    let placement_position_checks = batch
        .episodes
        .iter()
        .map(|episode| episode.placement_position_checks)
        .sum::<usize>();
    let mean_final_stage = batch
        .episodes
        .iter()
        .map(|episode| episode.final_observation.stage as f64)
        .sum::<f64>()
        / episodes as f64;
    let mut termination_reasons = BTreeMap::new();
    for episode in &batch.episodes {
        *termination_reasons
            .entry(format!("{:?}", episode.termination_reason))
            .or_insert(0) += 1;
    }
    let elapsed_for_rate = elapsed_seconds.max(f64::EPSILON);
    let seed_start = *seeds.first().expect("non-empty seeds checked");
    let seed_end = *seeds.last().expect("non-empty seeds checked");
    let provenance = BenchmarkProvenance {
        benchmark_schema_version: BENCHMARK_SCHEMA_VERSION,
        policy: policy_name.into(),
        action_mode: if semantic_actions {
            "semantic".to_string()
        } else {
            "legacy".to_string()
        },
        config_digest: config::config_digest(config.as_ref()),
        environment_version: crate::environment::ENVIRONMENT_VERSION,
        action_schema_version: crate::environment::ACTION_SCHEMA_VERSION,
        seed_start,
        seed_end,
        seed_digest: seed_digest(seeds),
        max_decisions,
        threads,
        record_steps: false,
    };
    Ok(BenchmarkReport {
        provenance,
        episodes,
        victories,
        full_clear_rate: victories as f64 / episodes as f64,
        decisions,
        ticks_advanced,
        candidate_evaluations,
        placement_position_checks,
        mean_final_stage,
        elapsed_seconds,
        episodes_per_second: episodes as f64 / elapsed_for_rate,
        decisions_per_second: decisions as f64 / elapsed_for_rate,
        ticks_per_second: ticks_advanced as f64 / elapsed_for_rate,
        candidate_evaluations_per_second: candidate_evaluations as f64 / elapsed_for_rate,
        placement_position_checks_per_second: placement_position_checks as f64 / elapsed_for_rate,
        mean_candidates_per_decision: candidate_evaluations as f64 / decisions.max(1) as f64,
        termination_reasons,
    })
}

pub fn random_legal_policy(seed: u64) -> impl EnvironmentPolicy {
    let mut random_state = seed ^ 0xD1B5_4A32_D192_ED03;
    move |_observation: &Observation, legal_actions: &[LegalAction]| {
        random_state ^= random_state >> 12;
        random_state ^= random_state << 25;
        random_state ^= random_state >> 27;
        let index =
            (random_state.wrapping_mul(2_685_821_657_736_338_717) as usize) % legal_actions.len();
        Ok(legal_actions[index].action.clone())
    }
}

pub fn scripted_policy(
    observation: &Observation,
    legal_actions: &[LegalAction],
) -> Result<AgentAction> {
    crate::policy_runner::scripted_expert_action(observation, legal_actions)
}

fn seed_digest(seeds: &[u64]) -> String {
    let mut digest = sha2::Sha256::new();
    digest.update(b"tower-defense-benchmark-seed-list");
    digest.update(BENCHMARK_SEED_SCHEDULE_SCHEMA_VERSION.to_be_bytes());
    digest.update((seeds.len() as u64).to_be_bytes());
    for seed in seeds {
        digest.update(seed.to_be_bytes());
    }
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_aggregates_without_recording_steps() {
        let report = run_policy(
            Arc::new(GameConfig::default_config()),
            &[0],
            "random_legal",
            1,
            1,
            random_legal_policy,
        )
        .expect("benchmark should run");

        assert_eq!(report.provenance.record_steps, false);
        assert_eq!(report.episodes, 1);
        assert_eq!(report.decisions, 1);
        assert!(report.candidate_evaluations > 0);
    }

    #[test]
    fn report_rejects_unsorted_seeds() {
        let error = run_policy(
            Arc::new(GameConfig::default_config()),
            &[1, 0],
            "random_legal",
            1,
            1,
            random_legal_policy,
        )
        .expect_err("unsorted seeds should be rejected");

        assert!(error.to_string().contains("strictly increasing"));
    }

    #[test]
    fn semantic_report_records_action_mode() {
        let report = run_semantic_policy(
            Arc::new(GameConfig::default_config()),
            &[0],
            "scripted_expert",
            8,
            1,
            |_| scripted_policy,
        )
        .expect("semantic benchmark should run");

        assert_eq!(report.provenance.action_mode, "semantic");
        assert!(report.candidate_evaluations > 0);
    }
}
