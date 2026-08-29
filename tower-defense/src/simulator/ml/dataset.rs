use super::contract::{DATASET_SCHEMA_VERSION, MlContract};
use super::seed::SeedRange;
use crate::config::GameConfig;
use crate::simulator::environment::ActionKind;
use crate::simulator::policy_runner::{
    run_item_expert_trajectory, run_monte_carlo_expert_trajectory, run_scripted_expert_trajectory,
    run_scripted_oracle_trajectory, run_spiral_expert_trajectory,
};
use crate::simulator::trajectory::Trajectory;
use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpertDatasetMetadata {
    pub dataset_schema_version: u32,
    pub contract: MlContract,
    pub expert_policy_id: String,
    pub expert_policy_version: u32,
    pub dataset_role: DatasetRole,
    pub seed_start: u64,
    pub seed_end: u64,
    pub seed_digest: String,
    pub max_decisions_per_episode: usize,
    pub episode_count: usize,
    pub step_count: usize,
    pub action_kind_counts: BTreeMap<String, usize>,
    pub includes_truncated: bool,
    pub git_revision: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatasetRole {
    BehaviorSmoke,
    Expert,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpertDataset {
    pub metadata: ExpertDatasetMetadata,
    pub episodes: Vec<Trajectory>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "record", content = "value")]
enum DatasetLine {
    Metadata(ExpertDatasetMetadata),
    Episode(Trajectory),
}

pub fn collect_expert_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
    include_truncated: bool,
) -> Result<ExpertDataset> {
    collect_behavior_dataset_with_runner(
        config,
        seed_range,
        max_decisions_per_episode,
        include_truncated,
        |config, seed, _| run_scripted_oracle_trajectory(config, seed),
        "scripted_oracle_smoke",
    )
}

pub fn collect_scripted_expert_behavior_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
) -> Result<ExpertDataset> {
    collect_behavior_dataset_with_runner(
        config,
        seed_range,
        max_decisions_per_episode,
        true,
        run_scripted_expert_trajectory,
        "scripted_expert_behavior",
    )
}

pub fn collect_spiral_expert_behavior_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
) -> Result<ExpertDataset> {
    collect_behavior_dataset_with_runner(
        config,
        seed_range,
        max_decisions_per_episode,
        true,
        run_spiral_expert_trajectory,
        "spiral_expert_behavior",
    )
}

pub fn collect_monte_carlo_expert_behavior_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
) -> Result<ExpertDataset> {
    collect_behavior_dataset_with_runner(
        config,
        seed_range,
        max_decisions_per_episode,
        true,
        run_monte_carlo_expert_trajectory,
        "monte_carlo_expert_behavior",
    )
}

pub fn collect_item_expert_behavior_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
) -> Result<ExpertDataset> {
    collect_behavior_dataset_with_runner(
        config,
        seed_range,
        max_decisions_per_episode,
        true,
        run_item_expert_trajectory,
        "item_expert_behavior",
    )
}

pub fn collect_all_expert_behavior_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
) -> Result<ExpertDataset> {
    type ExpertRunner = fn(Arc<GameConfig>, u64, usize) -> Result<Trajectory>;
    let runners: [ExpertRunner; 4] = [
        run_scripted_expert_trajectory,
        run_spiral_expert_trajectory,
        run_monte_carlo_expert_trajectory,
        run_item_expert_trajectory,
    ];
    let jobs = runners
        .into_iter()
        .enumerate()
        .flat_map(|(policy_index, runner)| {
            seed_range
                .seeds()
                .into_iter()
                .map(move |seed| (policy_index, seed, runner))
        })
        .collect::<Vec<_>>();
    let mut episodes = jobs
        .par_iter()
        .map(|&(policy_index, seed, runner)| {
            let trajectory = runner(Arc::clone(&config), seed, max_decisions_per_episode)?;
            if trajectory.steps.len() > max_decisions_per_episode {
                return Err(anyhow::anyhow!(
                    "combined expert exceeded max decisions for policy {policy_index} seed {seed}"
                ));
            }
            Ok((policy_index, seed, trajectory))
        })
        .collect::<Result<Vec<_>>>()?;
    episodes.sort_by_key(|(policy_index, seed, _)| (*seed, *policy_index));
    let episodes = episodes
        .into_iter()
        .map(|(_, _, trajectory)| trajectory)
        .collect::<Vec<_>>();

    let mut action_kind_counts = BTreeMap::new();
    for episode in &episodes {
        for step in &episode.steps {
            *action_kind_counts
                .entry(step.action.kind().wire_name().to_string())
                .or_insert(0) += 1;
        }
    }
    for index in 0..ActionKind::COUNT {
        action_kind_counts
            .entry(action_kind_name(index))
            .or_insert(0);
    }
    let dataset = ExpertDataset {
        metadata: ExpertDatasetMetadata {
            dataset_schema_version: DATASET_SCHEMA_VERSION,
            contract: MlContract::from_config(&config),
            expert_policy_id: "combined_experts_behavior".to_string(),
            expert_policy_version: 1,
            dataset_role: DatasetRole::BehaviorSmoke,
            seed_start: seed_range.start,
            seed_end: seed_range.end_inclusive,
            seed_digest: seed_range.digest(),
            max_decisions_per_episode,
            episode_count: episodes.len(),
            step_count: episodes.iter().map(|episode| episode.steps.len()).sum(),
            action_kind_counts,
            includes_truncated: episodes
                .iter()
                .any(|episode| episode.steps.iter().any(|step| step.truncated)),
            git_revision: super::neural_checkpoint::current_git_revision()?,
        },
        episodes,
    };
    validate_dataset(&dataset, &config)?;
    Ok(dataset)
}

