//! Full-game statistics using the same semantic policy as Phase 4 evaluation.

use crate::config::GameConfig;
use crate::environment::{AgentAction, DecisionPoint, GameEnvironment, LegalAction, Observation};
use crate::events::{
    ContentKind, SimEvent, card_service_kind_from_key_id, item_kind_from_key_id,
    upgrade_kind_from_event_key,
};
use crate::ml::semantic_bc::SemanticPolicy;
use crate::policy_runner::{EpisodeResult, ForcedActionStats, PolicyRunnerConfig, PolicyStep};
use crate::recording::{SimRecorder, SimulationProvenance};
use anyhow::{Context, Result, ensure};
use std::sync::Arc;
use td_core::ShopContentCategory;

pub struct RecordedEpisode {
    pub episode: EpisodeResult,
    pub events: Vec<SimEvent>,
    pub stages: Vec<StageResult>,
}

pub struct StageResult {
    stage: usize,
    victory: bool,
    hp_before: f32,
    hp_after: f32,
    towers_placed: usize,
    gold_before: usize,
    gold_after: usize,
    tower_kind: String,
    rerolls_used: usize,
}

struct Statistics {
    events: Vec<SimEvent>,
    stages: Vec<StageResult>,
    service_cards_selected: usize,
}

impl Statistics {
    fn new() -> Self {
        Self {
            events: vec![SimEvent::GameStart],
            stages: Vec::new(),
            service_cards_selected: 0,
        }
    }

