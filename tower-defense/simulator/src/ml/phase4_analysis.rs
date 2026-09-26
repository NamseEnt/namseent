//! Teacher-corpus override/disagreement analysis (before any distillation).

use super::phase4_dataset::{DecisionSample, EpisodeRecord};
use super::semantic_bc::{LabelSource, SemanticPolicy, prepare_sample};
use crate::teacher_terminal_gate::{DISCOVERY_ROLLOUTS_PER_CANDIDATE, VALIDATION_ROLLOUTS_PER_ARM};
use anyhow::Result;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize)]
pub struct OverrideRow {
    pub opportunities: usize,
    pub overrides: usize,
    pub override_rate: f64,
    /// Mean validation paired delta of the selected candidate, over overrides.
    pub mean_override_validation_delta: Option<f64>,
    delta_sum: f64,
}

impl OverrideRow {
    fn add(&mut self, is_override: bool, delta: Option<f64>) {
        self.opportunities += 1;
        if is_override {
            self.overrides += 1;
            self.delta_sum += delta.unwrap_or(0.0);
        }
    }

    fn finish(&mut self) {
        self.override_rate = self.overrides as f64 / self.opportunities.max(1) as f64;
        self.mean_override_validation_delta =
            (self.overrides > 0).then(|| self.delta_sum / self.overrides as f64);
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DisagreementRow {
    pub decisions: usize,
    pub bc_differs_from_canonical: usize,
    pub bc_differs_rate: f64,
    pub teacher_overrides: usize,
    pub bc_matches_teacher_on_overrides: usize,
    pub bc_matches_canonical_on_overrides: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct TeacherCorpusAnalysis {
    pub episodes: usize,
    pub decisions: usize,
    pub forced_decisions: usize,
    pub overrides: usize,
    pub override_rate: f64,
    pub terminal_rollouts: usize,
    pub rollouts_per_decision: f64,
    pub teacher_seconds: f64,
    pub per_seed: Vec<SeedRow>,
    /// Keyed by the canonical action's kind: how often the teacher replaced it.
    pub by_canonical_kind: BTreeMap<String, OverrideRow>,
    /// Keyed by action kind: opportunities are decisions whose S4/1 proposal
    /// contained the kind, overrides are decisions where the teacher chose it.
    pub by_selected_kind: BTreeMap<String, OverrideRow>,
    pub override_pairs: BTreeMap<String, usize>,
    pub by_decision_point: BTreeMap<String, OverrideRow>,
    pub by_stage_bucket: BTreeMap<String, OverrideRow>,
    pub by_clear_rate_bucket: BTreeMap<String, OverrideRow>,
    /// Selected candidate's validation mean paired delta over overrides.
    pub override_delta_histogram: BTreeMap<String, usize>,
    pub override_p_holm_histogram: BTreeMap<String, usize>,
    /// Best finalist's validation mean delta where the teacher kept canonical.
    pub rejected_best_delta_histogram: BTreeMap<String, usize>,
    pub bc_disagreement: Option<BTreeMap<String, DisagreementRow>>,
}

#[derive(Debug, Serialize)]
pub struct SeedRow {
    pub seed: u64,
    pub teacher_terminal_clear_rate: f32,
    pub canonical_terminal_clear_rate: Option<f32>,
    pub paired_delta: Option<f32>,
    pub teacher_decisions: usize,
    pub overrides: usize,
    pub teacher_seconds: f64,
}

fn bucket(value: f64, edges: &[f64]) -> String {
    for window in edges.windows(2) {
        if value < window[1] {
            return format!("[{:+.1}, {:+.1})", window[0], window[1]);
        }
    }
    format!(">= {:+.1}", edges[edges.len() - 1])
}

fn stage_bucket(stage: usize) -> String {
    let start = (stage.saturating_sub(1) / 5) * 5 + 1;
    format!("stage {:02}-{:02}", start, start + 4)
}

fn clear_rate_bucket(clear_rate: f32) -> String {
    let start = ((clear_rate / 10.0).floor() as i32 * 10).clamp(0, 100);
    format!("clear_rate {:03}-{:03}", start, start + 10)
}

fn kind_of(action_id: &str) -> String {
    action_id.split(':').next().unwrap_or(action_id).to_string()
}

fn selected_delta(sample: &DecisionSample) -> Option<f64> {
    let label = sample.teacher.as_ref()?;
    label
        .validation
        .iter()
        .find(|stat| stat.action_id == label.teacher_action_id)
        .map(|stat| stat.mean_delta)
}

pub fn analyze_teacher_corpus(
    episodes: &[EpisodeRecord],
    bc_policy: Option<&SemanticPolicy>,
) -> Result<TeacherCorpusAnalysis> {
    let mut analysis = TeacherCorpusAnalysis {
        episodes: episodes.len(),
        ..TeacherCorpusAnalysis::default()
    };
    let delta_edges = [-100.0, 0.0, 0.5, 1.0, 2.0, 4.0, 8.0];
    let p_edges = [0.0, 0.001, 0.01, 0.02, 0.05];
    for episode in episodes {
        let mut seed_overrides = 0usize;
        let mut seed_seconds = 0.0;
        for sample in &episode.samples {
            let Some(label) = &sample.teacher else {
                continue;
            };
            analysis.decisions += 1;
            analysis.forced_decisions += label.forced as usize;
            seed_seconds += label.elapsed_seconds;
            if !label.forced {
                analysis.terminal_rollouts += label.proposal_action_ids.len()
                    * DISCOVERY_ROLLOUTS_PER_CANDIDATE
                    + (label.discovery_top3.len() + 1) * VALIDATION_ROLLOUTS_PER_ARM;
            }
            let is_override = label.teacher_override;
            seed_overrides += is_override as usize;
            let delta = selected_delta(sample);
            let canonical_kind = kind_of(&label.canonical_action_id);
            let selected_kind = kind_of(&label.teacher_action_id);
            analysis
                .by_canonical_kind
                .entry(canonical_kind.clone())
                .or_default()
                .add(is_override, delta);
            let mut proposed_kinds = label
                .proposal_action_ids
                .iter()
                .map(|id| kind_of(id))
                .collect::<Vec<_>>();
            proposed_kinds.sort();
            proposed_kinds.dedup();
            for kind in proposed_kinds {
                analysis
                    .by_selected_kind
                    .entry(kind.clone())
                    .or_default()
                    .add(is_override && kind == selected_kind, delta);
            }
            analysis
                .by_decision_point
                .entry(format!("{:?}", sample.decision_point))
                .or_default()
                .add(is_override, delta);
            analysis
                .by_stage_bucket
                .entry(stage_bucket(sample.stage))
                .or_default()
                .add(is_override, delta);
            analysis
                .by_clear_rate_bucket
                .entry(clear_rate_bucket(sample.clear_rate_before))
                .or_default()
                .add(is_override, delta);
            if is_override {
                *analysis
                    .override_pairs
                    .entry(format!("{canonical_kind} -> {selected_kind}"))
                    .or_insert(0) += 1;
                if let Some(delta) = delta {
                    *analysis
                        .override_delta_histogram
                        .entry(bucket(delta, &delta_edges))
                        .or_insert(0) += 1;
                }
                if let Some(stat) = label
                    .validation
                    .iter()
                    .find(|stat| stat.action_id == label.teacher_action_id)
                {
                    *analysis
                        .override_p_holm_histogram
                        .entry(bucket(stat.p_holm, &p_edges))
                        .or_insert(0) += 1;
                }
            } else if let Some(best) = label
                .validation
                .iter()
                .map(|stat| stat.mean_delta)
                .max_by(f64::total_cmp)
            {
                *analysis
                    .rejected_best_delta_histogram
                    .entry(bucket(best, &delta_edges))
                    .or_insert(0) += 1;
            }
        }
        analysis.overrides += seed_overrides;
        analysis.teacher_seconds += seed_seconds;
        let canonical = episode
            .canonical_reference
            .as_ref()
            .map(|reference| reference.terminal_clear_rate);
        analysis.per_seed.push(SeedRow {
            seed: episode.game_seed,
            teacher_terminal_clear_rate: episode.final_terminal_clear_rate,
            canonical_terminal_clear_rate: canonical,
            paired_delta: canonical.map(|canonical| episode.final_terminal_clear_rate - canonical),
            teacher_decisions: episode.decision_count,
            overrides: seed_overrides,
            teacher_seconds: seed_seconds,
        });
    }
    analysis.override_rate = analysis.overrides as f64 / analysis.decisions.max(1) as f64;
    analysis.rollouts_per_decision =
        analysis.terminal_rollouts as f64 / analysis.decisions.max(1) as f64;
    for row in analysis
        .by_canonical_kind
        .values_mut()
        .chain(analysis.by_selected_kind.values_mut())
        .chain(analysis.by_decision_point.values_mut())
        .chain(analysis.by_stage_bucket.values_mut())
        .chain(analysis.by_clear_rate_bucket.values_mut())
    {
        row.finish();
    }
    if let Some(policy) = bc_policy {
        let samples = episodes
            .iter()
            .flat_map(|episode| &episode.samples)
            .collect::<Vec<_>>();
        let predictions = samples
            .par_iter()
            .map(|sample| {
                let prepared = prepare_sample(sample, LabelSource::Chosen, 1.0);
                policy
                    .choose_encoded(&prepared.encoded)
                    .map(|(index, _)| index)
            })
            .collect::<Result<Vec<_>>>()?;
        let mut rows: BTreeMap<String, DisagreementRow> = BTreeMap::new();
        for (sample, prediction) in samples.iter().zip(predictions) {
            let is_override = sample.chosen_index != sample.canonical_index;
            for key in [
                "all".to_string(),
                format!("point {:?}", sample.decision_point),
                format!(
                    "canonical {}",
                    sample.candidates[sample.canonical_index].kind
                ),
            ] {
                let row = rows.entry(key).or_default();
                row.decisions += 1;
                row.bc_differs_from_canonical += (prediction != sample.canonical_index) as usize;
                if is_override {
                    row.teacher_overrides += 1;
                    row.bc_matches_teacher_on_overrides +=
                        (prediction == sample.chosen_index) as usize;
                    row.bc_matches_canonical_on_overrides +=
                        (prediction == sample.canonical_index) as usize;
                }
            }
        }
        for row in rows.values_mut() {
            row.bc_differs_rate =
                row.bc_differs_from_canonical as f64 / row.decisions.max(1) as f64;
        }
        analysis.bc_disagreement = Some(rows);
    }
    Ok(analysis)
}
