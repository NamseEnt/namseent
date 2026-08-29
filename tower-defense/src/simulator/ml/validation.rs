use super::encoding::candidate_entity_rows;
use super::features::observation_features;
use super::model::{DeepSetsActorCritic, InferenceBackend, PolicyDevice};
use super::rollout::logits_for;
use super::rollout::{RolloutBatch, RolloutConfig, collect_rollouts};
use super::seed::SeedRange;
use crate::config::GameConfig;
use crate::simulator::environment::{
    AgentAction, LegalAction, Observation, RewardConfig, StepReason,
};
use crate::simulator::policy_runner::{self, BatchResult, PolicyRunnerConfig};
use anyhow::Result;
use burn::tensor::backend::Backend;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaselinePolicy {
    RandomLegal,
    Checkpoint,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineEpisodeReport {
    pub seed: u64,
    pub victory: bool,
    pub clear_rate: f32,
    pub final_stage: usize,
    pub decisions: usize,
    pub truncated: bool,
    pub no_progress_cycle: bool,
    pub escaped_hp: f32,
    pub player_damage: f32,
    pub towers_placed: usize,
    pub items_used: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineReport {
    pub policy: BaselinePolicy,
    pub seed_range: SeedRange,
    pub seed_digest: String,
    pub episodes: Vec<BaselineEpisodeReport>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedEpisodeReport {
    pub seed: u64,
    pub left_clear_rate: f32,
    pub right_clear_rate: f32,
    pub left_victory: bool,
    pub right_victory: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedBaselineReport {
    pub left: BaselinePolicy,
    pub right: BaselinePolicy,
    pub seed_range: SeedRange,
    pub seed_digest: String,
    pub episodes: Vec<PairedEpisodeReport>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluationProvenance {
    pub run_id: String,
    pub git_revision: String,
    pub config_digest: String,
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub checkpoint_path: String,
    pub checkpoint_sha256: String,
    pub seed_digest: String,
    pub max_decisions: usize,
    pub reward_config: RewardConfig,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FinalScore {
    pub full_clear_count: usize,
    pub mean_partial_clear_rate: f64,
    pub truncated_count: usize,
}

impl FinalScore {
    pub fn is_better_than(self, other: Self) -> bool {
        (
            self.full_clear_count,
            self.mean_partial_clear_rate,
            usize::MAX - self.truncated_count,
        ) > (
            other.full_clear_count,
            other.mean_partial_clear_rate,
            usize::MAX - other.truncated_count,
        )
    }
}

pub fn final_score(report: &ClearRateReport) -> FinalScore {
    FinalScore {
        full_clear_count: report.full_clear_count,
        mean_partial_clear_rate: report.mean_partial_clear_rate,
        truncated_count: report.truncation_breakdown.values().sum(),
    }
}

pub fn pair_baseline_reports(
    left: &BaselineReport,
    right: &BaselineReport,
) -> Result<Vec<PairedEpisodeReport>> {
    if left.seed_range != right.seed_range || left.seed_digest != right.seed_digest {
        anyhow::bail!("baseline reports must use the same seed range");
    }
    if left.episodes.len() != right.episodes.len() {
        anyhow::bail!("baseline reports must contain the same episode count");
    }
    left.episodes
        .iter()
        .zip(&right.episodes)
        .map(|(left, right)| {
            if left.seed != right.seed {
                anyhow::bail!("baseline report episode seeds are not aligned");
            }
            Ok(PairedEpisodeReport {
                seed: left.seed,
                left_clear_rate: left.clear_rate,
                right_clear_rate: right.clear_rate,
                left_victory: left.victory,
                right_victory: right.victory,
            })
        })
        .collect()
}

pub fn paired_baseline_report(
    left: &BaselineReport,
    right: &BaselineReport,
) -> Result<PairedBaselineReport> {
    Ok(PairedBaselineReport {
        left: left.policy,
        right: right.policy,
        seed_range: left.seed_range,
        seed_digest: left.seed_digest.clone(),
        episodes: pair_baseline_reports(left, right)?,
    })
}

impl BaselineReport {
    fn from_batch(policy: BaselinePolicy, seed_range: SeedRange, batch: BatchResult) -> Self {
        Self {
            policy,
            seed_digest: seed_range.digest(),
            seed_range,
            episodes: batch
                .episodes
                .into_iter()
                .map(|episode| BaselineEpisodeReport {
                    seed: episode.seed,
                    victory: episode.victory,
                    clear_rate: episode.clear_rate,
                    final_stage: episode.final_observation.stage,
                    decisions: episode.decision_count,
                    truncated: episode.truncated,
                    no_progress_cycle: episode.steps.as_ref().is_some_and(|steps| {
                        steps.iter().any(|step| {
                            step.outcome.info.reason
                                == crate::simulator::environment::StepReason::NoProgressCycle
                        })
                    }),
                    escaped_hp: episode.metrics.total_escaped_hp,
                    player_damage: episode.metrics.total_player_damage,
                    towers_placed: episode.metrics.total_towers_placed,
                    items_used: episode.metrics.total_items_used,
                })
                .collect(),
        }
    }
}

pub fn evaluate_baseline(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    policy: BaselinePolicy,
    max_decisions: usize,
) -> Result<BaselineReport> {
    let runner_config = PolicyRunnerConfig {
        max_decisions_per_episode: max_decisions,
        record_steps: true,
        max_stage: None,
        reward_config: crate::simulator::environment::RewardConfig::default(),
    };
    let batch = match policy {
        BaselinePolicy::RandomLegal => {
            policy_runner::run_batch(config, &seed_range.seeds(), &runner_config, |seed| {
                let mut state = seed ^ 0xD1B5_4A32_D192_ED03;
                move |_observation: &Observation, legal_actions: &[LegalAction]| {
                    state ^= state >> 12;
                    state ^= state << 25;
                    state ^= state >> 27;
                    let index = (state.wrapping_mul(2_685_821_657_736_338_717) as usize)
                        % legal_actions.len();
                    Ok(legal_actions[index].action.clone())
                }
            })?
        }
        BaselinePolicy::Checkpoint => unreachable!("checkpoint evaluation requires a model"),
    };
    Ok(BaselineReport::from_batch(policy, seed_range, batch))
}

pub fn evaluate_checkpoint(
    model: Arc<DeepSetsActorCritic<InferenceBackend>>,
    device: Arc<PolicyDevice>,
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions: usize,
) -> Result<BaselineReport> {
    let batch = policy_runner::run_batch(
        config,
        &seed_range.seeds(),
        &PolicyRunnerConfig {
            max_decisions_per_episode: max_decisions,
            record_steps: true,
            max_stage: None,
            reward_config: crate::simulator::environment::RewardConfig::default(),
        },
        move |_| {
            let model = Arc::clone(&model);
            let device = Arc::clone(&device);
            move |observation: &Observation, legal_actions: &[LegalAction]| -> Result<AgentAction> {
                let state = observation_features(observation);
                let rows = legal_actions
                    .iter()
                    .map(|legal| {
                        candidate_entity_rows(observation, &legal.action)
                            .pop()
                            .expect("candidate entity row must exist")
                    })
                    .collect::<Vec<_>>();
                let logits =
                    logits_for(model.as_ref(), device.as_ref(), observation, &state, &rows);
                let index = logits
                    .iter()
                    .enumerate()
                    .max_by(|left, right| {
                        left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0))
                    })
                    .map(|(index, _)| index)
                    .expect("checkpoint must produce logits");
                Ok(legal_actions[index].action.clone())
            }
        },
    )?;
    Ok(BaselineReport::from_batch(
        BaselinePolicy::Checkpoint,
        seed_range,
        batch,
    ))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearRateReport {
    pub seed_range: SeedRange,
    pub seed_digest: String,
    pub episodes: usize,
    pub clears: usize,
    pub clear_rate: f64,
    pub mean_final_stage: f64,
    pub decision_steps: usize,
    pub elapsed_seconds: f64,
    pub episodes_per_second: f64,
    pub decisions_per_second: f64,
    pub episode_returns: Vec<f32>,
    pub termination_reasons: Vec<StepReason>,
    pub full_clear_count: usize,
    pub mean_partial_clear_rate: f64,
    pub median_partial_clear_rate: f64,
    pub stage_p25: f64,
    pub stage_p50: f64,
    pub stage_p75: f64,
    pub stage_p90: f64,
    pub truncation_breakdown: BTreeMap<String, usize>,
    pub diagnostics: super::rollout::RolloutDiagnostics,
}

impl ClearRateReport {
    pub fn from_batch(seed_range: SeedRange, batch: &RolloutBatch, elapsed_seconds: f64) -> Self {
        let diagnostics = batch.diagnostics();
        let final_stages = batch
            .episodes
            .iter()
            .map(|episode| episode.final_stage as f64)
            .collect::<Vec<_>>();
        let clears = batch
            .episodes
            .iter()
            .filter(|episode| episode.cleared)
            .count();
        let episodes = batch.episodes.len();
        let decision_steps = batch.steps().count();
        let mut sorted_stages = final_stages.clone();
        sorted_stages.sort_by(f64::total_cmp);
        let partial_clear_rates = batch
            .episodes
            .iter()
            .filter(|episode| !episode.cleared)
            .map(|episode| f64::from(episode.final_clear_rate))
            .collect::<Vec<_>>();
        let mut truncation_breakdown = BTreeMap::new();
        for episode in &batch.episodes {
            if matches!(
                episode.termination_reason,
                StepReason::MaxTicks | StepReason::MaxDecisions | StepReason::NoProgressCycle
            ) {
                *truncation_breakdown
                    .entry(format!("{:?}", episode.termination_reason))
                    .or_insert(0) += 1;
            }
        }
        Self {
            seed_range,
            seed_digest: seed_range.digest(),
            episodes,
            clears,
            clear_rate: batch.clear_rate(),
            mean_final_stage: final_stages.iter().sum::<f64>() / final_stages.len().max(1) as f64,
            decision_steps,
            elapsed_seconds,
            episodes_per_second: episodes as f64 / elapsed_seconds.max(f64::EPSILON),
            decisions_per_second: decision_steps as f64 / elapsed_seconds.max(f64::EPSILON),
            episode_returns: batch
                .episodes
                .iter()
                .map(|episode| episode.episode_return)
                .collect(),
            termination_reasons: batch
                .episodes
                .iter()
                .map(|episode| episode.termination_reason.clone())
                .collect(),
            full_clear_count: clears,
            mean_partial_clear_rate: mean_f64(&partial_clear_rates),
            median_partial_clear_rate: percentile(&partial_clear_rates, 0.50),
            stage_p25: percentile(&sorted_stages, 0.25),
            stage_p50: percentile(&sorted_stages, 0.50),
            stage_p75: percentile(&sorted_stages, 0.75),
            stage_p90: percentile(&sorted_stages, 0.90),
            truncation_breakdown,
            diagnostics,
        }
    }

    pub fn canonical_json_without_timing(&self) -> String {
        let mut value = serde_json::to_value(self).expect("report should serialize");
        let object = value.as_object_mut().expect("report should be an object");
        for key in [
            "elapsed_seconds",
            "episodes_per_second",
            "decisions_per_second",
        ] {
            object.remove(key);
        }
        serde_json::to_string(&value).expect("canonical json should serialize")
    }
}

fn mean_f64(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn percentile(values: &[f64], probability: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values[((values.len() - 1) as f64 * probability).round() as usize]
}

pub fn evaluate_clear_rate<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    rollout_config: &RolloutConfig,
    progress: Option<&ProgressBar>,
) -> Result<ClearRateReport> {
    let started = std::time::Instant::now();
    let batch = collect_rollouts(
        model,
        device,
        config,
        &seed_range.seeds(),
        rollout_config,
        progress,
    )?;
    let elapsed_seconds = started.elapsed().as_secs_f64();
    Ok(ClearRateReport::from_batch(
        seed_range,
        &batch,
        elapsed_seconds,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulator::ml::model::{InferenceBackend, ModelConfig, default_policy_device};
    use crate::simulator::ml::rollout::EpisodeRollout;
    use std::sync::Arc;

    #[test]
    fn clear_rate_report_preserves_exact_seed_digest() {
        let range = SeedRange::try_new(2, 4).unwrap();
        let report = ClearRateReport {
            seed_range: range,
            seed_digest: range.digest(),
            episodes: 3,
            clears: 1,
            clear_rate: 1.0 / 3.0,
            mean_final_stage: 2.0,
            decision_steps: 30,
            elapsed_seconds: 1.5,
            episodes_per_second: 2.0,
            decisions_per_second: 20.0,
            episode_returns: vec![1.0, -1.0, 0.0],
            termination_reasons: vec![StepReason::Terminal; 3],
            full_clear_count: 1,
            mean_partial_clear_rate: 0.5,
            median_partial_clear_rate: 0.5,
            stage_p25: 1.0,
            stage_p50: 2.0,
            stage_p75: 2.0,
            stage_p90: 2.0,
            truncation_breakdown: BTreeMap::new(),
            diagnostics: RolloutBatch {
                episodes: Vec::new(),
            }
            .diagnostics(),
        };
        assert_eq!(report.seed_digest, range.digest());
        let _ = DeepSetsActorCritic::<InferenceBackend>::new(
            ModelConfig::default(),
            &default_policy_device(),
        );
    }

    #[test]
    fn baseline_reports_are_deterministic_and_seed_ordered() {
        let range = SeedRange::try_new(0, 1).expect("valid range");
        let first = evaluate_baseline(
            Arc::new(GameConfig::default_config()),
            range,
            BaselinePolicy::RandomLegal,
            1,
        )
        .expect("baseline should run");
        let second = evaluate_baseline(
            Arc::new(GameConfig::default_config()),
            range,
            BaselinePolicy::RandomLegal,
            1,
        )
        .expect("baseline should rerun");
        assert_eq!(first, second);
        assert_eq!(
            first
                .episodes
                .iter()
                .map(|episode| episode.seed)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
    }

    #[test]
    fn paired_reports_require_and_preserve_seed_alignment() {
        let range = SeedRange::try_new(0, 1).expect("valid range");
        let left = evaluate_baseline(
            Arc::new(GameConfig::default_config()),
            range,
            BaselinePolicy::RandomLegal,
            1,
        )
        .expect("left baseline should run");
        let right = evaluate_baseline(
            Arc::new(GameConfig::default_config()),
            range,
            BaselinePolicy::RandomLegal,
            1,
        )
        .expect("right baseline should run");
        let paired = pair_baseline_reports(&left, &right).expect("reports should pair");
        assert_eq!(
            paired
                .iter()
                .map(|episode| episode.seed)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
    }

    #[test]
    fn clear_rate_report_is_byte_deterministic_without_timing() {
        let range = SeedRange::try_new(0, 2).expect("valid range");
        let make_batch = |elapsed_seconds: f64| {
            ClearRateReport::from_batch(
                range,
                &RolloutBatch {
                    episodes: vec![
                        EpisodeRollout {
                            seed: 0,
                            steps: Vec::new(),
                            cleared: true,
                            final_clear_rate: 1.0,
                            final_stage: 3,
                            escaped_hp: 0.0,
                            player_damage: 12.0,
                            tower_damage: 0.0,
                            episode_return: 5.0,
                            termination_reason: StepReason::Terminal,
                            reward_component_sums: BTreeMap::new(),
                            decision_point_counts: BTreeMap::new(),
                            forced_action_count: 0,
                            forced_action_counts: BTreeMap::new(),
                        },
                        EpisodeRollout {
                            seed: 1,
                            steps: Vec::new(),
                            cleared: false,
                            final_clear_rate: 0.4,
                            final_stage: 2,
                            escaped_hp: 6.0,
                            player_damage: 8.0,
                            tower_damage: 0.0,
                            episode_return: -1.5,
                            termination_reason: StepReason::MaxDecisions,
                            reward_component_sums: BTreeMap::new(),
                            decision_point_counts: BTreeMap::new(),
                            forced_action_count: 0,
                            forced_action_counts: BTreeMap::new(),
                        },
                    ],
                },
                elapsed_seconds,
            )
        };
        let first_report = make_batch(1.0);
        let second_report = make_batch(7.5);
        assert_ne!(first_report.elapsed_seconds, second_report.elapsed_seconds);
        assert_eq!(
            first_report.canonical_json_without_timing(),
            second_report.canonical_json_without_timing()
        );
    }

    #[test]
    fn final_score_prefers_full_clear_then_partial_progress_then_less_truncation() {
        let base = FinalScore {
            full_clear_count: 1,
            mean_partial_clear_rate: 0.5,
            truncated_count: 2,
        };
        assert!(
            FinalScore {
                full_clear_count: 2,
                ..base
            }
            .is_better_than(base)
        );
        assert!(
            FinalScore {
                mean_partial_clear_rate: 0.6,
                ..base
            }
            .is_better_than(base)
        );
        assert!(
            FinalScore {
                truncated_count: 1,
                ..base
            }
            .is_better_than(base)
        );
    }
}