    fn observe(
        &mut self,
        before: &Observation,
        action: &AgentAction,
        after: &Observation,
    ) -> Result<()> {
        if self
            .stages
            .last()
            .is_none_or(|row| row.stage != before.stage)
        {
            self.events.push(SimEvent::StageStart {
                stage: before.stage,
            });
            self.stages.push(StageResult {
                stage: before.stage,
                victory: false,
                hp_before: before.hp_raw as f32 / 1000.0,
                hp_after: before.hp_raw as f32 / 1000.0,
                towers_placed: 0,
                gold_before: before.gold,
                gold_after: before.gold,
                tower_kind: String::new(),
                rerolls_used: 0,
            });
        }
        let row = self.stages.last_mut().expect("stage initialized");
        match action {
            AgentAction::PurchaseShopItem { slot_index } => {
                let slot = before
                    .shop
                    .iter()
                    .find(|slot| slot.index == *slot_index)
                    .with_context(|| {
                        format!("shop purchase references missing slot {slot_index}")
                    })?;
                let category = ShopContentCategory::from_id(slot.kind_id).with_context(|| {
                    format!("unknown shop content category id {}", slot.kind_id)
                })?;
                self.events.push(SimEvent::ShopPurchase {
                    stage: before.stage,
                    cost: slot.cost,
                    content_kind: ContentKind::from_shop_observation(category, slot.key_id)?,
                });
            }
            AgentAction::Reroll { .. } => {
                row.rerolls_used += 1;
                self.events.push(SimEvent::CardReroll {
                    stage: before.stage,
                    reroll_number: row.rerolls_used,
                });
            }
            AgentAction::UseInventoryItem { item_index } => {
                let item = before
                    .inventory
                    .iter()
                    .find(|item| item.index == *item_index)
                    .with_context(|| {
                        format!("item use references missing inventory slot {item_index}")
                    })?;
                self.events.push(SimEvent::ItemUsed {
                    stage: before.stage,
                    item_kind: item_kind_from_key_id(item.key_id)?,
                });
            }
            AgentAction::SelectTreasure { option_index } => {
                let kind = before
                    .treasure_options
                    .get(*option_index)
                    .with_context(|| {
                        format!("treasure selection references missing option {option_index}")
                    })?;
                self.events.push(SimEvent::TreasureSelected {
                    stage: before.stage,
                    upgrade_kind: upgrade_kind_from_event_key(kind)?,
                });
            }
            AgentAction::ConfirmCardServiceSelection => {
                if let Some(service) = &before.card_service {
                    if service.current_step == 0 {
                        self.service_cards_selected = 0;
                    }
                    self.service_cards_selected += service.selected_card_indices.len();
                    if service.current_step + 1 == service.step_count {
                        self.events.push(SimEvent::CardServiceUsed {
                            stage: before.stage,
                            service_kind: card_service_kind_from_key_id(service.key_id)?,
                            cards_selected: self.service_cards_selected,
                        });
                        self.service_cards_selected = 0;
                    }
                }
            }
            AgentAction::RemoveTower { tower_id } => {
                if let Some(tower) = before.towers.iter().find(|tower| tower.id == *tower_id) {
                    self.events.push(SimEvent::TowerRemoved {
                        stage: before.stage,
                        x: tower.left,
                        y: tower.top,
                    });
                }
            }
            _ => {}
        }
        // Compare stable tower identities so macro BuildTower and legacy
        // SelectTower/PlaceTower execution produce the same statistics.
        for tower in &after.towers {
            if !before.towers.iter().any(|old| old.id == tower.id) {
                row.towers_placed += 1;
                row.tower_kind = tower.template.kind.clone();
                self.events.push(SimEvent::TowerSelected {
                    stage: before.stage,
                    tower_kind: tower.template.kind.clone(),
                    rank: tower.template.rank.clone().unwrap_or_default(),
                    suit: tower.template.suit.clone().unwrap_or_default(),
                });
                self.events.push(SimEvent::TowerPlaced {
                    stage: before.stage,
                    tower_kind: tower.template.kind.clone(),
                    x: tower.left,
                    y: tower.top,
                });
            }
        }
        if matches!(action, AgentAction::StartDefense) {
            self.events.push(SimEvent::DefenseStart {
                stage: before.stage,
            });
        }
        row.hp_after = after.hp_raw as f32 / 1000.0;
        row.gold_after = after.gold;
        let defending = |point: &DecisionPoint| {
            matches!(
                point,
                DecisionPoint::Defense
                    | DecisionPoint::PreDefenseItem
                    | DecisionPoint::DamageResponseItem
            )
        };
        if defending(&before.decision_point) && !defending(&after.decision_point) {
            row.victory =
                !matches!(after.decision_point, DecisionPoint::Terminal) || after.hp_raw > 0;
            self.events.push(SimEvent::DefenseEnd {
                stage: before.stage,
                victory: row.victory,
            });
        }
        Ok(())
    }

    fn finish(mut self, episode: EpisodeResult) -> Result<RecordedEpisode> {
        ensure!(
            episode.terminated && !episode.truncated,
            "seed {} did not finish a full game: {:?}; no result recorded for this episode",
            episode.seed,
            episode.termination_reason
        );
        for row in &mut self.stages {
            if row.stage < episode.final_observation.stage
                || (row.stage == episode.final_observation.stage && episode.victory)
            {
                row.victory = true;
            }
        }
        self.events.push(SimEvent::GameEnd {
            final_stage: episode.final_observation.stage,
            victory: episode.victory,
            clear_rate: episode.clear_rate,
        });
        Ok(RecordedEpisode {
            episode,
            events: self.events,
            stages: self.stages,
        })
    }
}

