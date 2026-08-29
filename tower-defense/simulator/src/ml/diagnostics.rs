use super::encoding::{EntityRow, candidate_rows_for_legal_actions};
use super::features::observation_features;
use super::model::{DeepSetsActorCritic, InferenceBackend, PolicyDevice};
use super::rollout::RolloutConfig;
use crate::config::GameConfig;
use crate::environment::{LegalAction, Observation};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticStep {
    pub decision_point: crate::environment::DecisionPoint,
    pub legal_action_ids: Vec<String>,
    pub candidate_rows: Vec<EntityRow>,
    pub duplicate_candidate_row_groups: Vec<Vec<usize>>,
    pub logits: Vec<f32>,
    pub selected_index: usize,
    pub selected_action_id: String,
    pub pre_progress_fingerprint: String,
    pub post_progress_fingerprint: String,
    pub state_hash: String,
    pub reward: crate::environment::RewardComponents,
    pub terminated: bool,
    pub truncated: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticTrace {
    pub seed: u64,
    pub steps: Vec<DiagnosticStep>,
    pub final_state_hash: String,
    pub final_stage: usize,
    pub episode_return: f32,
    pub termination_reason: crate::environment::StepReason,
    pub first_repeated_state_hash: Option<String>,
    pub first_repeated_state_step: Option<usize>,
    pub first_repeated_state_period: Option<usize>,
    pub first_repeated_fingerprint_step: Option<usize>,
}

pub fn collect_diagnostic_trace(
    model: Arc<DeepSetsActorCritic<InferenceBackend>>,
    device: Arc<PolicyDevice>,
    config: Arc<GameConfig>,
    seed: u64,
    rollout_config: &RolloutConfig,
) -> Result<DiagnosticTrace> {
    let decisions = Arc::new(std::sync::Mutex::new(
        Vec::<(Vec<EntityRow>, Vec<f32>)>::new(),
    ));
    let decisions_for_policy = Arc::clone(&decisions);
    let episode = crate::policy_runner::run_episode(
        config,
        seed,
        &crate::policy_runner::PolicyRunnerConfig {
            max_decisions_per_episode: rollout_config.max_decisions_per_episode,
            record_steps: true,
            max_stage: rollout_config.max_stage,
            reward_config: rollout_config.reward_config.clone(),
        },
        move |observation: &Observation, legal_actions: &[LegalAction]| {
            let rows = candidate_rows_for_legal_actions(observation, legal_actions);
            let logits = super::rollout::logits_for(
                model.as_ref(),
                device.as_ref(),
                observation,
                &observation_features(observation),
                &rows,
            );
            if !logits.iter().all(|logit| logit.is_finite()) {
                bail!("non-finite diagnostic logit for seed {seed}");
            }
            let index = logits
                .iter()
                .enumerate()
                .max_by(|left, right| left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0)))
                .map(|(index, _)| index)
                .expect("diagnostic logits must not be empty");
            decisions_for_policy
                .lock()
                .expect("diagnostic decision lock")
                .push((rows, logits));
            Ok(legal_actions[index].action.clone())
        },
    )?;
    let decisions = Arc::try_unwrap(decisions)
        .expect("diagnostic decisions have one owner")
        .into_inner()
        .expect("diagnostic decision lock");
    let policy_steps = episode
        .steps
        .as_ref()
        .expect("diagnostic trace records steps");
    if decisions.len() != policy_steps.len() {
        bail!("diagnostic decision and environment step counts differ");
    }
    let mut seen_states = BTreeMap::<String, usize>::new();
    let mut seen_fingerprints = BTreeMap::<String, usize>::new();
    let mut first_repeated_state_hash = None;
    let mut first_repeated_state_step = None;
    let mut first_repeated_state_period = None;
    let mut first_repeated_fingerprint_step = None;
    let mut steps = Vec::with_capacity(policy_steps.len());
    for (index, (policy_step, (candidate_rows, logits))) in
        policy_steps.iter().zip(decisions).enumerate()
    {
        let state_hash = policy_step.outcome.state_hash.clone();
        if first_repeated_state_step.is_none()
            && let Some(previous) = seen_states.insert(state_hash.clone(), index)
        {
            first_repeated_state_hash = Some(state_hash.clone());
            first_repeated_state_step = Some(index);
            first_repeated_state_period = Some(index - previous);
        }
        if first_repeated_fingerprint_step.is_none()
            && seen_fingerprints
                .insert(policy_step.post_progress_fingerprint.clone(), index)
                .is_some()
        {
            first_repeated_fingerprint_step = Some(index);
        }
        let selected_index = candidate_rows
            .iter()
            .enumerate()
            .find_map(|(candidate_index, _)| {
                (policy_step.legal_actions[candidate_index].action == policy_step.action)
                    .then_some(candidate_index)
            })
            .expect("selected diagnostic action must be legal");
        steps.push(DiagnosticStep {
            decision_point: policy_step.observation.decision_point.clone(),
            legal_action_ids: policy_step
                .legal_actions
                .iter()
                .map(|legal| legal.id.clone())
                .collect(),
            duplicate_candidate_row_groups: duplicate_candidate_row_groups(&candidate_rows),
            candidate_rows,
            logits,
            selected_index,
            selected_action_id: policy_step.action.action_id(),
            pre_progress_fingerprint: policy_step.pre_progress_fingerprint.clone(),
            post_progress_fingerprint: policy_step.post_progress_fingerprint.clone(),
            state_hash,
            reward: policy_step.outcome.reward.clone(),
            terminated: policy_step.outcome.terminated,
            truncated: policy_step.outcome.truncated,
        });
    }
    Ok(DiagnosticTrace {
        seed,
        steps,
        final_state_hash: episode.final_state_hash,
        final_stage: episode.final_observation.stage,
        episode_return: episode.episode_return,
        termination_reason: episode.termination_reason,
        first_repeated_state_hash,
        first_repeated_state_step,
        first_repeated_state_period,
        first_repeated_fingerprint_step,
    })
}

pub(crate) fn duplicate_candidate_row_groups(rows: &[EntityRow]) -> Vec<Vec<usize>> {
    let mut groups = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let Some(group) = groups
            .iter_mut()
            .find(|group: &&mut Vec<usize>| rows[group[0]] == *row)
        else {
            groups.push(vec![index]);
            continue;
        };
        group.push(index);
    }
    groups.retain(|group| group.len() > 1);
    groups
}
