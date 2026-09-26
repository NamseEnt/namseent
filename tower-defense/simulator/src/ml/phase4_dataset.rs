//! Phase 4A decision dataset: frozen seed split, schema, provenance,
//! resumable per-seed storage, and the canonical / teacher generators.
//!
//! One gzip-compressed JSON file per game seed (`seed-XXXXXXXXXX.json.gz`)
//! holds the whole episode. A seed whose file exists is complete; files are
//! written to a temporary name and renamed, so an interrupted run leaves no
//! partial episode and can simply be restarted.

use super::neural_checkpoint::current_git_revision;
use super::semantic_candidates::{
    POLICY_CANDIDATE_SET_VERSION, PolicyCandidate, PolicyCandidates,
    SEMANTIC_CANDIDATE_ENCODER_VERSION, policy_candidates,
};
use crate::config::{GAME_CONFIG_VERSION, GameConfig, config_digest};
use crate::environment::{AgentAction, DecisionPoint, GameEnvironment, Observation};
use crate::ml::contract::{
    ACTION_WIRE_SCHEMA_VERSION, CATALOG_SCHEMA_VERSION, OBSERVATION_SCHEMA_VERSION,
};
use crate::policy_action::PolicyActionSpace;
use crate::teacher_selection::{
    CandidateValidationStat, TEACHER_SELECTION_SCHEMA_VERSION, TeacherSelectionPools,
    select_teacher_action_with_raw_outcomes,
};
use anyhow::{Context, Result, bail};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

pub const PHASE4_DATASET_SCHEMA_VERSION: u32 = 1;
/// Runaway guard: reaching it is an invariant failure, never a result.
pub const MAX_EPISODE_DECISIONS: usize = 512;
/// Game seeds used by the Phase 3 pilot, held-out, post-hoc extension and
/// terminal gate. Never used for Phase 4 training or tuning.
pub const PHASE3_RESERVED_GAME_SEEDS: std::ops::RangeInclusive<u64> = 108..=131;
pub const CLEAR_RATE_TOLERANCE: f64 = 1e-3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Phase4Split {
    CanonicalTrain,
    CanonicalValidation,
    TeacherTrain,
    DevelopmentEvaluation,
    FinalEvaluation,
}

impl Phase4Split {
    pub const ALL: [Self; 5] = [
        Self::CanonicalTrain,
        Self::CanonicalValidation,
        Self::TeacherTrain,
        Self::DevelopmentEvaluation,
        Self::FinalEvaluation,
    ];

    /// Frozen before any Phase 4A data was generated (see
    /// docs/game-ai/13-phase4a-bc-distillation.md).
    pub fn range(self) -> std::ops::RangeInclusive<u64> {
        match self {
            Self::CanonicalTrain => 2_000_000..=2_009_999,
            Self::CanonicalValidation => 2_100_000..=2_100_063,
            Self::TeacherTrain => 2_200_000..=2_200_015,
            Self::DevelopmentEvaluation => 2_300_000..=2_300_127,
            Self::FinalEvaluation => 2_400_000..=2_400_255,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::CanonicalTrain => "canonical_train",
            Self::CanonicalValidation => "canonical_validation",
            Self::TeacherTrain => "teacher_train",
            Self::DevelopmentEvaluation => "development_evaluation",
            Self::FinalEvaluation => "final_evaluation",
        }
    }