pub fn run_semantic_statistics(
    config: Arc<GameConfig>,
    seed: u64,
    runner: &PolicyRunnerConfig,
    policy: &SemanticPolicy,
) -> Result<RecordedEpisode> {
    ensure!(
        runner.max_decisions_per_episode > 0,
        "max decisions must be positive"
    );
    let mut environment = GameEnvironment::new(config, seed);
    let mut statistics = Statistics::new();
    let mut steps = runner.record_steps.then(Vec::new);
    let mut decisions = 0;
    let mut candidate_evaluations = 0;
    let mut episode_return = 0.0;
    let mut forced_actions = ForcedActionStats::default();
    while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
        ensure!(
            decisions < runner.max_decisions_per_episode,
            "seed {seed}: max decisions reached before game end; no result recorded for this episode"
        );
        let choice = policy
            .choose(&environment)
            .with_context(|| format!("seed {seed}: semantic policy failed"))?;
        ensure!(
            choice.legal_mask[choice.index] && environment.semantic_action_is_legal(&choice.action),
            "seed {seed}: semantic policy selected an illegal action"
        );
        let before = choice.candidates.observation;
        let pre_progress_fingerprint = runner
            .record_steps
            .then(|| environment.progress_fingerprint());
        candidate_evaluations += choice.candidates.candidates.len();
        let mut outcome = environment
            .semantic_step(choice.action.clone())
            .map_err(|error| anyhow::anyhow!("seed {seed}: {error:?}"))?;
        statistics
            .observe(&before, &choice.action, &outcome.observation)
            .with_context(|| format!("seed {seed}: failed to record simulation event"))?;
        episode_return += outcome.reward.total();
        while !outcome.terminated && !outcome.truncated {
            let Some(action) = environment.forced_action() else {
                break;
            };
            let before = outcome.observation.clone();
            forced_actions.total += 1;
            *forced_actions
                .by_decision_point
                .entry(format!("{:?}", before.decision_point))
                .or_default() += 1;
            outcome = environment
                .step(action.clone())
                .map_err(|error| anyhow::anyhow!("seed {seed}: {error:?}"))?;
            statistics
                .observe(&before, &action, &outcome.observation)
                .with_context(|| format!("seed {seed}: failed to record forced-action event"))?;
            episode_return += outcome.reward.total();
        }
        ensure!(
            !outcome.truncated,
            "seed {seed}: simulation truncated: {:?}",
            outcome.info.reason
        );
        if let Some(steps) = &mut steps {
            let legal_actions = choice
                .candidates
                .candidates
                .into_iter()
                .zip(choice.legal_mask)
                .filter(|(_, legal)| *legal)
                .map(|(candidate, _)| LegalAction {
                    id: candidate.action_id,
                    action: candidate.action,
                })
                .collect::<Vec<_>>();
            // Spatial policies complete a candidate with a chosen cell.
            let mut legal_actions = legal_actions;
            if !legal_actions
                .iter()
                .any(|legal| legal.action == choice.action)
            {
                legal_actions.push(LegalAction {
                    id: choice.action.action_id(),
                    action: choice.action.clone(),
                });
            }
            steps.push(PolicyStep {
                observation: before,
                legal_actions,
                action: choice.action,
                outcome,
                pre_progress_fingerprint: pre_progress_fingerprint.expect("trace requested"),
                post_progress_fingerprint: environment.progress_fingerprint(),
            });
        }
        decisions += 1;
    }
    let final_observation = environment.snapshot();
    statistics.finish(EpisodeResult {
        seed,
        decision_count: decisions,
        ticks_advanced: final_observation.sim_tick,
        candidate_evaluations,
        placement_position_checks: 0,
        forced_actions,
        terminated: true,
        truncated: false,
        victory: environment.victory(),
        clear_rate: environment.clear_rate(),
        final_observation,
        final_state_hash: environment.state_hash(),
        metrics: environment.metrics(),
        episode_return,
        termination_reason: crate::environment::StepReason::Terminal,
        steps,
    })
}

