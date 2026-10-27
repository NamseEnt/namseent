//! Generic content-decision distillation using information-set rollout labels.
//! Only the zero-initialized candidate effect adapter is updated here.
use super::{
    model::{TrainBackend, default_policy_device},
    policy_v2::{PolicyNet, factorized_log_probs},
    retraining::write_json_atomic,
    semantic_bc::semantic_decision_with,
    semantic_candidates::{CandidateMode, PolicyCandidate, encode_decision},
};
use crate::{
    config::GameConfig,
    environment::{DecisionPoint, GameEnvironment, Observation},
};
use anyhow::{Result, ensure};
use burn::{
    optim::{AdamConfig, GradientsParams, Optimizer},
    tensor::{Int, Tensor, TensorData},
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc, time::Instant};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentTeacherSpec {
    pub games_per_config: usize,
    pub scenario_count: usize,
    pub seed_start: u64,
    pub epochs: usize,
    pub learning_rate: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Sample {
    observation: Observation,
    candidates: Vec<PolicyCandidate>,
    legal_mask: Vec<bool>,
    target: usize,
    scenario_rollouts: usize,
    scenario_decisions: usize,
}
fn collect(config: Arc<GameConfig>, seed: u64, spec: &ContentTeacherSpec) -> Result<Vec<Sample>> {
    let mut env = GameEnvironment::new(config, seed);
    let mut samples = Vec::new();
    for _ in 0..super::phase4_dataset::MAX_EPISODE_DECISIONS {
        if matches!(env.decision_point(), DecisionPoint::Terminal) {
            return Ok(samples);
        }
        let action = if matches!(env.decision_point(), DecisionPoint::TreasureSelection) {
            let decision = semantic_decision_with(&env, CandidateMode::Top8)?;
            let teacher = crate::teacher::evaluate_semantic_candidate_set(
                &env,
                &super::semantic_candidates::candidates_as_legal_actions(
                    &decision.candidates.candidates,
                ),
                &crate::teacher::RolloutTeacherConfig {
                    scenario_seeds: (0..spec.scenario_count as u64)
                        .map(|s| {
                            u64::from_le_bytes(
                                td_core::derive_seed(
                                    seed,
                                    td_core::domain::ML_TOWER_TEACHER_HIDDEN_ORDER,
                                    &[s],
                                )[..8]
                                    .try_into()
                                    .unwrap(),
                            )
                        })
                        .collect(),
                    horizon_sim_ticks: 1_000_000,
                    build_tower_rollout_limit: Some(8),
                },
            )?;
            let target = decision
                .candidates
                .index_of_action_id(&teacher.selected_action_id)
                .ok_or_else(|| anyhow::anyhow!("teacher action outside content candidates"))?;
            let action = decision.candidates.candidates[target].action.clone();
            samples.push(Sample {
                observation: decision.candidates.observation,
                candidates: decision.candidates.candidates,
                legal_mask: decision.legal_mask,
                target,
                scenario_rollouts: teacher.candidate_count * spec.scenario_count,
                scenario_decisions: teacher.candidates.iter().map(|c| c.total_decisions).sum(),
            });
            action
        } else {
            crate::policy_runner::canonical_scripted_semantic_action(&env)?
        };
        super::phase4_dataset::step_to_next_decision(&mut env, action)?;
    }
    anyhow::bail!("content teacher seed {seed} failed to terminate")
}
pub fn warmup(
    actor: &mut PolicyNet<TrainBackend>,
    source: &GameConfig,
    target: &GameConfig,
    spec: &ContentTeacherSpec,
    run_dir: &Path,
) -> Result<()> {
    ensure!(
        spec.games_per_config > 0
            && spec.scenario_count > 0
            && spec.epochs > 0
            && spec.learning_rate.is_finite()
            && spec.learning_rate > 0.0,
        "invalid content teacher spec"
    );
    let started = Instant::now();
    let device = default_policy_device();
    let configs = [Arc::new(source.clone()), Arc::new(target.clone())];
    let samples = (0..spec.games_per_config * 2)
        .into_par_iter()
        .map(|i| {
            collect(
                Arc::clone(&configs[i / spec.games_per_config]),
                spec.seed_start + (i % spec.games_per_config) as u64,
                spec,
            )
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    ensure!(!samples.is_empty(), "content teacher produced no decisions");
    write_json_atomic(&run_dir.join("content-teacher-samples.json"), &samples)?;
    let collection_seconds = started.elapsed().as_secs_f64();
    let encoded = samples
        .iter()
        .map(|s| encode_decision(&s.observation, &s.candidates, s.legal_mask.clone()))
        .collect::<Vec<_>>();
    let mut layer = actor
        .scorer
        .candidate_effect_adapter()
        .ok_or_else(|| anyhow::anyhow!("content teacher requires effect inputs"))?;
    let mut optimizer = AdamConfig::new().init();
    let mut losses = Vec::new();
    for _ in 0..spec.epochs {
        for start in (0..samples.len()).step_by(64) {
            let end = (start + 64).min(samples.len());
            let decisions = encoded[start..end].iter().collect::<Vec<_>>();
            actor.scorer = actor
                .scorer
                .clone()
                .with_candidate_effect_adapter(layer.clone());
            let distribution = factorized_log_probs(actor, &decisions, &device);
            let indices = Tensor::<TrainBackend, 2, Int>::from_data(
                TensorData::new(
                    samples[start..end]
                        .iter()
                        .map(|s| s.target as i64)
                        .collect::<Vec<_>>(),
                    [end - start, 1],
                ),
                &device,
            );
            let loss = -distribution.joint.gather(1, indices).mean();
            let value = loss.clone().into_data().to_vec::<f32>()?[0];
            ensure!(value.is_finite(), "nonfinite content teacher loss");
            losses.push(value);
            let gradients = GradientsParams::from_grads(loss.backward(), &layer);
            layer = optimizer.step(spec.learning_rate, layer, gradients);
        }
    }
    actor.scorer = actor.scorer.clone().with_candidate_effect_adapter(layer);
    write_json_atomic(
        &run_dir.join("content-teacher-report.json"),
        &serde_json::json!({"schema_version":1,"spec":spec,"games":spec.games_per_config*2,"content_decisions":samples.len(),"scenario_rollouts":samples.iter().map(|s|s.scenario_rollouts).sum::<usize>(),"scenario_decisions":samples.iter().map(|s|s.scenario_decisions).sum::<usize>(),"collection_seconds":collection_seconds,"seconds":started.elapsed().as_secs_f64(),"first_loss":losses.first(),"last_loss":losses.last(),"training_scope":"candidate effect adapter only; frozen base weights","continuation":"canonical scripted; hidden future order resampled per paired scenario; terminal horizon"}),
    )?;
    eprintln!(
        "content teacher: {} decisions, {} scenario rollouts, loss {:.4} -> {:.4}, {:.1}s",
        samples.len(),
        samples.iter().map(|s| s.scenario_rollouts).sum::<usize>(),
        losses[0],
        losses[losses.len() - 1],
        started.elapsed().as_secs_f64()
    );
    Ok(())
}