    /// The first `count` seeds of this split, or the whole split for `None`.
    pub fn seeds(self, count: Option<usize>) -> Result<Vec<u64>> {
        let range = self.range();
        let size = (range.end() - range.start() + 1) as usize;
        let count = count.unwrap_or(size);
        if count == 0 || count > size {
            bail!("{} has {size} seeds; requested {count}", self.name());
        }
        Ok((*range.start()..).take(count).collect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePolicy {
    Canonical,
    Teacher,
    LearnedPolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetProvenance {
    pub git_commit: String,
    pub git_dirty_paths: Vec<String>,
    pub dataset_schema_version: u32,
    pub observation_schema_version: u32,
    pub catalog_schema_version: u32,
    pub action_wire_schema_version: u32,
    pub environment_action_schema_version: u32,
    pub policy_candidate_set_version: u32,
    pub candidate_encoder_version: u32,
    pub game_config_version: u32,
    pub game_config_digest: String,
    pub source_policy: SourcePolicy,
    pub split: Phase4Split,
    pub split_range: (u64, u64),
    pub max_episode_decisions: usize,
    pub teacher_rule_version: Option<u32>,
    pub teacher_pools: Option<TeacherSelectionPools>,
}

impl DatasetProvenance {
    pub fn new(
        config: &GameConfig,
        source_policy: SourcePolicy,
        split: Phase4Split,
        teacher_pools: Option<TeacherSelectionPools>,
    ) -> Result<Self> {
        let range = split.range();
        Ok(Self {
            git_commit: current_git_revision()?,
            git_dirty_paths: git_dirty_paths(),
            dataset_schema_version: PHASE4_DATASET_SCHEMA_VERSION,
            observation_schema_version: OBSERVATION_SCHEMA_VERSION,
            catalog_schema_version: CATALOG_SCHEMA_VERSION,
            action_wire_schema_version: ACTION_WIRE_SCHEMA_VERSION,
            environment_action_schema_version: crate::environment::ACTION_SCHEMA_VERSION,
            policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
            candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
            game_config_version: GAME_CONFIG_VERSION,
            game_config_digest: config_digest(config),
            source_policy,
            split,
            split_range: (*range.start(), *range.end()),
            max_episode_decisions: MAX_EPISODE_DECISIONS,
            teacher_rule_version: teacher_pools
                .as_ref()
                .map(|_| TEACHER_SELECTION_SCHEMA_VERSION),
            teacher_pools,
        })
    }

    /// Everything that must agree for two episodes to be mixed in one
    /// dataset (the git commit and dirty paths may differ between shards).
    pub fn compatibility_key(&self) -> String {
        serde_json::to_string(&(
            self.dataset_schema_version,
            self.observation_schema_version,
            self.catalog_schema_version,
            self.action_wire_schema_version,
            self.environment_action_schema_version,
            self.policy_candidate_set_version,
            self.game_config_version,
            &self.game_config_digest,
            self.source_policy,
            self.split,
            self.teacher_rule_version,
            &self.teacher_pools,
        ))
        .expect("provenance key serializes")
    }

    pub fn check_current(&self, config: &GameConfig) -> Result<()> {
        if self.dataset_schema_version != PHASE4_DATASET_SCHEMA_VERSION
            || self.observation_schema_version != OBSERVATION_SCHEMA_VERSION
            || self.catalog_schema_version != CATALOG_SCHEMA_VERSION
            || self.action_wire_schema_version != ACTION_WIRE_SCHEMA_VERSION
            || self.environment_action_schema_version != crate::environment::ACTION_SCHEMA_VERSION
            || self.policy_candidate_set_version != POLICY_CANDIDATE_SET_VERSION
            || self.game_config_version != GAME_CONFIG_VERSION
            || self.game_config_digest != config_digest(config)
        {
            bail!("dataset provenance does not match the current schema/config contract");
        }
        Ok(())
    }
}

fn git_dirty_paths() -> Vec<String> {
    std::process::Command::new("git")
        .args([
            "-C",
            concat!(env!("CARGO_MANIFEST_DIR"), "/.."),
            "status",
            "--porcelain",
            "--",
            "core",
            "simulator/src",
            "simulator/Cargo.toml",
            "gameconfig.jsonc",
        ])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|text| text.lines().map(str::to_string).collect())
        .unwrap_or_else(|| vec!["<git status failed>".to_string()])
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CandidateRecord {
    pub action: AgentAction,
    pub action_id: String,
    pub kind: String,
    /// Index in `PolicyActionSpace` (the canonical dense policy-action index).
    pub policy_action_index: usize,
    pub family_rank: Option<usize>,
}

impl CandidateRecord {
    pub fn policy_candidate(&self) -> PolicyCandidate {
        PolicyCandidate {
            action: self.action.clone(),
            action_id: self.action_id.clone(),
            family_rank: self.family_rank,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeacherLabel {
    pub teacher_rule_version: u32,
    pub canonical_action_id: String,
    pub teacher_action_id: String,
    pub teacher_override: bool,
    /// S4/1 proposal was empty: no rollouts, baseline returned directly.
    pub forced: bool,
    pub proposal_action_ids: Vec<String>,
    pub reroll_proposal_scores: Vec<(String, f32)>,
    /// `[proposal candidate][discovery scenario]` terminal clear_rate, in
    /// `teacher_pools.discovery_seeds` order.
    pub discovery_outcomes: Vec<Vec<f32>>,
    pub discovery_means: Vec<(String, f32)>,
    pub discovery_top3: Vec<String>,
    /// Baseline terminal clear_rate per validation scenario.
    pub validation_baseline_outcomes: Vec<f32>,
    /// `[finalist][validation scenario]` terminal clear_rate, finalists in
    /// `discovery_top3` order.
    pub validation_candidate_outcomes: Vec<Vec<f32>>,
    /// Paired deltas, t statistics, p-values and the Holm result.
    pub validation: Vec<CandidateValidationStat>,
    pub elapsed_seconds: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecisionSample {
    pub game_seed: u64,
    pub decision_index: usize,
    pub state_hash: String,
    pub decision_point: DecisionPoint,
    pub stage: usize,
    pub observation: Observation,
    pub candidates: Vec<CandidateRecord>,
    /// Aligned with `candidates`; every stored candidate is legal.
    pub legal_mask: Vec<bool>,
    /// Size and legal count of the full `PolicyActionSpace` for this state.
    pub policy_action_count: usize,
    pub policy_legal_count: usize,
    pub chosen_index: usize,
    pub chosen_action_id: String,
    pub chosen_policy_action_index: usize,
    pub action_kind: String,
    pub canonical_index: usize,
    pub canonical_action_id: String,
    pub clear_rate_before: f32,
    pub clear_rate_after: f32,
    pub delta_clear_rate: f32,
    pub terminated: bool,
    pub final_terminal_clear_rate: f32,
    pub victory: bool,
    pub source_policy: SourcePolicy,
    pub teacher: Option<TeacherLabel>,
}

impl DecisionSample {
    pub fn policy_candidates(&self) -> Vec<PolicyCandidate> {
        self.candidates
            .iter()
            .map(CandidateRecord::policy_candidate)
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanonicalReference {
    pub terminal_clear_rate: f32,
    pub final_stage: usize,
    pub decision_count: usize,
    pub victory: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpisodeRecord {
    pub provenance: DatasetProvenance,
    pub game_seed: u64,
    pub source_policy: SourcePolicy,
    pub initial_clear_rate: f32,
    pub final_terminal_clear_rate: f32,
    pub victory: bool,
    pub final_stage: usize,
    pub decision_count: usize,
    pub final_state_hash: String,
    pub elapsed_seconds: f64,
    /// Canonical baseline on the same seed (teacher episodes only).
    pub canonical_reference: Option<CanonicalReference>,
    pub samples: Vec<DecisionSample>,
}

/// Applies `action` and settles forced `Continue`s, the same transition as
/// `semantic_step` + `settle_forced_actions`, without the policy trace.
/// Returns whether the episode reached the terminal state.
pub(crate) fn step_to_next_decision(
    environment: &mut GameEnvironment,
    action: AgentAction,
) -> Result<bool> {
    let mut outcome = environment
        .rollout_step_trusted(action)
        .map_err(|error| anyhow::anyhow!("semantic step failed: {error:?}"))?;
    loop {
        if outcome.truncated {
            bail!("episode hit the environment tick limit - invariant failure");
        }
        if outcome.terminated || matches!(environment.decision_point(), DecisionPoint::Terminal) {
            return Ok(true);
        }
        match environment.forced_action() {
            Some(forced) => {
                outcome = environment
                    .rollout_step_trusted(forced)
                    .map_err(|error| anyhow::anyhow!("forced step failed: {error:?}"))?;
            }
            None => return Ok(false),
        }
    }
}

struct DecisionContext {
    state_hash: String,
    candidates: PolicyCandidates,
    records: Vec<CandidateRecord>,
    legal_mask: Vec<bool>,
    policy_action_count: usize,
    policy_legal_count: usize,
    canonical_index: usize,
}

fn decision_context(environment: &GameEnvironment) -> Result<DecisionContext> {
    let state_hash = environment.state_hash();
    let candidates = policy_candidates(environment)?;
    let space = PolicyActionSpace::compute(environment);
    let mut records = Vec::with_capacity(candidates.candidates.len());
    let mut legal_mask = Vec::with_capacity(candidates.candidates.len());
    for candidate in &candidates.candidates {
        let legal = environment.semantic_action_is_legal(&candidate.action);
        if !legal {
            bail!(
                "candidate {} is not legal at state {state_hash}",
                candidate.action_id
            );
        }
        let policy_action_index = space.action_to_index(&candidate.action).with_context(|| {
            format!(
                "candidate {} has no legal PolicyActionSpace index at state {state_hash}",
                candidate.action_id
            )
        })?;
        legal_mask.push(legal);
        records.push(CandidateRecord {
            action: candidate.action.clone(),
            action_id: candidate.action_id.clone(),
            kind: candidate.action.kind().wire_name().to_string(),
            policy_action_index,
            family_rank: candidate.family_rank,
        });
    }
    let canonical_index = candidates.canonical_index().with_context(|| {
        format!(
            "canonical action {} is missing from the policy candidate set at state {state_hash}",
            candidates.canonical_action.action_id()
        )
    })?;
    Ok(DecisionContext {
        state_hash,
        policy_action_count: space.action_count(),
        policy_legal_count: space.legal_mask().iter().filter(|legal| **legal).count(),
        candidates,
        records,
        legal_mask,
        canonical_index,
    })
}

/// Plays one episode to the actual terminal state, choosing each action with
/// `choose` (which returns the index into the policy candidate set plus an
/// optional teacher label), and records every decision.
fn run_recorded_episode<F>(
    config: Arc<GameConfig>,
    provenance: DatasetProvenance,
    game_seed: u64,
    source_policy: SourcePolicy,
    mut choose: F,
) -> Result<EpisodeRecord>
where
    F: FnMut(&GameEnvironment, &DecisionContext) -> Result<(usize, Option<TeacherLabel>)>,
{
    let started = Instant::now();
    let mut environment = GameEnvironment::new(config, game_seed);
    let initial_clear_rate = environment.clear_rate();
    let mut samples: Vec<DecisionSample> = Vec::new();
    while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
        if samples.len() >= MAX_EPISODE_DECISIONS {
            bail!(
                "seed {game_seed}: episode reached the {MAX_EPISODE_DECISIONS}-decision safety cap \
                 - invariant failure"
            );
        }
        let context = decision_context(&environment)?;
        let (chosen_index, teacher) = choose(&environment, &context)?;
        let chosen = context
            .records
            .get(chosen_index)
            .with_context(|| format!("seed {game_seed}: chosen index out of range"))?
            .clone();
        let clear_rate_before = environment.clear_rate();
        let observation = context.candidates.observation.clone();
        let terminated = step_to_next_decision(&mut environment, chosen.action.clone())?;
        let clear_rate_after = environment.clear_rate();
        samples.push(DecisionSample {
            game_seed,
            decision_index: samples.len(),
            state_hash: context.state_hash,
            decision_point: observation.decision_point.clone(),
            stage: observation.stage,
            observation,
            candidates: context.records.clone(),
            legal_mask: context.legal_mask,
            policy_action_count: context.policy_action_count,
            policy_legal_count: context.policy_legal_count,
            chosen_index,
            chosen_action_id: chosen.action_id.clone(),
            chosen_policy_action_index: chosen.policy_action_index,
            action_kind: chosen.kind.clone(),
            canonical_index: context.canonical_index,
            canonical_action_id: context.records[context.canonical_index].action_id.clone(),
            clear_rate_before,
            clear_rate_after,
            delta_clear_rate: clear_rate_after - clear_rate_before,
            terminated,
            final_terminal_clear_rate: 0.0,
            victory: false,
            source_policy,
            teacher,
        });
        if terminated {
            break;
        }
    }
    let final_terminal_clear_rate = environment.clear_rate();
    let victory = final_terminal_clear_rate >= 100.0;
    for sample in &mut samples {
        sample.final_terminal_clear_rate = final_terminal_clear_rate;
        sample.victory = victory;
    }
    Ok(EpisodeRecord {
        provenance,
        game_seed,
        source_policy,
        initial_clear_rate,
        final_terminal_clear_rate,
        victory,
        final_stage: environment.snapshot().stage,
        decision_count: samples.len(),
        final_state_hash: environment.state_hash(),
        elapsed_seconds: started.elapsed().as_secs_f64(),
        canonical_reference: None,
        samples,
    })
}

pub fn collect_canonical_episode(
    config: Arc<GameConfig>,
    provenance: DatasetProvenance,
    game_seed: u64,
) -> Result<EpisodeRecord> {
    run_recorded_episode(
        config,
        provenance,
        game_seed,
        SourcePolicy::Canonical,
        |_, context| Ok((context.canonical_index, None)),
    )
}

pub fn collect_teacher_episode(
    config: Arc<GameConfig>,
    provenance: DatasetProvenance,
    game_seed: u64,
    pools: &TeacherSelectionPools,
) -> Result<EpisodeRecord> {
    let reference = collect_canonical_episode(Arc::clone(&config), provenance.clone(), game_seed)?;
    let mut record = run_recorded_episode(
        config,
        provenance,
        game_seed,
        SourcePolicy::Teacher,
        |environment, context| {
            let started = Instant::now();
            let (decision, action, raw) =
                select_teacher_action_with_raw_outcomes(environment, pools)?;
            let elapsed_seconds = started.elapsed().as_secs_f64();
            if decision.baseline_action_id != context.records[context.canonical_index].action_id {
                bail!("teacher baseline differs from the canonical action");
            }
            let chosen_index = context
                .candidates
                .index_of_action_id(&action.action_id())
                .with_context(|| {
                    format!(
                        "teacher action {} is not in the policy candidate set",
                        action.action_id()
                    )
                })?;
            eprintln!(
                "  seed {} decision [{:?}] proposal={} selected={}{} ({:.1}s)",
                environment.seed(),
                environment.decision_point(),
                decision.proposal_action_ids.len(),
                decision.selected_action_id,
                if decision.selected_action_id == decision.baseline_action_id {
                    ""
                } else {
                    " (override)"
                },
                elapsed_seconds
            );
            Ok((
                chosen_index,
                Some(TeacherLabel {
                    teacher_rule_version: decision.schema_version,
                    canonical_action_id: decision.baseline_action_id.clone(),
                    teacher_override: decision.selected_action_id != decision.baseline_action_id,
                    teacher_action_id: decision.selected_action_id,
                    forced: decision.forced,
                    proposal_action_ids: decision.proposal_action_ids,
                    reroll_proposal_scores: raw.reroll_proposal_scores,
                    discovery_outcomes: raw.discovery_outcomes,
                    discovery_means: decision.discovery,
                    discovery_top3: decision.discovery_top3,
                    validation_baseline_outcomes: raw.validation_baseline_outcomes,
                    validation_candidate_outcomes: raw.validation_candidate_outcomes,
                    validation: decision.validation,
                    elapsed_seconds,
                }),
            ))
        },
    )?;
    record.canonical_reference = Some(CanonicalReference {
        terminal_clear_rate: reference.final_terminal_clear_rate,
        final_stage: reference.final_stage,
        decision_count: reference.decision_count,
        victory: reference.victory,
    });
    Ok(record)
}

pub fn episode_file_name(game_seed: u64) -> String {
    format!("seed-{game_seed:010}.json.gz")
}

pub fn write_episode(directory: &Path, record: &EpisodeRecord) -> Result<PathBuf> {
    std::fs::create_dir_all(directory)?;
    let path = directory.join(episode_file_name(record.game_seed));
    let temporary = directory.join(format!(
        ".{}.tmp-{}",
        episode_file_name(record.game_seed),
        std::process::id()
    ));
    {
        let file = std::fs::File::create(&temporary)?;
        let mut encoder = GzEncoder::new(std::io::BufWriter::new(file), Compression::new(6));
        serde_json::to_writer(&mut encoder, record)?;
        encoder.finish()?.flush()?;
    }
    std::fs::rename(&temporary, &path)?;
    Ok(path)
}

pub fn read_episode(path: &Path) -> Result<EpisodeRecord> {
    let file = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut json = Vec::new();
    GzDecoder::new(std::io::BufReader::new(file))
        .read_to_end(&mut json)
        .with_context(|| format!("decompress {}", path.display()))?;
    serde_json::from_slice(&json).with_context(|| format!("parse {}", path.display()))
}

pub fn episode_paths(directory: &Path) -> Result<Vec<(u64, PathBuf)>> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(directory)
        .with_context(|| format!("read dataset directory {}", directory.display()))?
    {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(seed) = name
            .strip_prefix("seed-")
            .and_then(|rest| rest.strip_suffix(".json.gz"))
            .and_then(|seed| seed.parse::<u64>().ok())
        else {
            continue;
        };
        paths.push((seed, path));
    }
    paths.sort();
    Ok(paths)
}

/// Loads every episode of a dataset directory, sorted by seed, rejecting
/// mixed provenance and failing validation.
pub fn load_episodes(directory: &Path, config: &GameConfig) -> Result<Vec<EpisodeRecord>> {
    let episodes = episode_paths(directory)?
        .par_iter()
        .map(|(_, path)| read_episode(path))
        .collect::<Result<Vec<_>>>()?;
    if let Some(first) = episodes.first() {
        first.provenance.check_current(config)?;
        let key = first.provenance.compatibility_key();
        for episode in &episodes {
            if episode.provenance.compatibility_key() != key {
                bail!(
                    "{}: seed {} has incompatible provenance",
                    directory.display(),
                    episode.game_seed
                );
            }
            validate_episode(episode)?;
        }
    }
    Ok(episodes)
}

pub fn validate_episode(episode: &EpisodeRecord) -> Result<()> {
    let seed = episode.game_seed;
    let range = episode.provenance.split.range();
    if !range.contains(&seed) {
        bail!(
            "seed {seed} is outside its split {:?}",
            episode.provenance.split
        );
    }
    if PHASE3_RESERVED_GAME_SEEDS.contains(&seed) {
        bail!("seed {seed} is a reserved Phase 3 seed");
    }
    if episode.decision_count != episode.samples.len() || episode.samples.is_empty() {
        bail!("seed {seed}: decision count mismatch");
    }
    if !episode
        .samples
        .last()
        .is_some_and(|sample| sample.terminated)
    {
        bail!("seed {seed}: episode did not reach the terminal state");
    }
    let mut expected_before = episode.initial_clear_rate;
    let mut delta_sum = 0.0f64;
    for (index, sample) in episode.samples.iter().enumerate() {
        if sample.decision_index != index || sample.game_seed != seed {
            bail!("seed {seed}: decision {index} index/seed mismatch");
        }
        if sample.legal_mask.len() != sample.candidates.len()
            || sample.chosen_index >= sample.candidates.len()
            || sample.canonical_index >= sample.candidates.len()
        {
            bail!("seed {seed} decision {index}: candidate alignment mismatch");
        }
        if !sample.legal_mask[sample.chosen_index] {
            bail!("seed {seed} decision {index}: chosen action is masked illegal");
        }
        let chosen = &sample.candidates[sample.chosen_index];
        if chosen.action_id != sample.chosen_action_id
            || chosen.policy_action_index != sample.chosen_policy_action_index
            || sample.candidates[sample.canonical_index].action_id != sample.canonical_action_id
        {
            bail!("seed {seed} decision {index}: chosen/canonical id mismatch");
        }
        let mut indices = sample
            .candidates
            .iter()
            .map(|candidate| candidate.policy_action_index)
            .collect::<Vec<_>>();
        indices.sort_unstable();
        indices.dedup();
        if indices.len() != sample.candidates.len()
            || indices
                .last()
                .is_some_and(|last| *last >= sample.policy_action_count)
        {
            bail!("seed {seed} decision {index}: policy action indices are not unique/in range");
        }
        if (sample.clear_rate_before - expected_before).abs() as f64 > CLEAR_RATE_TOLERANCE {
            bail!(
                "seed {seed} decision {index}: clear_rate_before {} != previous clear_rate_after {}",
                sample.clear_rate_before,
                expected_before
            );
        }
        if sample.terminated != (index + 1 == episode.samples.len()) {
            bail!("seed {seed} decision {index}: terminated flag mismatch");
        }
        match (&sample.teacher, sample.source_policy) {
            (Some(label), SourcePolicy::Teacher) => {
                if label.teacher_action_id != sample.chosen_action_id
                    || label.canonical_action_id != sample.canonical_action_id
                    || label.teacher_override != (sample.chosen_index != sample.canonical_index)
                {
                    bail!("seed {seed} decision {index}: teacher label mismatch");
                }
            }
            (None, SourcePolicy::Teacher) => bail!("seed {seed}: teacher sample without label"),
            _ => {}
        }
        if sample.source_policy == SourcePolicy::Canonical
            && sample.chosen_index != sample.canonical_index
        {
            bail!("seed {seed} decision {index}: canonical sample did not choose canonical");
        }
        delta_sum += sample.delta_clear_rate as f64;
        expected_before = sample.clear_rate_after;
    }
    let telescoped = (episode.final_terminal_clear_rate - episode.initial_clear_rate) as f64;
    if (delta_sum - telescoped).abs() > CLEAR_RATE_TOLERANCE
        || (expected_before - episode.final_terminal_clear_rate).abs() as f64 > CLEAR_RATE_TOLERANCE
    {
        bail!("seed {seed}: clear_rate deltas do not telescope ({delta_sum} vs {telescoped})");
    }
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct CollectionSummary {
    pub directory: String,
    pub split: Phase4Split,
    pub source_policy: SourcePolicy,
    pub requested_seeds: usize,
    pub skipped_existing: usize,
    pub generated: usize,
    pub elapsed_seconds: f64,
}

/// Generates every missing episode in `seeds` into `directory`. Existing
/// complete files are validated and skipped; `shard` = `(index, count)`
/// keeps only seeds whose offset in `seeds` is `index` modulo `count`.
pub fn collect_into_directory(
    config: Arc<GameConfig>,
    directory: &Path,
    split: Phase4Split,
    seeds: &[u64],
    source_policy: SourcePolicy,
    shard: (usize, usize),
    teacher_pools: Option<TeacherSelectionPools>,
) -> Result<CollectionSummary> {
    let started = Instant::now();
    let (shard_index, shard_count) = shard;
    if shard_count == 0 || shard_index >= shard_count {
        bail!("invalid shard {shard_index}/{shard_count}");
    }
    let range = split.range();
    for seed in seeds {
        if !range.contains(seed) || PHASE3_RESERVED_GAME_SEEDS.contains(seed) {
            bail!("seed {seed} is not in split {}", split.name());
        }
    }
    let mut unique = seeds.to_vec();
    unique.sort_unstable();
    unique.dedup();
    if unique.len() != seeds.len() {
        bail!("duplicate seeds requested");
    }
    std::fs::create_dir_all(directory)?;
    let provenance = DatasetProvenance::new(&config, source_policy, split, teacher_pools.clone())?;
    let existing = episode_paths(directory)?
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    let mut skipped = 0usize;
    let mut pending = Vec::new();
    for (offset, seed) in seeds.iter().enumerate() {
        if offset % shard_count != shard_index {
            continue;
        }
        if let Some(path) = existing.get(seed) {
            let episode = read_episode(path)?;
            if episode.provenance.compatibility_key() != provenance.compatibility_key() {
                bail!("{} has incompatible provenance", path.display());
            }
            validate_episode(&episode)?;
            skipped += 1;
        } else {
            pending.push(*seed);
        }
    }
    let generate = |seed: u64| -> Result<()> {
        let episode = match source_policy {
            SourcePolicy::Canonical => {
                collect_canonical_episode(Arc::clone(&config), provenance.clone(), seed)?
            }
            SourcePolicy::Teacher => collect_teacher_episode(
                Arc::clone(&config),
                provenance.clone(),
                seed,
                teacher_pools
                    .as_ref()
                    .context("teacher collection requires teacher pools")?,
            )?,
            SourcePolicy::LearnedPolicy => bail!("learned-policy collection is not a generator"),
        };
        validate_episode(&episode)?;
        write_episode(directory, &episode)?;
        eprintln!(
            "seed {seed}: {} decisions, terminal clear_rate {:.2}, {:.1}s",
            episode.decision_count, episode.final_terminal_clear_rate, episode.elapsed_seconds
        );
        Ok(())
    };
    match source_policy {
        SourcePolicy::Teacher => pending.iter().try_for_each(|seed| generate(*seed))?,
        _ => pending.par_iter().try_for_each(|seed| generate(*seed))?,
    }
    Ok(CollectionSummary {
        directory: directory.display().to_string(),
        split,
        source_policy,
        requested_seeds: seeds.len(),
        skipped_existing: skipped,
        generated: pending.len(),
        elapsed_seconds: started.elapsed().as_secs_f64(),
    })
}

/// Copies every episode of `inputs` into `output`, refusing duplicate seeds
/// whose content differs and mixed provenance.
pub fn merge_directories(inputs: &[PathBuf], output: &Path) -> Result<usize> {
    std::fs::create_dir_all(output)?;
    let mut key: Option<String> = None;
    let mut merged = 0usize;
    for input in inputs {
        for (seed, path) in episode_paths(input)? {
            let episode = read_episode(&path)?;
            validate_episode(&episode)?;
            let episode_key = episode.provenance.compatibility_key();
            if key.get_or_insert_with(|| episode_key.clone()) != &episode_key {
                bail!("{}: incompatible provenance", path.display());
            }
            let target = output.join(episode_file_name(seed));
            if target.exists() {
                if std::fs::read(&target)? != std::fs::read(&path)? {
                    let existing = read_episode(&target)?;
                    if existing.samples != episode.samples {
                        bail!("seed {seed}: duplicate with different content");
                    }
                }
                continue;
            }
            std::fs::copy(&path, &target)?;
            merged += 1;
        }
    }
    Ok(merged)
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TransitionDeltaStats {
    pub count: usize,
    pub negative_count: usize,
    pub zero_count: usize,
    pub min: f32,
    pub max: f32,
    pub sum: f64,
}

#[derive(Debug, Default, Serialize)]
pub struct RewardInvariantReport {
    pub episodes: usize,
    pub decisions: usize,
    pub max_telescoping_error: f64,
    pub max_boundary_error: f64,
    pub negative_delta_decisions: usize,
    /// Keyed by `"<decision point> -> <next decision point>"`.
    pub by_transition: BTreeMap<String, TransitionDeltaStats>,
    pub by_action_kind: BTreeMap<String, TransitionDeltaStats>,
}

fn add_delta(stats: &mut TransitionDeltaStats, delta: f32) {
    if stats.count == 0 {
        stats.min = delta;
        stats.max = delta;
    }
    stats.count += 1;
    stats.negative_count += (delta < 0.0) as usize;
    stats.zero_count += (delta == 0.0) as usize;
    stats.min = stats.min.min(delta);
    stats.max = stats.max.max(delta);
    stats.sum += delta as f64;
}

/// Potential-based progress reward `r_t = clear_rate(s_{t+1}) - clear_rate(s_t)`
/// checked on recorded trajectories: gamma = 1 telescoping and per-transition
/// jumps/decreases.
pub fn reward_invariant_report(episodes: &[EpisodeRecord]) -> RewardInvariantReport {
    let mut report = RewardInvariantReport {
        episodes: episodes.len(),
        ..RewardInvariantReport::default()
    };
    for episode in episodes {
        let mut sum = 0.0f64;
        let mut previous_after = episode.initial_clear_rate;
        for (index, sample) in episode.samples.iter().enumerate() {
            report.decisions += 1;
            sum += sample.delta_clear_rate as f64;
            report.max_boundary_error = report
                .max_boundary_error
                .max((sample.clear_rate_before - previous_after).abs() as f64);
            previous_after = sample.clear_rate_after;
            let next_point = episode
                .samples
                .get(index + 1)
                .map_or("Terminal".to_string(), |next| {
                    format!("{:?}", next.decision_point)
                });
            let key = format!("{:?} -> {next_point}", sample.decision_point);
            add_delta(
                report.by_transition.entry(key).or_default(),
                sample.delta_clear_rate,
            );
            add_delta(
                report
                    .by_action_kind
                    .entry(sample.action_kind.clone())
                    .or_default(),
                sample.delta_clear_rate,
            );
            report.negative_delta_decisions += (sample.delta_clear_rate < 0.0) as usize;
        }
        let telescoped = (episode.final_terminal_clear_rate - episode.initial_clear_rate) as f64;
        report.max_telescoping_error = report.max_telescoping_error.max((sum - telescoped).abs());
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::teacher_terminal_gate::run_canonical_terminal_episode;

    fn config() -> Arc<GameConfig> {
        Arc::new(GameConfig::default_config())
    }

    fn provenance(split: Phase4Split) -> DatasetProvenance {
        DatasetProvenance::new(&config(), SourcePolicy::Canonical, split, None).expect("provenance")
    }

    #[test]
    fn frozen_splits_are_disjoint_and_exclude_phase3_seeds() {
        for (index, left) in Phase4Split::ALL.iter().enumerate() {
            let left_range = left.range();
            assert!(*left_range.start() > 1_000);
            assert!(!left_range.contains(PHASE3_RESERVED_GAME_SEEDS.start()));
            assert!(!left_range.contains(PHASE3_RESERVED_GAME_SEEDS.end()));
            for right in &Phase4Split::ALL[index + 1..] {
                let right_range = right.range();
                assert!(
                    left_range.end() < right_range.start()
                        || right_range.end() < left_range.start()
                );
            }
        }
    }

    #[test]
    fn canonical_episode_matches_terminal_gate_baseline_and_telescopes() {
        for seed in [0u64, 3] {
            let mut episode =
                collect_canonical_episode(config(), provenance(Phase4Split::CanonicalTrain), seed)
                    .expect("episode");
            let reference = run_canonical_terminal_episode(config(), seed).expect("reference");
            assert_eq!(episode.decision_count, reference.decision_count);
            assert_eq!(episode.final_terminal_clear_rate, reference.clear_rate);
            assert_eq!(episode.final_state_hash, reference.final_state_hash);
            assert_eq!(episode.final_stage, reference.final_stage);
            for sample in &episode.samples {
                assert!(sample.legal_mask[sample.chosen_index]);
                assert_eq!(sample.chosen_index, sample.canonical_index);
            }
            let report = reward_invariant_report(std::slice::from_ref(&episode));
            assert!(report.max_telescoping_error <= CLEAR_RATE_TOLERANCE);
            assert!(report.max_boundary_error <= CLEAR_RATE_TOLERANCE);
            episode.game_seed = 2_000_000 + seed;
            for sample in &mut episode.samples {
                sample.game_seed = episode.game_seed;
            }
            validate_episode(&episode).expect("valid");
        }
    }

    #[test]
    fn episode_roundtrip_preserves_samples_and_provenance() {
        let mut episode =
            collect_canonical_episode(config(), provenance(Phase4Split::CanonicalTrain), 1)
                .expect("episode");
        episode.game_seed = 2_000_001;
        for sample in &mut episode.samples {
            sample.game_seed = episode.game_seed;
        }
        let directory =
            std::env::temp_dir().join(format!("phase4-roundtrip-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        write_episode(&directory, &episode).expect("write");
        let read = read_episode(&directory.join(episode_file_name(2_000_001))).expect("read");
        assert_eq!(read, episode);
        assert_eq!(read.provenance, episode.provenance);
        let loaded = load_episodes(&directory, &config()).expect("load");
        assert_eq!(loaded.len(), 1);
        std::fs::remove_dir_all(&directory).expect("cleanup");
    }

    #[test]
    fn stored_candidate_indices_roundtrip_through_policy_action_space() {
        let config = config();
        let mut environment = GameEnvironment::new(Arc::clone(&config), 5);
        for _ in 0..25 {
            if matches!(environment.decision_point(), DecisionPoint::Terminal) {
                break;
            }
            let context = decision_context(&environment).expect("context");
            let space = PolicyActionSpace::compute(&environment);
            for record in &context.records {
                let action = space
                    .index_to_action(record.policy_action_index)
                    .expect("index maps back to an action");
                assert_eq!(
                    space.action_to_index(&action),
                    Some(record.policy_action_index)
                );
                assert_eq!(
                    space.action_to_index(&record.action),
                    Some(record.policy_action_index)
                );
                assert!(space.legal_mask()[record.policy_action_index]);
            }
            let action = context.records[context.canonical_index].action.clone();
            if step_to_next_decision(&mut environment, action).expect("step") {
                break;
            }
        }
    }

    #[test]
    fn corrupted_episode_is_rejected() {
        let mut episode =
            collect_canonical_episode(config(), provenance(Phase4Split::CanonicalTrain), 2)
                .expect("episode");
        episode.game_seed = 2_000_002;
        for sample in &mut episode.samples {
            sample.game_seed = episode.game_seed;
        }
        validate_episode(&episode).expect("valid");
        let mut illegal = episode.clone();
        let chosen = illegal.samples[0].chosen_index;
        illegal.samples[0].legal_mask[chosen] = false;
        assert!(validate_episode(&illegal).is_err());
        let mut broken = episode.clone();
        broken.samples[1].delta_clear_rate += 1.0;
        assert!(validate_episode(&broken).is_err());
        let mut reserved = episode;
        reserved.game_seed = 120;
        assert!(validate_episode(&reserved).is_err());
    }

    #[test]
    fn teacher_label_roundtrip_preserves_raw_outcomes() {
        let label = TeacherLabel {
            teacher_rule_version: TEACHER_SELECTION_SCHEMA_VERSION,
            canonical_action_id: "continue".to_string(),
            teacher_action_id: "reroll:1,2".to_string(),
            teacher_override: true,
            forced: false,
            proposal_action_ids: vec!["reroll:1,2".to_string()],
            reroll_proposal_scores: vec![("reroll:1,2".to_string(), 3.5)],
            discovery_outcomes: vec![vec![30.25, 31.5, 29.0]],
            discovery_means: vec![("reroll:1,2".to_string(), 30.25)],
            discovery_top3: vec!["reroll:1,2".to_string()],
            validation_baseline_outcomes: vec![28.0, 29.5],
            validation_candidate_outcomes: vec![vec![31.0, 30.0]],
            validation: vec![CandidateValidationStat {
                action_id: "reroll:1,2".to_string(),
                mean_delta: 1.75,
                sd_delta: 1.76,
                se_delta: 1.25,
                t_statistic: 1.4,
                p_value: 0.19,
                p_holm: 0.19,
                passed: false,
            }],
            elapsed_seconds: 1.0,
        };
        let json = serde_json::to_string(&label).expect("serialize");
        let back: TeacherLabel = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, label);
    }
}