pub fn record_episode(
    recorder: &SimRecorder,
    sim_id: &str,
    recorded: &RecordedEpisode,
    provenance: &SimulationProvenance,
) -> Result<()> {
    let episode = &recorded.episode;
    recorder.record_simulation_start_with_provenance(
        sim_id,
        &provenance.policy_kind,
        &provenance.policy_kind,
        &provenance.policy_kind,
        &provenance.policy_kind,
        &provenance.policy_kind,
        episode.seed,
        provenance,
    )?;
    recorder.record_events(sim_id, &recorded.events)?;
    for event in &recorded.events {
        if let SimEvent::TreasureSelected {
            stage,
            upgrade_kind,
        } = event
        {
            recorder.record_upgrade(sim_id, upgrade_kind.key(), *stage, None)?;
        }
    }
    for row in &recorded.stages {
        recorder.record_stage_result(
            sim_id,
            row.stage,
            row.victory,
            row.hp_before,
            row.hp_after,
            row.towers_placed,
            row.gold_before,
            row.gold_after,
            &row.tower_kind,
            row.rerolls_used,
        )?;
    }
    recorder.record_damage_dealt(sim_id, episode.metrics.total_tower_damage)?;
    // completed_at is set last: incomplete records are excluded by stats queries.
    recorder.record_simulation_end(
        sim_id,
        episode.victory,
        episode.final_observation.stage,
        episode.clear_rate,
        episode.final_observation.hp_raw as f32 / 1000.0,
        episode.final_observation.gold,
        episode.metrics.total_towers_placed,
        episode.metrics.total_items_used,
        episode.metrics.total_player_damage,
        episode.metrics.total_gold_earned,
    )
}

/// Validate the policy's schema and configuration before creating a results DB.
pub fn load_semantic_policy(
    path: &std::path::Path,
    config: &GameConfig,
    allow_config_change: bool,
) -> Result<(SemanticPolicy, Option<usize>)> {
    use crate::ml::phase4_dataset::DatasetProvenance;
    use crate::ml::semantic_ppo::{PpoActorFile, PpoRunMetadata};
    let check_provenance = |provenances: &[DatasetProvenance]| -> Result<()> {
        ensure!(
            !provenances.is_empty(),
            "checkpoint is missing training configuration provenance"
        );
        for provenance in provenances {
            let mut provenance = provenance.clone();
            if allow_config_change {
                provenance.game_config_digest = crate::config::config_digest(config);
            }
            provenance.check_current(config)?;
        }
        Ok(())
    };
    let iteration = if path.join("ppo-actor.json").exists() {
        let actor: PpoActorFile =
            serde_json::from_slice(&std::fs::read(path.join("ppo-actor.json"))?)?;
        if let Some(environment) = &actor.environment {
            if allow_config_change {
                environment.check_encoding()?;
                crate::ml::retraining::numeric_changes(&environment.game_config, config)?;
                ensure!(
                    environment.core_tree_hash == crate::ml::phase4_dataset::core_tree_hash(),
                    "core rules source changed since this checkpoint"
                );
            } else {
                environment.check_run(config)?;
            }
        } else {
            let parent: PpoRunMetadata = serde_json::from_slice(
                &std::fs::read(
                    path.parent()
                        .context("checkpoint has no parent")?
                        .join("ppo.json"),
                )
                .context("legacy PPO checkpoint needs its parent ppo.json")?,
            )?;
            check_provenance(&parent.init_bc_metadata.train_provenance)?;
        }
        Some(actor.iteration)
    } else {
        let metadata = crate::ml::semantic_bc::load_checkpoint_metadata(path)?;
        check_provenance(&metadata.train_provenance)?;
        None
    };
    Ok((SemanticPolicy::from_path(path)?, iteration))
}

#[derive(Default)]
pub struct SimulationSummary {
    pub samples: usize,
    pub victories: usize,
    pub clear_rates: Vec<f32>,
    pub damage_taken: f64,
    pub damage_dealt: f64,
}