fn collect_behavior_dataset_with_runner(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
    include_truncated: bool,
    runner: impl Fn(Arc<GameConfig>, u64, usize) -> Result<Trajectory> + Sync,
    policy_id: &str,
) -> Result<ExpertDataset> {
    if max_decisions_per_episode == 0 {
        bail!("max decisions per episode must be positive");
    }
    let mut episodes = seed_range
        .seeds()
        .par_iter()
        .map(|&seed| {
            let trajectory = runner(Arc::clone(&config), seed, max_decisions_per_episode)?;
            if trajectory.steps.len() > max_decisions_per_episode {
                return Err(anyhow::anyhow!(
                    "behavior policy exceeded max decisions for seed {seed}"
                ));
            }
            let is_truncated = trajectory.steps.iter().any(|step| step.truncated);
            if include_truncated || !is_truncated {
                Ok(Some(trajectory))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    episodes.sort_by_key(|episode| episode.metadata.seed);

    let mut action_kind_counts = BTreeMap::new();
    let mut step_count = 0;
    let mut includes_truncated = false;
    for episode in &episodes {
        step_count += episode.steps.len();
        includes_truncated |= episode.steps.iter().any(|step| step.truncated);
        for step in &episode.steps {
            let name = step.action.kind().wire_name().to_string();
            *action_kind_counts.entry(name).or_insert(0) += 1;
        }
    }
    for kind in 0..ActionKind::COUNT {
        let name = match kind {
            0 => ActionKind::PurchaseShopItem,
            1 => ActionKind::StartSelectingTower,
            2 => ActionKind::BeginRerollSelection,
            3 => ActionKind::BeginTowerSelection,
            4 => ActionKind::SelectHandCard,
            5 => ActionKind::DeselectHandCard,
            6 => ActionKind::ConfirmCardSelection,
            7 => ActionKind::CancelCardSelection,
            8 => ActionKind::Reroll,
            9 => ActionKind::SelectTower,
            10 => ActionKind::PlaceTower,
            11 => ActionKind::RemoveTower,
            12 => ActionKind::StartDefense,
            13 => ActionKind::SelectTreasure,
            14 => ActionKind::SelectCardServiceCard,
            15 => ActionKind::ConfirmCardServiceSelection,
            16 => ActionKind::UseInventoryItem,
            17 => ActionKind::Continue,
            _ => unreachable!(),
        };
        action_kind_counts
            .entry(name.wire_name().to_string())
            .or_insert(0);
    }
    let metadata = ExpertDatasetMetadata {
        dataset_schema_version: DATASET_SCHEMA_VERSION,
        contract: MlContract::from_config(&config),
        expert_policy_id: policy_id.to_string(),
        expert_policy_version: 1,
        dataset_role: DatasetRole::BehaviorSmoke,
        seed_start: seed_range.start,
        seed_end: seed_range.end_inclusive,
        seed_digest: seed_range.digest(),
        max_decisions_per_episode,
        episode_count: episodes.len(),
        step_count,
        action_kind_counts,
        includes_truncated,
        git_revision: super::neural_checkpoint::current_git_revision()?,
    };
    let dataset = ExpertDataset { metadata, episodes };
    validate_dataset(&dataset, &config)?;
    Ok(dataset)
}

pub fn collect_behavior_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
) -> Result<ExpertDataset> {
    collect_expert_dataset(config, seed_range, max_decisions_per_episode, true)
}

pub fn collect_strict_expert_dataset(
    config: Arc<GameConfig>,
    seed_range: SeedRange,
    max_decisions_per_episode: usize,
) -> Result<ExpertDataset> {
    if max_decisions_per_episode == 0 {
        bail!("max decisions per episode must be positive");
    }
    let mut results = seed_range
        .seeds()
        .par_iter()
        .map(|&seed| {
            run_scripted_expert_trajectory(Arc::clone(&config), seed, max_decisions_per_episode)
                .map(|episode| (seed, episode))
        })
        .collect::<Result<Vec<_>>>()?;
    results.sort_by_key(|(seed, _)| *seed);
    let mut attempts = Vec::new();
    let mut episodes = Vec::new();
    for (seed, episode) in results {
        let last_actions = episode
            .steps
            .iter()
            .rev()
            .take(3)
            .rev()
            .map(|step| step.action.action_id())
            .collect::<Vec<_>>();
        let first_actions = episode
            .steps
            .iter()
            .take(12)
            .map(|step| step.action.action_id())
            .collect::<Vec<_>>();
        attempts.push((
            seed,
            episode.outcome.clone(),
            episode.steps.len(),
            first_actions,
            last_actions,
        ));
        if episode
            .outcome
            .as_ref()
            .is_some_and(|outcome| outcome.victory)
        {
            episodes.push(episode);
            break;
        }
    }
    if episodes.is_empty() {
        let summary = attempts
            .iter()
            .map(|(seed, outcome, step_count, first_actions, last_actions)| {
                format!(
                    "seed={seed} stage={} clear_rate={:.3} reason={:?} steps={step_count} first_actions={first_actions:?} last_actions={last_actions:?} return={:.3}",
                    outcome.as_ref().map_or(0, |outcome| outcome.final_stage),
                    outcome.as_ref().map_or(0.0, |outcome| outcome.clear_rate),
                    outcome.as_ref().map(|outcome| &outcome.termination_reason),
                    outcome
                        .as_ref()
                        .map_or(0.0, |outcome| outcome.episode_return),
                )
            })
            .collect::<Vec<_>>();
        bail!("scripted full-clear expert found no victory in seed range: {summary:?}");
    }
    let mut action_kind_counts = BTreeMap::new();
    for episode in &episodes {
        for step in &episode.steps {
            *action_kind_counts
                .entry(step.action.kind().wire_name().to_string())
                .or_insert(0) += 1;
        }
    }
    for index in 0..ActionKind::COUNT {
        action_kind_counts
            .entry(action_kind_name(index))
            .or_insert(0);
    }
    let dataset = ExpertDataset {
        metadata: ExpertDatasetMetadata {
            dataset_schema_version: DATASET_SCHEMA_VERSION,
            contract: MlContract::from_config(&config),
            expert_policy_id: "scripted_full_clear_v1".to_string(),
            expert_policy_version: 1,
            dataset_role: DatasetRole::Expert,
            seed_start: seed_range.start,
            seed_end: seed_range.end_inclusive,
            seed_digest: seed_range.digest(),
            max_decisions_per_episode,
            episode_count: episodes.len(),
            step_count: episodes.iter().map(|episode| episode.steps.len()).sum(),
            action_kind_counts,
            includes_truncated: episodes.iter().any(|episode| {
                episode
                    .outcome
                    .as_ref()
                    .is_some_and(|outcome| outcome.truncated)
            }),
            git_revision: super::neural_checkpoint::current_git_revision()?,
        },
        episodes,
    };
    validate_dataset(&dataset, &config)?;
    Ok(dataset)
}

pub fn validate_dataset(dataset: &ExpertDataset, config: &GameConfig) -> Result<()> {
    if dataset.metadata.dataset_schema_version != DATASET_SCHEMA_VERSION {
        bail!(
            "unsupported expert dataset schema {}",
            dataset.metadata.dataset_schema_version
        );
    }
    if dataset.metadata.git_revision.trim().is_empty() {
        bail!("expert dataset git revision must not be empty");
    }
    if dataset.metadata.dataset_role == DatasetRole::Expert {
        validate_strict_expert_dataset(dataset)?;
    }
    let expected = MlContract::from_config(config);
    if dataset.metadata.contract != expected {
        bail!("expert dataset contract does not match current configuration");
    }
    let expected_digest =
        SeedRange::try_new(dataset.metadata.seed_start, dataset.metadata.seed_end)?.digest();
    if dataset.metadata.seed_digest != expected_digest {
        bail!("expert dataset seed digest does not match its seed range");
    }
    for episode in &dataset.episodes {
        expected
            .validate_trajectory(episode)
            .map_err(|error| anyhow::anyhow!("expert dataset contract failed: {error}"))?;
        for step in &episode.steps {
            let matches = step
                .legal_actions
                .iter()
                .filter(|legal| legal.action == step.action)
                .count();
            if matches != 1 {
                bail!("expert action must occur exactly once in the legal action list");
            }
        }
    }
    let step_count: usize = dataset
        .episodes
        .iter()
        .map(|episode| episode.steps.len())
        .sum();
    if dataset.metadata.step_count != step_count {
        bail!("expert dataset step count does not match metadata");
    }
    let includes_truncated = dataset
        .episodes
        .iter()
        .any(|episode| episode.steps.iter().any(|step| step.truncated));
    if dataset.metadata.includes_truncated != includes_truncated {
        bail!("expert dataset truncation metadata does not match episodes");
    }
    let mut action_kind_counts = BTreeMap::new();
    for episode in &dataset.episodes {
        for step in &episode.steps {
            *action_kind_counts
                .entry(step.action.kind().wire_name().to_string())
                .or_insert(0) += 1;
        }
    }
    for kind in 0..ActionKind::COUNT {
        let name = action_kind_name(kind);
        action_kind_counts.entry(name).or_insert(0);
    }
    if dataset.metadata.action_kind_counts != action_kind_counts {
        bail!("expert dataset action-kind counts do not match episodes");
    }
    Ok(())
}

pub fn validate_strict_expert_dataset(dataset: &ExpertDataset) -> Result<()> {
    if dataset.metadata.dataset_role != DatasetRole::Expert {
        bail!("behavior dataset cannot be used as an expert dataset");
    }
    for episode in &dataset.episodes {
        let outcome = episode
            .outcome
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("expert episode has no outcome summary"))?;
        if !outcome.victory || outcome.clear_rate < 100.0 {
            bail!(
                "expert episode is not a full-clear victory: clear_rate={:.2} final_stage={} return={:.3}",
                outcome.clear_rate,
                outcome.final_stage,
                outcome.episode_return
            );
        }
        if !outcome.terminated || outcome.truncated {
            bail!("expert episode must terminate without truncation");
        }
        if matches!(
            outcome.termination_reason,
            crate::simulator::environment::StepReason::MaxTicks
                | crate::simulator::environment::StepReason::MaxDecisions
                | crate::simulator::environment::StepReason::NoProgressCycle
        ) {
            bail!("expert episode has a forbidden termination reason");
        }
    }
    Ok(())
}

fn action_kind_name(index: usize) -> String {
    const NAMES: [&str; ActionKind::COUNT] = [
        "purchase_shop_item",
        "start_selecting_tower",
        "begin_reroll_selection",
        "begin_tower_selection",
        "select_hand_card",
        "deselect_hand_card",
        "confirm_card_selection",
        "cancel_card_selection",
        "reroll",
        "select_tower",
        "place_tower",
        "remove_tower",
        "start_defense",
        "select_treasure",
        "select_card_service_card",
        "confirm_card_service_selection",
        "use_inventory_item",
        "continue",
    ];
    NAMES[index].to_string()
}

pub fn write_jsonl(dataset: &ExpertDataset, path: &Path) -> Result<()> {
    let mut writer = File::create(path)
        .with_context(|| format!("failed to create dataset {}", path.display()))?;
    serde_json::to_writer(
        &mut writer,
        &DatasetLine::Metadata(dataset.metadata.clone()),
    )?;
    writer.write_all(b"\n")?;
    for episode in &dataset.episodes {
        serde_json::to_writer(&mut writer, &DatasetLine::Episode(episode.clone()))?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

pub fn read_jsonl(path: &Path) -> Result<ExpertDataset> {
    let reader = BufReader::new(File::open(path)?);
    let mut metadata = None;
    let mut episodes = Vec::new();
    for line in reader.lines() {
        match serde_json::from_str::<DatasetLine>(&line?)? {
            DatasetLine::Metadata(value) => {
                if metadata.replace(value).is_some() {
                    bail!("expert dataset contains multiple metadata records");
                }
            }
            DatasetLine::Episode(value) => episodes.push(value),
        }
    }
    let metadata = metadata.context("expert dataset has no metadata record")?;
    if metadata.episode_count != episodes.len() {
        bail!("expert dataset episode count does not match metadata");
    }
    Ok(ExpertDataset { metadata, episodes })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_dataset(config: &GameConfig, range: SeedRange) -> ExpertDataset {
        ExpertDataset {
            metadata: ExpertDatasetMetadata {
                dataset_schema_version: DATASET_SCHEMA_VERSION,
                contract: MlContract::from_config(config),
                expert_policy_id: "test-policy".to_string(),
                expert_policy_version: 1,
                dataset_role: DatasetRole::BehaviorSmoke,
                seed_start: range.start,
                seed_end: range.end_inclusive,
                seed_digest: range.digest(),
                max_decisions_per_episode: 1,
                episode_count: 0,
                step_count: 0,
                action_kind_counts: (0..ActionKind::COUNT)
                    .map(|index| (action_kind_name(index), 0))
                    .collect(),
                includes_truncated: false,
                git_revision: "test-revision".to_string(),
            },
            episodes: Vec::new(),
        }
    }

    #[test]
    fn dataset_metadata_and_jsonl_round_trip_preserve_contract() {
        let config = GameConfig::default_config();
        let range = SeedRange::try_new(0, 0).expect("valid range");
        let dataset = empty_dataset(&config, range);
        validate_dataset(&dataset, &config).expect("dataset is valid");
        let path = std::env::temp_dir().join(format!("td-expert-{}.jsonl", std::process::id()));
        write_jsonl(&dataset, &path).expect("write dataset");
        let loaded = read_jsonl(&path).expect("read dataset");
        assert_eq!(dataset, loaded);
        std::fs::remove_file(path).expect("remove dataset");
    }

    #[test]
    fn seed_digest_is_deterministic() {
        let range = SeedRange::try_new(4, 6).expect("valid range");
        assert_eq!(range.digest(), range.digest());
    }

    #[test]
    fn dataset_provenance_requires_non_empty_git_revision() {
        let config = GameConfig::default_config();
        let range = SeedRange::try_new(0, 0).expect("valid range");
        let mut dataset = empty_dataset(&config, range);
        assert!(!dataset.metadata.git_revision.trim().is_empty());
        dataset.metadata.git_revision.clear();
        assert!(validate_dataset(&dataset, &config).is_err());
    }

    #[test]
    fn smoke_dataset_is_valid_but_not_strict_expert() {
        let config = GameConfig::default_config();
        let range = SeedRange::try_new(0, 0).expect("valid range");
        let dataset = empty_dataset(&config, range);

        validate_dataset(&dataset, &config).expect("smoke dataset is valid");
        assert!(validate_strict_expert_dataset(&dataset).is_err());
    }

    #[test]
    fn seed_zero_behavior_dataset_round_trips_with_real_episode() {
        let config = Arc::new(GameConfig::default_config());
        let range = SeedRange::try_new(0, 0).expect("valid range");
        let dataset = collect_behavior_dataset(Arc::clone(&config), range, 256)
            .expect("collect seed zero behavior dataset");

        assert_eq!(dataset.metadata.dataset_role, DatasetRole::BehaviorSmoke);
        assert_eq!(dataset.metadata.episode_count, 1);
        assert_eq!(dataset.metadata.step_count, 61);
        assert!(!dataset.episodes[0].outcome.as_ref().unwrap().victory);
        validate_dataset(&dataset, &config).expect("behavior dataset is valid");

        let path = std::env::temp_dir().join(format!(
            "td-behavior-seed-zero-{}.jsonl",
            std::process::id()
        ));
        write_jsonl(&dataset, &path).expect("write behavior dataset");
        let loaded = read_jsonl(&path).expect("read behavior dataset");
        assert_eq!(loaded, dataset);
        assert!(validate_strict_expert_dataset(&loaded).is_err());
        std::fs::remove_file(path).expect("remove behavior dataset");
    }

    #[test]
    fn dataset_rejects_external_schema_mismatch() {
        let config = GameConfig::default_config();
        let range = SeedRange::try_new(0, 0).expect("valid range");
        let mut dataset = empty_dataset(&config, range);
        dataset.metadata.dataset_schema_version = DATASET_SCHEMA_VERSION - 1;

        let error = validate_dataset(&dataset, &config).expect_err("legacy schema must fail");

        assert!(
            error
                .to_string()
                .contains("unsupported expert dataset schema")
        );
    }
}