impl SimulationSummary {
    pub fn observe(&mut self, episode: &EpisodeResult) {
        self.samples += 1;
        self.victories += episode.victory as usize;
        self.clear_rates.push(episode.clear_rate);
        self.damage_taken += episode.metrics.total_player_damage as f64;
        self.damage_dealt += episode.metrics.total_tower_damage as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfigTestExt;
    use crate::environment::{CardServiceObservation, InventoryObservation, ShopSlotObservation};

    #[test]
    fn statistics_build_content_events_from_typed_catalog_ids() {
        let config = Arc::new(GameConfig::default_config());
        let environment = GameEnvironment::new(config, 0);
        let mut observation = environment.snapshot();
        observation.shop = vec![ShopSlotObservation {
            index: 0,
            purchased: false,
            kind: "item".into(),
            kind_id: td_core::ShopContentCategory::Item.id(),
            key: "wrong_item_name".into(),
            key_id: td_core::ItemKind::Bread.id(),
            cost: 10,
        }];
        observation.inventory = vec![InventoryObservation {
            index: 0,
            key: "wrong_item_name".into(),
            key_id: td_core::ItemKind::Bread.id(),
        }];
        observation.treasure_options = vec!["black_white".into()];
        observation.card_service = Some(CardServiceObservation {
            key: "wrong_service_name".into(),
            key_id: td_core::CardServiceKind::LongSword.id(),
            polish_delta_raw: None,
            suit_change_raw: None,
            target_filter: None,
            current_step: 0,
            step_count: 1,
            required_count: 1,
            selected_card_indices: vec![0],
            candidate_card_indices: vec![0],
        });

        let mut statistics = Statistics::new();
        for action in [
            AgentAction::PurchaseShopItem { slot_index: 0 },
            AgentAction::UseInventoryItem { item_index: 0 },
            AgentAction::SelectTreasure { option_index: 0 },
            AgentAction::ConfirmCardServiceSelection,
        ] {
            statistics
                .observe(&observation, &action, &observation)
                .unwrap();
        }

        assert!(statistics.events.iter().any(|event| matches!(
            event,
            SimEvent::ShopPurchase {
                content_kind: ContentKind::Item(td_core::ItemKind::Bread),
                ..
            }
        )));
        assert!(statistics.events.iter().any(|event| matches!(
            event,
            SimEvent::ItemUsed {
                item_kind: td_core::ItemKind::Bread,
                ..
            }
        )));
        assert!(statistics.events.iter().any(|event| matches!(
            event,
            SimEvent::TreasureSelected {
                upgrade_kind: td_core::UpgradeKind::BlackWhite,
                ..
            }
        )));
        assert!(statistics.events.iter().any(|event| matches!(
            event,
            SimEvent::CardServiceUsed {
                service_kind: td_core::CardServiceKind::LongSword,
                cards_selected: 1,
                ..
            }
        )));
    }

    #[test]
    fn reports_exclude_other_runs_and_incomplete_games() {
        let path = std::env::temp_dir().join(format!("td-statistics-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let recorder = SimRecorder::new(&path).unwrap();
        for id in ["old-1", "current-1", "current-incomplete"] {
            recorder
                .record_simulation_start_with_provenance(
                    id,
                    "test",
                    "test",
                    "test",
                    "test",
                    "test",
                    1,
                    &SimulationProvenance::legacy(),
                )
                .unwrap();
            recorder
                .record_events(
                    id,
                    &[SimEvent::TreasureSelected {
                        stage: 1,
                        upgrade_kind: td_core::UpgradeKind::BlackWhite,
                    }],
                )
                .unwrap();
            if !id.ends_with("incomplete") {
                recorder
                    .record_simulation_end(id, false, 2, 3.0, 0.0, 0, 0, 0, 60.0, 0)
                    .unwrap();
            }
        }
        let all = crate::stats::Database::open(&path)
            .unwrap()
            .list_treasures()
            .unwrap();
        assert_eq!(all[0].total_purchases, 2);
        let current = crate::stats::Database::open_run(&path, "current-")
            .unwrap()
            .list_treasures()
            .unwrap();
        assert_eq!(current[0].total_purchases, 1);
        assert_eq!(current[0].selected_simulations, 1);
        drop(recorder);
        std::fs::remove_file(path).unwrap();
    }
}
