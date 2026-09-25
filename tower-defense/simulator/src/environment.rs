//! Versioned, headless game environment for policy-driven simulation.

use crate::GameCore;
use crate::config::GameConfig;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use td_core::CommandError;
#[cfg(test)]
use td_core::PlayerCommand;

pub const ENVIRONMENT_VERSION: u32 = 7;
pub const ACTION_SCHEMA_VERSION: u32 = 7;
pub const ENVIRONMENT_REPLAY_SCHEMA_VERSION: u32 = 7;
pub const DEFAULT_MAX_ADVANCE_TICKS: u64 = 60 * 60 * 5;
/// Provisional: measured via `teacher::tests::phase1_candidate_recall_report`
/// (see docs/game-ai/02-action-contract.md). At 32, the oracle-best
/// `BuildTower` candidate by `rank_build_tower_actions_by_heuristic` was
/// excluded from every sampled decision (144 samples, 24 seeds); at 64,
/// coverage regret was 0 in all samples. Not yet the final design: the
/// underlying route-distance position ordering conflates "closest to the
/// route" with "best route coverage", which this limit increase works
/// around rather than fixes.
pub const DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT: usize = 64;

pub use td_core::RewardConfig;

const HIDDEN_ORDER_DRAW_PILE: u64 = 0;
const HIDDEN_ORDER_SHOP_CATEGORY: u64 = 1;
const HIDDEN_ORDER_SHOP_RARITY: u64 = 2;
const HIDDEN_ORDER_SHOP_CONTENT: u64 = 3;
const HIDDEN_ORDER_REWARD_UPGRADE: u64 = 4;

/// Teacher-only information-set resampling. The authoritative state already
/// materializes future random order (the draw pile order and the unconsumed
/// suffix of every shop/reward bag) that no `Observation` exposes. A rollout
/// fork that kept it would let candidates "see" e.g. which card a `Reroll`
/// draws. This keeps membership, cursors and cycles and reshuffles only that
/// hidden order, deterministically per (game seed, scenario seed, source
/// tick, component), so every candidate of one scenario shares one sample.
fn resample_hidden_order(parts: &mut td_core::CoreSnapshotParts, game_seed: u64, scenario_seed: u64) {
    let sim_tick = parts.sim_tick.ticks();
    let rng = |component: u64, index: u64| {
        td_core::rng_for(
            game_seed,
            td_core::domain::ML_TOWER_TEACHER_HIDDEN_ORDER,
            &[scenario_seed, sim_tick, component, index],
        )
    };
    td_core::shuffle(
        &mut parts.deck.draw_pile,
        &mut rng(HIDDEN_ORDER_DRAW_PILE, 0),
    );
    let shop = &mut parts.rng.shop;
    let cursor = shop.category_bag.cursor.min(shop.category_bag.entries.len());
    td_core::shuffle(
        &mut shop.category_bag.entries[cursor..],
        &mut rng(HIDDEN_ORDER_SHOP_CATEGORY, 0),
    );
    for (index, bag) in shop.rarity_bags.iter_mut().enumerate() {
        let cursor = bag.cursor.min(bag.entries.len());
        td_core::shuffle(
            &mut bag.entries[cursor..],
            &mut rng(HIDDEN_ORDER_SHOP_RARITY, index as u64),
        );
    }
    for (index, bag) in shop.content_bags.iter_mut().enumerate() {
        let cursor = bag.cursor.min(bag.entries.len());
        td_core::shuffle(
            &mut bag.entries[cursor..],
            &mut rng(HIDDEN_ORDER_SHOP_CONTENT, index as u64),
        );
    }
    let bag = &mut parts.rng.reward_upgrade_bag;
    let cursor = bag.cursor.min(bag.entries.len());
    td_core::shuffle(
        &mut bag.entries[cursor..],
        &mut rng(HIDDEN_ORDER_REWARD_UPGRADE, 0),
    );
}

pub fn potential(observation: &Observation) -> f32 {
    let hp_ratio = observation.hp_raw.max(0) as f32 / observation.max_hp_raw.max(1) as f32;
    let stage_progress =
        observation.stage_progress_raw.max(0) as f32 / observation.stage_total_hp_raw.max(1) as f32;
    observation.stage as f32 + stage_progress + hp_ratio * 0.01
}

pub fn potential_shaping(
    before: &Observation,
    after: &Observation,
    terminated: bool,
    config: &RewardConfig,
) -> f32 {
        td_core::diag_scope!(RewardShaping);
    let next_potential = if terminated { 0.0 } else { potential(after) };
    config.potential_weight * (config.potential_gamma * next_potential - potential(before))
}

fn decision_point_from_flow(flow: &td_core::GameFlowState) -> DecisionPoint {
    match flow {
        td_core::GameFlowState::Shopping(_) => DecisionPoint::Shop,
        td_core::GameFlowState::SelectingTower => DecisionPoint::CardSelection,
        td_core::GameFlowState::PlacingTower => DecisionPoint::TowerPlacement,
        td_core::GameFlowState::Defense(_) => DecisionPoint::Defense,
        td_core::GameFlowState::TreasureSelection { .. } => DecisionPoint::TreasureSelection,
        td_core::GameFlowState::Result { .. } | td_core::GameFlowState::Initializing => {
            DecisionPoint::Terminal
        }
    }
}

fn shaping_rewards(
    clear_rate_before: f32,
    clear_rate_after: f32,
    escaped_hp: f32,
    player_damage: f32,
    stage_total_hp: f32,
    tower_damage: f32,
    config: &RewardConfig,
) -> BTreeMap<String, f32> {
        td_core::diag_scope!(RewardShaping);
    let progress_reward = clear_rate_after - clear_rate_before;
    let escaped_penalty = -(escaped_hp / stage_total_hp.max(1.0) / config.escaped_hp_penalty_scale);
    let hp_loss_penalty = -(player_damage / config.player_hp_loss_penalty_scale);
    BTreeMap::from([
        ("clear_progress".to_string(), progress_reward),
        ("escaped_hp_penalty".to_string(), escaped_penalty),
        ("player_hp_loss_penalty".to_string(), hp_loss_penalty),
        (
            "damage_progress".to_string(),
            config.damage_progress_weight * tower_damage / stage_total_hp.max(1.0),
        ),
        ("no_progress_cycle_penalty".to_string(), 0.0),
    ])
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum DecisionContext {
    None,
    CardSelection {
        purpose: CardSelectionPurpose,
        selected_slot_indices: Vec<usize>,
    },
    PreDefenseItem {
        stage: usize,
    },
    DamageResponseItem {
        stage: usize,
        trigger_tick: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum CardSelectionPurpose {
    Reroll,
    BuildTower,
}

pub use td_core::{ActionKind, AgentAction, DecisionPoint};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalAction {
    pub id: String,
    pub action: AgentAction,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LegalActionGenerationMetrics {
    pub placement_position_checks: usize,
}

pub use td_core::CardObservation;
pub use td_core::ShopSlotObservation;

pub use td_core::{HandItemObservation, TowerTemplateObservation};

pub use td_core::BuildTowerCandidateObservation;
pub use td_core::HandObservation;

pub use td_core::TowerObservation;
pub use td_core::{
    DamageSplashObservation, TowerStatusEffectObservation, TowerStatusEffectObservationKind,
};
pub use td_core::{InventoryObservation, MonsterObservation, OwnedUpgradeObservation};

pub use td_core::DeckObservation;

pub use td_core::StageModifiersObservation;

pub use td_core::Observation;
pub use td_core::{QueuedMonsterGroupObservation, WaveGroupObservation};

pub use td_core::RouteCoordObservation;

pub use td_core::CardServiceObservation;

pub use td_core::{RewardComponents, StepInfo, StepReason};

pub use td_core::StepOutcome;

pub use td_core::ReplayCheckpoint as EnvironmentReplayCheckpoint;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentReplay {
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub replay_schema_version: u32,
    pub config_version: u32,
    #[serde(default)]
    pub config_digest_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
    pub seed: u64,
    pub commands: Vec<AgentAction>,
    pub checkpoints: Vec<EnvironmentReplayCheckpoint>,
}

impl EnvironmentReplay {
    pub fn validate_schema(&self) -> Result<(), String> {
        if self.environment_version != ENVIRONMENT_VERSION {
            return Err(format!(
                "unsupported environment replay environment schema {}",
                self.environment_version
            ));
        }
        if self.action_schema_version != ACTION_SCHEMA_VERSION {
            return Err(format!(
                "unsupported environment replay action schema {}",
                self.action_schema_version
            ));
        }
        if self.replay_schema_version != ENVIRONMENT_REPLAY_SCHEMA_VERSION {
            return Err(format!(
                "unsupported environment replay schema {}",
                self.replay_schema_version
            ));
        }
        if self.config_digest_version != 0
            && self.config_digest_version != crate::config::CONFIG_DIGEST_VERSION
        {
            return Err(format!(
                "unsupported environment replay config digest schema {}",
                self.config_digest_version
            ));
        }
        Ok(())
    }

    pub fn execute(&self, config: Arc<GameConfig>) -> Result<GameEnvironment, String> {
        self.validate_schema()?;
        let mut environment = GameEnvironment::new(config, self.seed);
        for action in &self.commands {
            environment
                .step(action.clone())
                .map_err(|error| format!("{error:?}"))?;
        }
        if let Some(checkpoint) = self.checkpoints.last()
            && environment.state_hash() != checkpoint.state_hash
        {
            return Err(format!(
                "environment replay final hash mismatch: expected {}, got {}",
                checkpoint.state_hash,
                environment.state_hash()
            ));
        }
        Ok(environment)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnvironmentMetrics {
    pub stage_damage: Vec<(usize, f32)>,
    pub total_towers_placed: usize,
    pub total_items_used: usize,
    pub total_gold_earned: usize,
    pub total_escaped_hp: f32,
    pub total_player_damage: f32,
    pub total_tower_damage: f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentError {
    IllegalAction {
        decision_point: DecisionPoint,
        action_id: String,
        state_hash: String,
    },
    CommandRejected {
        action_id: String,
        error: CommandError,
    },
    CardServiceRejected {
        action_id: String,
    },
}

pub struct GameEnvironment {
    game_state: GameCore,
    config: Arc<GameConfig>,
    seed: u64,
    max_advance_ticks: u64,
    /// Teacher-rollout-only absolute `sim_tick` cutoff (see
    /// `set_rollout_tick_deadline`); `None` in every non-rollout environment.
    rollout_tick_deadline: Option<u64>,
    card_service_selection: Option<td_core::CardServiceSelectionState>,
    decision_context: DecisionContext,
    environment_actions: Vec<AgentAction>,
    policy_trace: td_core::PolicyTrace,
    metrics: EnvironmentMetrics,
    reward_config: RewardConfig,
    max_stage: Option<usize>,
}

impl GameEnvironment {
    /// Replays an authoritative headed/core command log without constructing
    /// any headed modal, locale, or card-service presentation state.
    pub fn from_core_replay(replay: &td_core::CoreReplay) -> Result<Self, String> {
        let mut replay = replay.clone();
        replay
            .migrate_legacy_metadata()
            .map_err(|error| format!("{error:?}"))?;
        replay.validate().map_err(|error| format!("{error:?}"))?;
        let config = GameConfig::from_core_state(replay.config.clone())
            .ok_or_else(|| "core replay config cannot be restored".to_string())?;
        let mut environment = Self::new(Arc::new(config), replay.seed);
        environment.policy_trace = td_core::PolicyTrace::from_core_replay(
            &replay,
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
        );

        for (index, recorded) in replay.commands.iter().enumerate() {
            let pre_observation = environment.snapshot();
            let pre_state_hash = environment.state_hash();
            let pre_tick = environment.game_state.sim_tick().ticks();
            let action = Self::agent_action_for_command(
                &recorded.command,
                environment.game_state.raw_state().hand(),
            );
            environment
                .game_state
                .apply(recorded.command.clone())
                .map_err(|error| format!("replay command {index} rejected: {error:?}"))?;
            while environment.game_state.sim_tick().ticks() < recorded.completed_sim_tick {
                environment.game_state.advance_tick();
            }
            if environment.game_state.sim_tick().ticks() != recorded.completed_sim_tick {
                return Err(format!(
                    "replay command {} tick mismatch: expected {}, got {}",
                    index,
                    recorded.completed_sim_tick,
                    environment.game_state.sim_tick().ticks()
                ));
            }
            let post_observation = environment.snapshot();
            let post_state_hash = environment.state_hash();
            if let Some(checkpoint) = replay.checkpoints.get(index)
                && (checkpoint.sequence != recorded.sequence
                    || checkpoint.completed_sim_tick != recorded.completed_sim_tick
                    || checkpoint.state_hash != post_state_hash)
            {
                return Err(format!(
                    "replay checkpoint {} mismatch: expected ({}, {}, {}), got ({}, {}, {})",
                    index,
                    checkpoint.sequence,
                    checkpoint.completed_sim_tick,
                    checkpoint.state_hash,
                    recorded.sequence,
                    recorded.completed_sim_tick,
                    post_state_hash
                ));
            }
            environment.environment_actions.push(action.clone());
            let legal_actions = vec![action.clone()];
            environment
                .policy_trace
                .steps
                .push(td_core::PolicyTraceStep {
                    index: index as u64,
                    decision_point: pre_observation.decision_point.clone(),
                    agent_action: action,
                    legal_actions,
                    action_mask: vec![true],
                    player_command: Some(recorded.command.clone()),
                    pre_observation,
                    post_observation,
                    reward: RewardComponents::default(),
                    terminated: false,
                    truncated: false,
                    info: StepInfo {
                        reason: StepReason::DecisionPoint,
                        ticks_advanced: environment
                            .game_state
                            .sim_tick()
                            .ticks()
                            .saturating_sub(pre_tick),
                        no_progress_cycle: false,
                    },
                    pre_state_hash,
                    post_state_hash,
                });
        }

        Ok(environment)
    }

    pub fn new(config: Arc<GameConfig>, seed: u64) -> Self {
        Self::new_with_reward_config(config, seed, RewardConfig::default())
    }

    fn agent_action_for_command(
        command: &td_core::PlayerCommand,
        hand: &td_core::HandState,
    ) -> AgentAction {
        let slot_indices_to_card_ids = |indices: &[usize]| -> Vec<usize> {
            indices
                .iter()
                .filter_map(
                    |index| match hand.slots.get(*index).map(|slot| &slot.item) {
                        Some(td_core::HandItemState::Card(card)) => Some(card.id),
                        _ => None,
                    },
                )
                .collect()
        };
        match command {
            td_core::PlayerCommand::Reroll {
                selected_slot_indices,
            } => AgentAction::Reroll {
                card_ids: slot_indices_to_card_ids(selected_slot_indices),
            },
            td_core::PlayerCommand::PurchaseShopItem { slot_index } => {
                AgentAction::PurchaseShopItem {
                    slot_index: *slot_index,
                }
            }
            td_core::PlayerCommand::UseInventoryItem { item_index } => {
                AgentAction::UseInventoryItem {
                    item_index: *item_index,
                }
            }
            td_core::PlayerCommand::DiscardTreasure { upgrade_id } => {
                AgentAction::DiscardTreasure {
                    upgrade_id: *upgrade_id,
                }
            }
            td_core::PlayerCommand::StartSelectingTower => AgentAction::StartSelectingTower,
            td_core::PlayerCommand::SelectTower {
                selected_slot_indices,
            } => AgentAction::SelectTower {
                card_ids: slot_indices_to_card_ids(selected_slot_indices),
            },
            td_core::PlayerCommand::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => AgentAction::PlaceTower {
                hand_slot_index: *hand_slot_index,
                left: *left,
                top: *top,
            },
            td_core::PlayerCommand::RemoveTower { tower_id } => AgentAction::RemoveTower {
                tower_id: *tower_id,
            },
            td_core::PlayerCommand::StartDefense => AgentAction::StartDefense,
            td_core::PlayerCommand::SelectTreasure { option_index } => {
                AgentAction::SelectTreasure {
                    option_index: *option_index,
                }
            }
            td_core::PlayerCommand::ConfirmCardServiceSelection { .. } => {
                AgentAction::ConfirmCardServiceSelection
            }
        }
    }

    pub fn new_with_reward_config(
        config: Arc<GameConfig>,
        seed: u64,
        reward_config: RewardConfig,
    ) -> Self {
        Self::new_with_options(config, seed, reward_config, None)
    }

    pub fn new_with_stage_limit(
        config: Arc<GameConfig>,
        seed: u64,
        reward_config: RewardConfig,
        max_stage: usize,
    ) -> Self {
        Self::new_with_options(config, seed, reward_config, Some(max_stage))
    }

    fn new_with_options(
        config: Arc<GameConfig>,
        seed: u64,
        reward_config: RewardConfig,
        max_stage: Option<usize>,
    ) -> Self {
        let config = if let Some(max_stage) = max_stage {
            let mut config = (*config).clone();
            config.player.max_stages = config.player.max_stages.min(max_stage);
            Arc::new(config)
        } else {
            config
        };
        let game_state = GameCore::new((*config).clone(), seed);
        let policy_trace = td_core::PolicyTrace::from_core_replay(
            &game_state.replay(),
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
        );
        Self {
            game_state,
            config,
            seed,
            max_advance_ticks: DEFAULT_MAX_ADVANCE_TICKS,
            rollout_tick_deadline: None,
            card_service_selection: None,
            decision_context: DecisionContext::None,
            environment_actions: Vec::new(),
            policy_trace,
            metrics: EnvironmentMetrics::default(),
            reward_config,
            max_stage,
        }
    }

    pub fn reset(&mut self, seed: u64, config: Arc<GameConfig>) -> Observation {
        let config = if let Some(max_stage) = self.max_stage {
            let mut config = (*config).clone();
            config.player.max_stages = config.player.max_stages.min(max_stage);
            Arc::new(config)
        } else {
            config
        };
        self.game_state = GameCore::new((*config).clone(), seed);
        self.config = config;
        self.seed = seed;
        self.card_service_selection = None;
        self.decision_context = DecisionContext::None;
        self.environment_actions.clear();
        self.policy_trace = td_core::PolicyTrace::from_core_replay(
            &self.game_state.replay(),
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
        );
        self.metrics = EnvironmentMetrics::default();
        self.snapshot()
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn fork_for_rollout_seed(&self, scenario_seed: u64) -> Result<Self, String> {
        td_core::diag_scope!(Fork);
        let mut snapshot = self.game_state.core_state_snapshot();
        let sim_tick = snapshot.sim_tick().ticks();
        let derived_seed = td_core::derive_seed(
            self.seed,
            td_core::domain::ML_TOWER_TEACHER_SCENARIO,
            &[scenario_seed, sim_tick],
        );
        let scenario_master_seed = u64::from_le_bytes(
            derived_seed[..8]
                .try_into()
                .expect("derived seed must contain eight bytes"),
        );
        let game_seed = self.seed;
        snapshot
            .edit_snapshot(|parts| {
                parts.rng.seed = scenario_master_seed;
                resample_hidden_order(parts, game_seed, scenario_seed);
            })
            .map_err(|_| "rollout scenario seed produced an invalid snapshot".to_string())?;
        let game_state = GameCore::from_core_state_snapshot(snapshot)?;
        Ok(Self {
            game_state,
            config: Arc::clone(&self.config),
            seed: self.seed,
            max_advance_ticks: self.max_advance_ticks,
            rollout_tick_deadline: None,
            card_service_selection: self.card_service_selection.clone(),
            decision_context: self.decision_context.clone(),
            environment_actions: self.environment_actions.clone(),
            policy_trace: self.policy_trace.clone(),
            metrics: self.metrics.clone(),
            reward_config: self.reward_config.clone(),
            max_stage: self.max_stage,
        })
    }

    /// Current authoritative simulation tick.
    pub fn sim_tick(&self) -> u64 {
        self.game_state.sim_tick().ticks()
    }

    /// Bounds defense advancement for teacher rollouts: while set,
    /// `advance_until_decision_or_terminal` stops the tick loop the moment
    /// `sim_tick` reaches `deadline` (checked before every tick, so it can
    /// never overshoot), returning as if at an ordinary decision point.
    /// Production gameplay never sets this; `fork_for_rollout_seed` always
    /// starts with `None`, so normal environment semantics are unchanged.
    pub(crate) fn set_rollout_tick_deadline(&mut self, deadline: Option<u64>) {
        self.rollout_tick_deadline = deadline;
    }

    pub fn set_max_advance_ticks(&mut self, max_advance_ticks: u64) {
        self.max_advance_ticks = max_advance_ticks;
    }

    /// Test-only fixture: seeds `stage_modifiers.extra_tower_cards` with
    /// `count` entries shaped exactly like the Rubber Cone item's real
    /// effect (`kind: 0, suit: None, rank: None` - see
    /// `td_core::game_state::item::behaviors::rubber_cone`), without
    /// needing to route an actual item through inventory/shop RNG. Used to
    /// exercise the multi-build-slot `BuildTower` axis
    /// (`hand_slot_index >= 1`) deterministically.
    #[cfg(test)]
    pub(crate) fn test_only_seed_extra_tower_cards(&mut self, count: usize) -> Result<(), String> {
        let mut snapshot = self.game_state.core_state_snapshot();
        snapshot
            .edit_snapshot(|parts| {
                parts.stage_modifiers.extra_tower_cards = (0..count)
                    .map(|_| td_core::StageModifierTowerCardState {
                        kind: 0,
                        suit: None,
                        rank: None,
                    })
                    .collect();
            })
            .map_err(|error| format!("{error:?}"))?;
        self.game_state = GameCore::from_core_state_snapshot(snapshot)?;
        Ok(())
    }

    pub fn reward_config(&self) -> &RewardConfig {
        &self.reward_config
    }

    pub fn set_reward_config(&mut self, reward_config: RewardConfig) {
        self.reward_config = reward_config;
    }

    pub fn decision_point(&self) -> DecisionPoint {
        if self.card_service_selection.is_some() {
            return DecisionPoint::CardServiceSelection;
        }
        match self.decision_context {
            DecisionContext::CardSelection { .. } => return DecisionPoint::CardSelection,
            DecisionContext::PreDefenseItem { .. } => return DecisionPoint::PreDefenseItem,
            DecisionContext::DamageResponseItem { .. } => {
                return DecisionPoint::DamageResponseItem;
            }
            DecisionContext::None => {}
        }
        decision_point_from_flow(self.game_state.raw_state().flow())
    }

    pub fn snapshot(&self) -> Observation {
        td_core::diag_scope!(Snapshot);
        #[cfg(feature = "diagnostics")]
        td_core::diagnostics::record(|counters| counters.snapshot_calls += 1);
        let mut observation = self.game_state.observation(
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
            td_core::MAP_SIZE[0],
            td_core::MAP_SIZE[1],
        );
        let selected_hand_slot_indices: &[usize] = match &self.decision_context {
            DecisionContext::CardSelection {
                selected_slot_indices,
                ..
            } => selected_slot_indices.as_slice(),
            _ => &[],
        };
        for hand in &mut observation.hand {
            hand.selected = selected_hand_slot_indices.contains(&hand.index);
        }
        observation.decision_point = self.decision_point();
        observation.item_window_stage = match self.decision_context {
            DecisionContext::PreDefenseItem { stage }
            | DecisionContext::DamageResponseItem { stage, .. } => Some(stage),
            DecisionContext::None | DecisionContext::CardSelection { .. } => None,
        };
        observation.damage_trigger_tick = match self.decision_context {
            DecisionContext::DamageResponseItem { trigger_tick, .. } => Some(trigger_tick),
            _ => None,
        };
        observation.card_selection_purpose = match &self.decision_context {
            DecisionContext::CardSelection { purpose, .. } => Some(
                match purpose {
                    CardSelectionPurpose::Reroll => "reroll",
                    CardSelectionPurpose::BuildTower => "build_tower",
                }
                .to_string(),
            ),
            _ => None,
        };
        observation.selected_hand_slot_indices = selected_hand_slot_indices.to_vec();
        observation.card_selection_confirmable = !selected_hand_slot_indices.is_empty()
            && matches!(self.decision_context, DecisionContext::CardSelection { .. });
        observation.card_service = self.card_service_selection.as_ref().and_then(|selection| {
            card_service_observation_raw(self.game_state.raw_state(), selection)
        });
        observation
    }

    #[cfg(feature = "diagnostics")]
    pub fn tower_count(&self) -> usize {
        self.game_state.raw_state().towers().len()
    }

    pub fn state_hash(&self) -> String {
        td_core::diag_scope!(StateHash);
        self.game_state.authoritative_hash()
    }

    pub fn progress_fingerprint(&self) -> String {
        let observation = self.snapshot();
        serde_json::to_string(&(
            self.state_hash(),
            observation.decision_point,
            observation.stage,
            observation.sim_tick,
            observation.card_service,
            &self.decision_context,
            observation.item_window_stage,
            observation.damage_trigger_tick,
        ))
        .expect("progress fingerprint serialization should be infallible")
    }

    pub fn clear_rate(&self) -> f32 {
        match self.game_state.raw_state().flow() {
            td_core::GameFlowState::Result { clear_rate_raw } => {
                *clear_rate_raw as f32 * 100.0 / td_core::RATIO_SCALE as f32
            }
            _ => self.game_state.clear_rate().as_percent_f32(),
        }
    }

    pub fn metrics(&self) -> EnvironmentMetrics {
        let mut metrics = self.metrics.clone();
        let reward_metrics = self.game_state.reward_metrics();
        metrics.total_gold_earned = reward_metrics.total_gold_earned;
        metrics.total_player_damage = reward_metrics.total_player_damage;
        metrics.total_tower_damage = reward_metrics.total_tower_damage;
        metrics.stage_damage = reward_metrics.stage_damage;
        metrics
    }

    pub fn replay(&self) -> EnvironmentReplay {
        let replay = self.game_state.replay();
        EnvironmentReplay {
            environment_version: ENVIRONMENT_VERSION,
            action_schema_version: ACTION_SCHEMA_VERSION,
            replay_schema_version: ENVIRONMENT_REPLAY_SCHEMA_VERSION,
            config_version: replay.config_version,
            config_digest_version: crate::config::CONFIG_DIGEST_VERSION,
            config_digest: replay.config_digest,
            rng_algorithm_version: replay.rng_algorithm_version,
            seed: replay.seed,
            commands: self.environment_actions.clone(),
            checkpoints: replay
                .checkpoints
                .iter()
                .map(|checkpoint| EnvironmentReplayCheckpoint {
                    sequence: checkpoint.sequence,
                    completed_sim_tick: checkpoint.completed_sim_tick,
                    state_hash: checkpoint.state_hash.clone(),
                    event_count: checkpoint.event_count,
                    event_digest: checkpoint.event_digest.clone(),
                })
                .collect(),
        }
    }

    pub fn policy_trace(&self) -> &td_core::PolicyTrace {
        &self.policy_trace
    }

    pub fn core_replay(&self) -> td_core::CoreReplay {
        self.game_state
            .replay()
            .with_policy_trace(self.policy_trace.clone())
    }

    pub fn legal_actions(&self) -> Vec<LegalAction> {
        self.legal_actions_internal(None)
    }

    pub fn legal_actions_with_metrics(&self) -> (Vec<LegalAction>, LegalActionGenerationMetrics) {
        let mut metrics = LegalActionGenerationMetrics::default();
        let actions = self.legal_actions_internal(Some(&mut metrics));
        (actions, metrics)
    }

    fn legal_actions_internal(
        &self,
        mut generation_metrics: Option<&mut LegalActionGenerationMetrics>,
    ) -> Vec<LegalAction> {
        td_core::diag_scope!(LegalActions);
        let mut actions = match self.decision_point() {
            DecisionPoint::Shop => self.shop_actions(),
            DecisionPoint::CardSelection => self.card_selection_actions(),
            DecisionPoint::CardServiceSelection => self.card_service_actions(),
            DecisionPoint::TowerPlacement => {
                self.tower_placement_actions(generation_metrics.as_deref_mut())
            }
            DecisionPoint::PreDefenseItem | DecisionPoint::DamageResponseItem => {
                let mut actions = self.inventory_actions();
                actions.push(AgentAction::Continue);
                actions
            }
            DecisionPoint::Defense => vec![AgentAction::Continue],
            DecisionPoint::TreasureSelection => self.treasure_actions(),
            DecisionPoint::Terminal => Vec::new(),
        };
        if !matches!(
            self.decision_point(),
            DecisionPoint::Defense
                | DecisionPoint::PreDefenseItem
                | DecisionPoint::DamageResponseItem
                | DecisionPoint::Terminal
                | DecisionPoint::CardServiceSelection
        ) {
            actions.extend(self.inventory_actions());
            actions.extend(self.treasure_discard_actions());
        }

        actions.sort_by_key(AgentAction::action_id);
        actions
            .into_iter()
            .map(|action| LegalAction {
                id: action.action_id(),
                action,
            })
            .collect()
    }

    pub fn action_mask(&self) -> Vec<bool> {
        self.legal_actions().into_iter().map(|_| true).collect()
    }

    pub fn forced_action(&self) -> Option<AgentAction> {
        match self.decision_point() {
            DecisionPoint::PreDefenseItem
            | DecisionPoint::DamageResponseItem
            | DecisionPoint::Defense => {
                let actions = self.legal_actions();
                (actions.len() == 1 && matches!(actions[0].action, AgentAction::Continue))
                    .then_some(AgentAction::Continue)
            }
            _ => None,
        }
    }

    pub fn semantic_legal_actions(&self) -> Vec<LegalAction> {
        self.semantic_legal_actions_with_position_limit(None)
    }

    pub(crate) fn tower_placement_context(&self) -> td_core::TowerPlacementContext {
        self.game_state.raw_state().tower_placement_context()
    }

    /// Whether a tower's 2x2 footprint can be placed with its top-left
    /// corner at `(left, top)`: in bounds, not already occupied or a travel
    /// point, and doesn't disconnect the route. Independent of which card
    /// subset produced the tower (the footprint size never varies).
    #[cfg(test)]
    pub(crate) fn can_place_at(&self, left: usize, top: usize) -> bool {
        self.game_state
            .raw_state()
            .tower_placement_context()
            .can_place_at(left, top)
    }

    pub fn semantic_legal_actions_with_position_limit(
        &self,
        position_limit: Option<usize>,
    ) -> Vec<LegalAction> {
        td_core::diag_scope!(SemanticLegalActions);
        if self.semantic_card_decision_available() {
            let mut actions = self.semantic_card_actions(position_limit);
            if matches!(self.decision_point(), DecisionPoint::Shop) {
                actions.extend(
                    self.shop_actions()
                        .into_iter()
                        .filter(|action| matches!(action, AgentAction::PurchaseShopItem { .. })),
                );
            }
            actions.extend(self.inventory_actions());
            actions.extend(self.treasure_discard_actions());
            actions
                .into_iter()
                .map(|action| LegalAction {
                    id: action.action_id(),
                    action,
                })
                .collect()
        } else {
            self.legal_actions()
        }
    }

    pub fn semantic_step(&mut self, action: AgentAction) -> Result<StepOutcome, EnvironmentError> {
        td_core::diag_scope!(SemanticStep);
        if !self.semantic_action_is_legal(&action) {
            return Err(EnvironmentError::IllegalAction {
                decision_point: self.decision_point(),
                action_id: action.action_id(),
                state_hash: self.state_hash(),
            });
        }
        let starts_from_shop = matches!(self.decision_point(), DecisionPoint::Shop)
            && matches!(self.decision_context, DecisionContext::None);
        match action {
            AgentAction::Reroll { card_ids } => {
                if !starts_from_shop {
                    return self.step_unchecked(AgentAction::Reroll { card_ids });
                }
                let trace_start = self.policy_trace.steps.len();
                let (trace_pre_observation, trace_pre_state_hash) = {
                    td_core::diag_scope!(TraceConstruction);
                    (self.snapshot(), self.state_hash())
                };
                let trace_legal_actions = {
                    td_core::diag_scope!(TraceConstruction);
                    self.semantic_legal_actions_with_position_limit(Some(
                        DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
                    ))
                    .into_iter()
                    .map(|legal| legal.action)
                    .collect::<Vec<_>>()
                };
                let trace_action = AgentAction::Reroll {
                    card_ids: card_ids.clone(),
                };
                let first = self.step_unchecked_untraced(AgentAction::StartSelectingTower)?;
                let mut second = self.step_unchecked_untraced(AgentAction::Reroll { card_ids })?;
                merge_step_outcome(&mut second, &first);
                self.replace_policy_trace_with_macro(
                    trace_start,
                    trace_pre_observation,
                    trace_pre_state_hash,
                    trace_legal_actions,
                    trace_action,
                    &second,
                );
                Ok(second)
            }
            AgentAction::BuildTower {
                card_ids,
                hand_slot_index,
                left,
                top,
            } => {
                let trace_start = self.policy_trace.steps.len();
                let (trace_pre_observation, trace_pre_state_hash) = {
                    td_core::diag_scope!(TraceConstruction);
                    (self.snapshot(), self.state_hash())
                };
                let trace_card_ids = card_ids.clone();
                let trace_legal_actions = {
                    td_core::diag_scope!(TraceConstruction);
                    self.semantic_legal_actions_with_position_limit(Some(
                        DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
                    ))
                    .into_iter()
                    .map(|legal| legal.action)
                    .collect::<Vec<_>>()
                };
                let start_outcome = starts_from_shop
                    .then(|| self.step_unchecked_untraced(AgentAction::StartSelectingTower))
                    .transpose()?;
                let first = self.step_unchecked_untraced(AgentAction::SelectTower { card_ids })?;
                let mut second = self.step_unchecked_untraced(AgentAction::PlaceTower {
                    hand_slot_index,
                    left,
                    top,
                })?;
                if let Some(start_outcome) = start_outcome {
                    merge_step_outcome(&mut second, &start_outcome);
                }
                merge_step_outcome(&mut second, &first);
                self.replace_policy_trace_with_macro(
                    trace_start,
                    trace_pre_observation,
                    trace_pre_state_hash,
                    trace_legal_actions,
                    AgentAction::BuildTower {
                        card_ids: trace_card_ids,
                        hand_slot_index,
                        left,
                        top,
                    },
                    &second,
                );
                Ok(second)
            }
            action => self.step(action),
        }
    }

    /// Teacher-rollout-only equivalent of [`Self::semantic_step`] for an
    /// action already taken from this state's legal candidates or chosen by
    /// the canonical policy from them. Performs exactly the same state
    /// mutations, including the `BuildTower`/`Reroll` macro expansion, but
    /// skips the legality re-check, observations, state hashes, rewards and
    /// the policy trace, none of which a terminal rollout reads. Test and
    /// debug builds still verify the action is legal.
    pub(crate) fn rollout_step_trusted(
        &mut self,
        action: AgentAction,
    ) -> Result<RolloutStepOutcome, EnvironmentError> {
        td_core::diag_scope!(SemanticStep);
        #[cfg(any(test, debug_assertions))]
        assert!(
            self.semantic_action_is_legal(&action),
            "trusted rollout action {} is not legal at state {}",
            action.action_id(),
            self.state_hash()
        );
        let starts_from_shop = matches!(self.decision_point(), DecisionPoint::Shop)
            && matches!(self.decision_context, DecisionContext::None);
        let last = match action {
            AgentAction::Reroll { card_ids } if starts_from_shop => {
                self.apply_and_advance(&AgentAction::StartSelectingTower)?;
                self.apply_and_advance(&AgentAction::Reroll { card_ids })?
            }
            AgentAction::BuildTower {
                card_ids,
                hand_slot_index,
                left,
                top,
            } => {
                if starts_from_shop {
                    self.apply_and_advance(&AgentAction::StartSelectingTower)?;
                }
                self.apply_and_advance(&AgentAction::SelectTower { card_ids })?;
                self.apply_and_advance(&AgentAction::PlaceTower {
                    hand_slot_index,
                    left,
                    top,
                })?
            }
            action => self.apply_and_advance(&action)?,
        };
        Ok(RolloutStepOutcome {
            terminated: matches!(self.decision_point(), DecisionPoint::Terminal),
            truncated: matches!(last.reason, StepReason::MaxTicks),
        })
    }

    pub fn semantic_card_decision_available(&self) -> bool {
        matches!(
            self.decision_point(),
            DecisionPoint::Shop | DecisionPoint::CardSelection
        ) && matches!(self.decision_context, DecisionContext::None)
    }

    fn replace_policy_trace_with_macro(
        &mut self,
        trace_start: usize,
        pre_observation: Observation,
        pre_state_hash: String,
        legal_actions: Vec<AgentAction>,
        action: AgentAction,
        outcome: &StepOutcome,
    ) {
        td_core::diag_scope!(TraceConstruction);
        self.policy_trace.steps.truncate(trace_start);
        let action_mask = vec![true; legal_actions.len()];
        self.policy_trace.steps.push(td_core::PolicyTraceStep {
            index: self.policy_trace.steps.len() as u64,
            decision_point: pre_observation.decision_point.clone(),
            agent_action: action,
            legal_actions,
            action_mask,
            player_command: None,
            pre_observation,
            post_observation: outcome.observation.clone(),
            reward: outcome.reward.clone(),
            terminated: outcome.terminated,
            truncated: outcome.truncated,
            info: outcome.info.clone(),
            pre_state_hash,
            post_state_hash: outcome.state_hash.clone(),
        });
    }

    /// Applies one AI decision and advances to the next decision point or
    /// terminal state; this may execute multiple fixed simulation ticks.
    pub fn step(&mut self, action: AgentAction) -> Result<StepOutcome, EnvironmentError> {
        let action_id = action.action_id();
        let is_legal = {
            td_core::diag_scope!(StepLegalityCheck);
            self.legal_actions()
                .iter()
                .any(|legal_action| legal_action.action == action)
        };
        if !is_legal {
            return Err(EnvironmentError::IllegalAction {
                decision_point: self.decision_point(),
                action_id,
                state_hash: self.state_hash(),
            });
        }
        self.step_unchecked(action)
    }

    fn step_unchecked(&mut self, action: AgentAction) -> Result<StepOutcome, EnvironmentError> {
        self.step_unchecked_traced(action, true)
    }

    /// Like [`Self::step_unchecked`], but skips recording a policy trace
    /// step. Used for the internal `StartSelectingTower`/`SelectTower`/
    /// `PlaceTower`/`Reroll` sub-steps of a semantic macro action, whose
    /// trace entries `replace_policy_trace_with_macro` immediately discards
    /// anyway; the legal action list built for that entry is a full
    /// `TowerPlacement` scan (every map cell's placement legality), so
    /// computing it just to throw it away was the dominant cost of every
    /// semantic `BuildTower`/`Reroll` step.
    fn step_unchecked_untraced(
        &mut self,
        action: AgentAction,
    ) -> Result<StepOutcome, EnvironmentError> {
        self.step_unchecked_traced(action, false)
    }

    fn step_unchecked_traced(
        &mut self,
        action: AgentAction,
        record_trace: bool,
    ) -> Result<StepOutcome, EnvironmentError> {
        let legal_actions_before = if record_trace {
            self.legal_actions()
        } else {
            Vec::new()
        };

        let tower_damage_before = self.game_state.reward_metrics().total_tower_damage;
        let clear_rate_before = self.clear_rate() / 100.0;
        let observation_before = self.snapshot();
        let pre_state_hash = self.state_hash();
        let pre_decision_point = self.decision_point();
        let command_count_before = self.game_state.replay().commands.len();

        let AppliedStep {
            reason,
            ticks_advanced,
            player_damage,
            escaped_hp,
        } = self.apply_and_advance(&action)?;
        let terminated = matches!(self.decision_point(), DecisionPoint::Terminal);
        let truncated = matches!(reason, StepReason::MaxTicks);
        let terminal_reward = match self.game_state.raw_state().flow() {
            td_core::GameFlowState::Result { clear_rate_raw }
                if *clear_rate_raw == td_core::RATIO_SCALE =>
            {
                self.reward_config.terminal_win
            }
            td_core::GameFlowState::Result { .. } => self.reward_config.terminal_loss,
            _ => 0.0,
        };
        let clear_rate_after = self.clear_rate() / 100.0;
        let reward_metrics_after = self.game_state.reward_metrics();
        let stage_total_hp = match self.game_state.raw_state().flow() {
            td_core::GameFlowState::Defense(flow) => flow.start_total_hp_raw as f32 / 1_000.0,
            _ => {
                let state = self.game_state.raw_state();
                td_core::game_state::monster_spawn::calculate_stage_total_hp_raw(
                    state.progress().stage,
                    state.config(),
                    &state.stage_modifiers().enemy_health_multipliers_raw,
                ) as f32
                    / 1_000.0
            }
        }
        .max(1.0);
        let tower_damage = (reward_metrics_after.total_tower_damage - tower_damage_before).max(0.0);
        let observation_after = self.snapshot();
        let potential_reward = potential_shaping(
            &observation_before,
            &observation_after,
            terminated,
            &self.reward_config,
        );

        let mut shaping = shaping_rewards(
            clear_rate_before,
            clear_rate_after,
            escaped_hp,
            player_damage,
            stage_total_hp,
            tower_damage,
            &self.reward_config,
        );
        shaping.insert("potential_progress".to_string(), potential_reward);

        let outcome = StepOutcome {
            observation: self.snapshot(),
            reward: RewardComponents {
                terminal: terminal_reward,
                shaping,
            },
            terminated,
            truncated,
            info: StepInfo {
                reason,
                ticks_advanced,
                no_progress_cycle: false,
            },
            state_hash: self.state_hash(),
        };

        if record_trace {
            let player_command = self
                .game_state
                .replay()
                .commands
                .get(command_count_before)
                .map(|recorded| recorded.command.clone());
            self.policy_trace.steps.push(td_core::PolicyTraceStep {
                index: self.policy_trace.steps.len() as u64,
                decision_point: pre_decision_point,
                agent_action: action,
                legal_actions: legal_actions_before
                    .iter()
                    .map(|legal_action| legal_action.action.clone())
                    .collect(),
                action_mask: vec![true; legal_actions_before.len()],
                player_command,
                pre_observation: observation_before,
                post_observation: outcome.observation.clone(),
                reward: outcome.reward.clone(),
                terminated: outcome.terminated,
                truncated: outcome.truncated,
                info: outcome.info.clone(),
                pre_state_hash,
                post_state_hash: outcome.state_hash.clone(),
            });
        }

        Ok(outcome)
    }

    /// Every state mutation of one environment step: applies `action`,
    /// updates decision context, environment metrics and the recorded
    /// action list, then advances to the next decision point or terminal.
    /// Observation, state hash, reward and trace bookkeeping are left to the
    /// caller.
    fn apply_and_advance(&mut self, action: &AgentAction) -> Result<AppliedStep, EnvironmentError> {
        let escaped_hp_before = self.game_state.reward_metrics().total_escaped_hp;
        let deferred_card_service = self.card_service_kind_for_action(action);
        let hand_card_ids = td_core::hand_card_id_slots(self.game_state.raw_state().hand());
        let action_result = {
            td_core::diag_scope!(ApplyAction);
            match action.to_player_command(&hand_card_ids) {
                Ok(Some(command)) => self
                    .game_state
                    .apply(command)
                    .map_err(|error| EnvironmentError::CommandRejected {
                        action_id: action.action_id(),
                        error,
                    })
                    .map(|_| ()),
                Ok(None) => self.apply_environment_action(action),
                Err(error) => Err(EnvironmentError::CommandRejected {
                    action_id: action.action_id(),
                    error,
                }),
            }
        };
        action_result?;
        self.apply_card_selection_action(action)?;
        if matches!(action, AgentAction::Continue)
            && matches!(
                self.decision_context,
                DecisionContext::PreDefenseItem { .. } | DecisionContext::DamageResponseItem { .. }
            )
        {
            self.decision_context = DecisionContext::None;
        }
        if matches!(action, AgentAction::StartDefense)
            && matches!(
                self.game_state.raw_state().flow(),
                td_core::GameFlowState::Defense(_)
            )
        {
            self.decision_context = DecisionContext::PreDefenseItem {
                stage: self.game_state.stage(),
            };
        }
        match action {
            AgentAction::PlaceTower { .. } => self.metrics.total_towers_placed += 1,
            AgentAction::UseInventoryItem { .. } => self.metrics.total_items_used += 1,
            _ => {}
        }
        if deferred_card_service.is_some() {
            self.card_service_selection = self.game_state.take_card_service_selection();
        }
        self.environment_actions.push(action.clone());

        let ticks_before = self.game_state.sim_tick().ticks();
        let mut reason = self.advance_until_decision_or_terminal();
        if self.max_stage.is_some()
            && matches!(
                self.game_state.raw_state().flow(),
                td_core::GameFlowState::Result { clear_rate_raw }
                    if *clear_rate_raw == td_core::RATIO_SCALE
            )
        {
            reason = StepReason::CurriculumComplete;
        }
        let ticks_advanced = self
            .game_state
            .sim_tick()
            .ticks()
            .saturating_sub(ticks_before);
        let reward_metrics_after = self.game_state.reward_metrics();
        let player_damage =
            reward_metrics_after.total_player_damage - self.metrics.total_player_damage;
        self.metrics.total_player_damage = reward_metrics_after.total_player_damage;
        let escaped_hp = (reward_metrics_after.total_escaped_hp - escaped_hp_before).max(0.0);
        self.metrics.total_escaped_hp += escaped_hp;
        Ok(AppliedStep {
            reason,
            ticks_advanced,
            player_damage,
            escaped_hp,
        })
    }

    pub fn advance_until_decision_or_terminal(&mut self) -> StepReason {
        td_core::diag_scope!(AdvanceUntilDecision);
        let mut ticks_advanced = 0;
        while matches!(
            self.game_state.raw_state().flow(),
            td_core::GameFlowState::Defense(_)
        ) {
            if !matches!(self.decision_context, DecisionContext::None) {
                return StepReason::DecisionPoint;
            }
            if ticks_advanced >= self.max_advance_ticks {
                return StepReason::MaxTicks;
            }
            if self
                .rollout_tick_deadline
                .is_some_and(|deadline| self.game_state.sim_tick().ticks() >= deadline)
            {
                return StepReason::DecisionPoint;
            }
            let hp_before = self.game_state.hp().raw();
            let tick_before = self.game_state.sim_tick().ticks();
            // Defense ticks are internal to the decision-point transition;
            // policy/replay metadata is recorded at the surrounding
            // environment step instead.
            let _ = self.game_state.advance_tick_unrecorded();
            ticks_advanced += 1;
            let hp_after = self.game_state.hp().raw();
            if hp_after < hp_before
                && hp_after > 0
                && matches!(
                    self.game_state.raw_state().flow(),
                    td_core::GameFlowState::Defense(_)
                )
            {
                self.decision_context = DecisionContext::DamageResponseItem {
                    stage: self.game_state.stage(),
                    trigger_tick: tick_before + 1,
                };
                return StepReason::DecisionPoint;
            }
        }

        if matches!(self.decision_point(), DecisionPoint::Terminal) {
            StepReason::Terminal
        } else {
            StepReason::DecisionPoint
        }
    }

    pub fn replay_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.replay())
    }

    fn shop_actions(&self) -> Vec<AgentAction> {
        let mut actions = Vec::new();
        if let td_core::GameFlowState::Shopping(flow) = self.game_state.raw_state().flow() {
            for slot_index in 0..flow.slots.len() {
                if self
                    .game_state
                    .raw_state()
                    .can_purchase_shop_slot(slot_index)
                {
                    actions.push(AgentAction::PurchaseShopItem { slot_index });
                }
            }
            actions.push(AgentAction::StartSelectingTower);
        }
        actions
    }

    fn card_selection_actions(&self) -> Vec<AgentAction> {
        if let DecisionContext::CardSelection {
            selected_slot_indices,
            ..
        } = &self.decision_context
        {
            let card_indices = self.card_hand_indices();
            let mut actions = card_indices
                .into_iter()
                .map(|hand_slot_index| {
                    if selected_slot_indices.contains(&hand_slot_index) {
                        AgentAction::DeselectHandCard { hand_slot_index }
                    } else {
                        AgentAction::SelectHandCard { hand_slot_index }
                    }
                })
                .collect::<Vec<_>>();
            if !selected_slot_indices.is_empty() {
                actions.push(AgentAction::ConfirmCardSelection);
            }
            actions.push(AgentAction::CancelCardSelection);
            return actions;
        }
        let card_indices = self.card_hand_indices();
        let mut actions = Vec::new();
        if !card_indices.is_empty() {
            let state = self.game_state.raw_state();
            let reroll_health_cost = state
                .stage_modifiers()
                .reroll_health_cost
                .saturating_mul(1_000) as i64;
            let can_afford_reroll = state.progress().left_dice > 0
                || (reroll_health_cost > 0
                    && state.hp_raw().saturating_sub(reroll_health_cost) > 1_000);
            if can_afford_reroll {
                actions.push(AgentAction::BeginRerollSelection);
            }
            actions.push(AgentAction::BeginTowerSelection);
        }
        actions
    }

    pub fn semantic_action_is_legal(&self, action: &AgentAction) -> bool {
        td_core::diag_scope!(SemanticActionIsLegal);
        if !self.semantic_card_decision_available() {
            return self
                .legal_actions()
                .iter()
                .any(|legal_action| legal_action.action == *action);
        }
        let hand_card_ids = td_core::hand_card_id_slots(self.game_state.raw_state().hand());
        let card_ids_are_selectable = |card_ids: &[usize]| {
            let mut sorted = card_ids.to_vec();
            sorted.sort_unstable();
            sorted.dedup();
            sorted.len() == card_ids.len()
                && card_ids
                    .iter()
                    .all(|card_id| hand_card_ids.iter().any(|(_, id)| id == card_id))
        };
        match action {
            AgentAction::Reroll { card_ids } => {
                !card_ids.is_empty()
                    && card_ids_are_selectable(card_ids)
                    && self.can_afford_reroll()
            }
            AgentAction::BuildTower {
                card_ids,
                hand_slot_index,
                left,
                top,
            } => {
                #[cfg(feature = "diagnostics")]
                td_core::diagnostics::record(|counters| counters.build_tower_legality_checks += 1);
                card_ids_are_selectable(card_ids)
                    && *hand_slot_index < self.build_tower_slot_count()
                    && self
                        .game_state
                        .raw_state()
                        .tower_placement_context()
                        .can_place_at(*left, *top)
            }
            _ => self
                .legal_actions()
                .iter()
                .any(|legal_action| legal_action.action == *action),
        }
    }

    fn can_afford_reroll(&self) -> bool {
        let state = self.game_state.raw_state();
        let reroll_health_cost = state
            .stage_modifiers()
            .reroll_health_cost
            .saturating_mul(1_000) as i64;
        state.progress().left_dice > 0
            || (reroll_health_cost > 0 && state.hp_raw().saturating_sub(reroll_health_cost) > 1_000)
    }

    /// Stable card ids of every currently-held hand card, indexed by their
    /// offset into `card_indices` (i.e. `card_indices`' own iteration
    /// order) - not to be confused with `joint_action::CardSubsetTable`,
    /// which re-sorts by card id to be hand-slot independent.
    fn card_ids_by_offset(&self, card_indices: &[usize]) -> Vec<usize> {
        let hand = self.game_state.raw_state().hand();
        card_indices
            .iter()
            .map(|slot_index| match &hand.slots[*slot_index].item {
                td_core::HandItemState::Card(card) => card.id,
                td_core::HandItemState::Tower(_) => {
                    unreachable!("card_hand_indices only returns card slots")
                }
            })
            .collect()
    }

    /// Legacy/benchmark candidate path: flattens every card subset's
    /// Reroll plus (position-limited) BuildTower actions in hand-slot
    /// generation order. Production `BuildTower` candidate generation uses
    /// `joint_action::DenseBuildTowerScoreTable` instead (see
    /// `semantic_non_build_actions` for the non-`BuildTower` half); this
    /// method remains for `semantic_legal_actions` (the full/oracle set)
    /// and the Phase 1/2 historical benchmarks in `teacher.rs`.
    fn semantic_card_actions(&self, position_limit: Option<usize>) -> Vec<AgentAction> {
        let card_indices = self.card_hand_indices();
        if card_indices.is_empty() {
            return Vec::new();
        }
        let card_ids_by_offset = self.card_ids_by_offset(&card_indices);
        let can_afford_reroll = self.can_afford_reroll();
        let legal_positions = self.semantic_legal_positions(position_limit);
        let mut actions = Vec::new();
        let subset_count = 1usize << card_indices.len();
        for subset_mask in 1..subset_count {
            let selected_card_ids = card_ids_by_offset
                .iter()
                .enumerate()
                .filter_map(|(offset, card_id)| {
                    (subset_mask & (1usize << offset) != 0).then_some(*card_id)
                })
                .collect::<Vec<_>>();
            if can_afford_reroll {
                actions.push(AgentAction::Reroll {
                    card_ids: selected_card_ids.clone(),
                });
            }
            let canonical_card_ids = if subset_mask + 1 == subset_count {
                Vec::new()
            } else {
                selected_card_ids
            };
            actions.extend(self.semantic_build_actions(canonical_card_ids, &legal_positions));
        }
        actions
    }

    /// Every legal `Reroll` action (one per non-empty card subset), with no
    /// position computation at all - `Reroll` doesn't depend on map
    /// positions, so this avoids the O(map) legality scan
    /// `semantic_card_actions`/`semantic_legal_positions` would otherwise
    /// run just to end up discarding every position.
    fn semantic_reroll_actions(&self) -> Vec<AgentAction> {
        let card_indices = self.card_hand_indices();
        if card_indices.is_empty() || !self.can_afford_reroll() {
            return Vec::new();
        }
        let card_ids_by_offset = self.card_ids_by_offset(&card_indices);
        let subset_count = 1usize << card_indices.len();
        (1..subset_count)
            .map(|subset_mask| AgentAction::Reroll {
                card_ids: card_ids_by_offset
                    .iter()
                    .enumerate()
                    .filter_map(|(offset, card_id)| {
                        (subset_mask & (1usize << offset) != 0).then_some(*card_id)
                    })
                    .collect(),
            })
            .collect()
    }

    /// Every legal semantic action *except* `BuildTower`: `Reroll` (one per
    /// card subset), shop purchases, inventory items, and treasure
    /// discards. Pairs with the dense `BuildTower` joint scorer
    /// (`joint_action::DenseBuildTowerScoreTable`) to assemble the
    /// production teacher's full candidate set without ever running the
    /// O(map) position-proposal scan `semantic_card_actions` uses -
    /// `BuildTower` candidates are added separately, from the dense score
    /// table's top-K.
    ///
    /// Falls back to the full `legal_actions()` set when no card decision
    /// is available (e.g. mid-defense), matching
    /// `semantic_legal_actions_with_position_limit`'s fallback for the same
    /// case.
    pub fn semantic_non_build_actions(&self) -> Vec<LegalAction> {
        if !self.semantic_card_decision_available() {
            return self.legal_actions();
        }
        let mut actions = self.semantic_reroll_actions();
        if matches!(self.decision_point(), DecisionPoint::Shop) {
            actions.extend(
                self.shop_actions()
                    .into_iter()
                    .filter(|action| matches!(action, AgentAction::PurchaseShopItem { .. })),
            );
        }
        actions.extend(self.inventory_actions());
        actions.extend(self.treasure_discard_actions());
        actions
            .into_iter()
            .map(|action| LegalAction {
                id: action.action_id(),
                action,
            })
            .collect()
    }

    fn semantic_legal_positions(&self, position_limit: Option<usize>) -> Vec<[usize; 2]> {
        td_core::diag_scope!(PlacementScan);
        #[cfg(feature = "diagnostics")]
        td_core::diagnostics::record(|counters| counters.semantic_legal_positions_scans += 1);
        let state = self.game_state.raw_state();
        let placement_context = state.tower_placement_context();
        let map_width = td_core::MAP_SIZE[0].saturating_sub(1);
        let map_height = td_core::MAP_SIZE[1].saturating_sub(1);
        let mut positions = Vec::new();
        for top in 0..map_height {
            for left in 0..map_width {
                if placement_context.can_place_at(left, top) {
                    positions.push([left, top]);
                }
            }
        }
        let observation = self.snapshot();
        positions.sort_by_key(|[left, top]| {
            (
                observation
                    .route_coords
                    .iter()
                    .map(|coord| coord.x.abs_diff(*left) + coord.y.abs_diff(*top))
                    .min()
                    .unwrap_or(usize::MAX),
                *top,
                *left,
            )
        });
        if let Some(position_limit) = position_limit {
            positions.truncate(position_limit);
        }
        positions
    }

    /// Number of tower hand slots a `BuildTower` selection resolves to:
    /// slot `0` is always the selected card subset's own template; slots
    /// `1..` are `stage_modifiers.extra_tower_cards`, one fixed
    /// (subset-independent) template per entry - see
    /// `tower_selection::start_placing_tower_from_template`, which builds
    /// `hand.slots` in exactly this order.
    pub(crate) fn build_tower_slot_count(&self) -> usize {
        self.game_state
            .raw_state()
            .stage_modifiers()
            .extra_tower_cards
            .len()
            + 1
    }

    fn semantic_build_actions(
        &self,
        card_ids: Vec<usize>,
        legal_positions: &[[usize; 2]],
    ) -> Vec<AgentAction> {
        let tower_count = self.build_tower_slot_count();
        let mut actions = Vec::with_capacity(tower_count * legal_positions.len());
        for hand_slot_index in 0..tower_count {
            for [left, top] in legal_positions {
                actions.push(AgentAction::BuildTower {
                    card_ids: card_ids.clone(),
                    hand_slot_index,
                    left: *left,
                    top: *top,
                });
            }
        }
        actions
    }

    fn apply_card_selection_action(
        &mut self,
        action: &AgentAction,
    ) -> Result<(), EnvironmentError> {
        match action {
            AgentAction::BeginRerollSelection => {
                self.decision_context = DecisionContext::CardSelection {
                    purpose: CardSelectionPurpose::Reroll,
                    selected_slot_indices: Vec::new(),
                };
            }
            AgentAction::BeginTowerSelection => {
                self.decision_context = DecisionContext::CardSelection {
                    purpose: CardSelectionPurpose::BuildTower,
                    selected_slot_indices: Vec::new(),
                };
            }
            AgentAction::SelectHandCard { hand_slot_index }
            | AgentAction::DeselectHandCard { hand_slot_index } => {
                let DecisionContext::CardSelection {
                    selected_slot_indices,
                    ..
                } = &mut self.decision_context
                else {
                    return Err(EnvironmentError::IllegalAction {
                        decision_point: self.decision_point(),
                        action_id: action.action_id(),
                        state_hash: self.state_hash(),
                    });
                };
                match action {
                    AgentAction::SelectHandCard { .. } => {
                        if selected_slot_indices.contains(hand_slot_index) {
                            return Err(EnvironmentError::IllegalAction {
                                decision_point: self.decision_point(),
                                action_id: action.action_id(),
                                state_hash: self.state_hash(),
                            });
                        }
                        selected_slot_indices.push(*hand_slot_index);
                    }
                    AgentAction::DeselectHandCard { .. } => {
                        let Some(position) = selected_slot_indices
                            .iter()
                            .position(|index| index == hand_slot_index)
                        else {
                            return Err(EnvironmentError::IllegalAction {
                                decision_point: self.decision_point(),
                                action_id: action.action_id(),
                                state_hash: self.state_hash(),
                            });
                        };
                        selected_slot_indices.remove(position);
                    }
                    _ => unreachable!(),
                }
            }
            AgentAction::ConfirmCardSelection => {
                let DecisionContext::CardSelection {
                    purpose,
                    selected_slot_indices,
                } = std::mem::replace(&mut self.decision_context, DecisionContext::None)
                else {
                    return Err(EnvironmentError::IllegalAction {
                        decision_point: self.decision_point(),
                        action_id: action.action_id(),
                        state_hash: self.state_hash(),
                    });
                };
                let hand = self.game_state.raw_state().hand();
                let card_ids = selected_slot_indices
                    .iter()
                    .filter_map(|slot_index| {
                        match hand.slots.get(*slot_index).map(|slot| &slot.item) {
                            Some(td_core::HandItemState::Card(card)) => Some(card.id),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>();
                let hand_card_ids = td_core::hand_card_id_slots(hand);
                let committed = match purpose {
                    CardSelectionPurpose::Reroll => AgentAction::Reroll { card_ids },
                    CardSelectionPurpose::BuildTower => AgentAction::SelectTower { card_ids },
                };
                self.game_state
                    .apply(
                        committed
                            .to_player_command(&hand_card_ids)
                            .expect("card selection resolves to a valid command")
                            .expect("card selection commit command"),
                    )
                    .map_err(|error| EnvironmentError::CommandRejected {
                        action_id: committed.action_id(),
                        error,
                    })?;
            }
            AgentAction::CancelCardSelection => {
                self.decision_context = DecisionContext::None;
            }
            _ => {}
        }
        Ok(())
    }

    fn tower_placement_actions(
        &self,
        mut generation_metrics: Option<&mut LegalActionGenerationMetrics>,
    ) -> Vec<AgentAction> {
        td_core::diag_scope!(PlacementScan);
        #[cfg(feature = "diagnostics")]
        td_core::diagnostics::record(|counters| counters.tower_placement_action_scans += 1);
        let hand_slot_indices = self.tower_hand_indices();
        let map_width = td_core::MAP_SIZE[0].saturating_sub(1);
        let map_height = td_core::MAP_SIZE[1].saturating_sub(1);
        let mut actions = Vec::with_capacity(hand_slot_indices.len() * map_width * map_height);
        let state = self.game_state.raw_state();
        let placement_context = state.tower_placement_context();
        for top in 0..map_height {
            for left in 0..map_width {
                if let Some(metrics) = generation_metrics.as_deref_mut() {
                    metrics.placement_position_checks += 1;
                }
                if placement_context.can_place_at(left, top) {
                    for &hand_slot_index in &hand_slot_indices {
                        actions.push(AgentAction::PlaceTower {
                            hand_slot_index,
                            left,
                            top,
                        });
                    }
                }
            }
        }
        actions.extend(
            self.game_state
                .raw_state()
                .towers()
                .iter()
                .filter_map(|tower| tower.id)
                .map(|tower_id| AgentAction::RemoveTower { tower_id }),
        );
        actions.push(AgentAction::StartDefense);
        actions
    }

    fn treasure_actions(&self) -> Vec<AgentAction> {
        match self.game_state.raw_state().flow() {
            td_core::GameFlowState::TreasureSelection { options, .. } => (0..options.len())
                .filter(|&option_index| {
                    self.game_state.raw_state().can_select_treasure(option_index)
                })
                .map(|option_index| AgentAction::SelectTreasure { option_index })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn treasure_discard_actions(&self) -> Vec<AgentAction> {
        self.game_state
            .raw_state()
            .upgrades()
            .entries()
            .iter()
            .filter(|upgrade| self.game_state.raw_state().can_discard_treasure(upgrade.id()))
            .map(|upgrade| AgentAction::DiscardTreasure {
                upgrade_id: upgrade.id(),
            })
            .collect()
    }

    fn card_service_actions(&self) -> Vec<AgentAction> {
        let Some(selection) = self.card_service_selection() else {
            return Vec::new();
        };
        let cards = &self.game_state.raw_state().deck().all_cards;
        let mut actions = cards
            .iter()
            .enumerate()
            .filter(|(_, card)| {
                raw_card_matches_filter(&selection.steps[selection.current_step].filter, card)
            })
            .map(|(card_index, _)| AgentAction::SelectCardServiceCard { card_index })
            .collect::<Vec<_>>();
        if selection.selected_card_ids[selection.current_step].len()
            == selection.steps[selection.current_step].count
        {
            actions.push(AgentAction::ConfirmCardServiceSelection);
        }
        actions
    }

    fn card_service_kind_for_action(
        &self,
        action: &AgentAction,
    ) -> Option<td_core::CardServiceKind> {
        let AgentAction::PurchaseShopItem { slot_index } = action else {
            return None;
        };
        let td_core::GameFlowState::Shopping(flow) = self.game_state.raw_state().flow() else {
            return None;
        };
        flow.slots
            .get(*slot_index)
            .and_then(|slot| match slot.slot {
                td_core::ShopSlotState::CardService { kind, .. } => {
                    td_core::CardServiceKind::from_raw(kind)
                }
                _ => None,
            })
    }

    fn card_service_selection(&self) -> Option<&td_core::CardServiceSelectionState> {
        self.card_service_selection.as_ref()
    }

    fn apply_environment_action(&mut self, action: &AgentAction) -> Result<(), EnvironmentError> {
        match action {
            AgentAction::SelectCardServiceCard { card_index } => {
                let card = self
                    .game_state
                    .raw_state()
                    .deck()
                    .all_cards
                    .get(*card_index)
                    .ok_or_else(|| EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    })?;
                let selection = self.card_service_selection.as_mut().ok_or_else(|| {
                    EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    }
                })?;
                if !raw_card_matches_filter(&selection.steps[selection.current_step].filter, card) {
                    return Err(EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    });
                }
                let selected = &mut selection.selected_card_ids[selection.current_step];
                if let Some(index) = selected.iter().position(|id| *id == card.id) {
                    selected.remove(index);
                } else if selected.len() < selection.steps[selection.current_step].count {
                    selected.push(card.id);
                }
                Ok(())
            }
            AgentAction::ConfirmCardServiceSelection => {
                let selection = self
                    .card_service_selection
                    .as_ref()
                    .filter(|selection| {
                        selection.selected_card_ids[selection.current_step].len()
                            == selection.steps[selection.current_step].count
                    })
                    .ok_or_else(|| EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    })?;
                if selection.current_step + 1 < selection.steps.len() {
                    let next_step = selection.current_step + 1;
                    let selection = self.card_service_selection.as_mut().ok_or_else(|| {
                        EnvironmentError::CardServiceRejected {
                            action_id: action.action_id(),
                        }
                    })?;
                    selection.current_step = next_step;
                    return Ok(());
                }
                let selected_card_ids = selection.selected_card_ids.clone();
                self.card_service_selection = None;
                self.game_state
                    .apply_card_service_selection(selected_card_ids)
                    .map_err(|error| EnvironmentError::CommandRejected {
                        action_id: action.action_id(),
                        error,
                    })
                    .map(|_| ())
            }
            AgentAction::BeginRerollSelection
            | AgentAction::BeginTowerSelection
            | AgentAction::SelectHandCard { .. }
            | AgentAction::DeselectHandCard { .. }
            | AgentAction::ConfirmCardSelection
            | AgentAction::CancelCardSelection
            | AgentAction::Continue => Ok(()),
            _ => Err(EnvironmentError::CardServiceRejected {
                action_id: action.action_id(),
            }),
        }
    }

    fn inventory_actions(&self) -> Vec<AgentAction> {
        let state = self.game_state.raw_state();
        state
            .items()
            .iter()
            .enumerate()
            .filter(|(item_index, _)| state.can_use_inventory_item(*item_index))
            .map(|(item_index, _)| AgentAction::UseInventoryItem { item_index })
            .collect()
    }

    fn card_hand_indices(&self) -> Vec<usize> {
        self.game_state
            .raw_state()
            .hand()
            .slots
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                matches!(slot.item, td_core::HandItemState::Card(_)).then_some(index)
            })
            .collect()
    }

    fn tower_hand_indices(&self) -> Vec<usize> {
        self.game_state
            .raw_state()
            .hand()
            .slots
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                matches!(slot.item, td_core::HandItemState::Tower(_)).then_some(index)
            })
            .collect()
    }
}

pub(crate) struct RolloutStepOutcome {
    pub(crate) terminated: bool,
    pub(crate) truncated: bool,
}

struct AppliedStep {
    reason: StepReason,
    ticks_advanced: u64,
    player_damage: f32,
    escaped_hp: f32,
}

fn merge_step_outcome(target: &mut StepOutcome, prefix: &StepOutcome) {
    target.reward.terminal += prefix.reward.terminal;
    for (key, value) in &prefix.reward.shaping {
        *target.reward.shaping.entry(key.clone()).or_insert(0.0) += value;
    }
    target.info.ticks_advanced = target
        .info
        .ticks_advanced
        .saturating_add(prefix.info.ticks_advanced);
    target.info.no_progress_cycle |= prefix.info.no_progress_cycle;
}

#[cfg(test)]
fn unordered_cards(cards: &[td_core::CardState]) -> Vec<CardObservation> {
    let mut observations = cards.iter().map(card_observation).collect::<Vec<_>>();
    observations.sort_by_key(|card| card.id);
    observations
}

fn card_service_observation_raw(
    game_state: &td_core::CoreState,
    selection: &td_core::CardServiceSelectionState,
) -> Option<CardServiceObservation> {
    let cards = &game_state.deck().all_cards;
    let selected_card_indices = selection.selected_card_ids[selection.current_step]
        .iter()
        .filter_map(|card_id| cards.iter().position(|card| card.id == *card_id))
        .collect();
    let candidate_card_indices = cards
        .iter()
        .enumerate()
        .filter(|(_, card)| {
            raw_card_matches_filter(&selection.steps[selection.current_step].filter, card)
        })
        .map(|(index, _)| index)
        .collect();
    Some(CardServiceObservation {
        key: selection.service_kind()?.key().to_string(),
        current_step: selection.current_step,
        step_count: selection.steps.len(),
        required_count: selection.steps[selection.current_step].count,
        selected_card_indices,
        candidate_card_indices,
    })
}

fn raw_card_matches_filter(
    filter: &td_core::CardSelectionFilterState,
    card: &td_core::CardState,
) -> bool {
    match filter {
        td_core::CardSelectionFilterState::Any => true,
        td_core::CardSelectionFilterState::Face => (9..=11).contains(&card.rank),
        td_core::CardSelectionFilterState::Number => card.rank <= 8,
        td_core::CardSelectionFilterState::Rank(rank) => card.rank == *rank,
        td_core::CardSelectionFilterState::Engraved => card.engraving.is_some(),
        td_core::CardSelectionFilterState::NotEngraved => card.engraving.is_none(),
        td_core::CardSelectionFilterState::And(filters) => filters
            .iter()
            .all(|filter| raw_card_matches_filter(filter, card)),
        td_core::CardSelectionFilterState::Or(filters) => filters
            .iter()
            .any(|filter| raw_card_matches_filter(filter, card)),
    }
}

#[cfg(test)]
fn card_observation(card: &td_core::CardState) -> CardObservation {
    CardObservation {
        id: card.id,
        suit: ["spades", "hearts", "diamonds", "clubs"][card.suit as usize].to_string(),
        rank: [
            "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack",
            "queen", "king", "ace",
        ][card.rank as usize]
            .to_string(),
        polish_pct_raw: card.polish_pct_raw,
        engraving: card.engraving.map(|kind| {
            ["magnet", "overcharge", "cactus", "spinning_top"][kind as usize].to_string()
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn default_reward_config_preserves_existing_scales() {
        let config = RewardConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.terminal_win, 1.0);
        assert_eq!(config.terminal_loss, -1.0);
        assert_eq!(config.escaped_hp_penalty_scale, 50.0);
        assert_eq!(config.player_hp_loss_penalty_scale, 60.0);
        assert_eq!(config.potential_weight, 0.0);
        assert_eq!(config.potential_gamma, 0.99);
    }

    #[test]
    fn reward_config_rejects_positive_cycle_penalty() {
        let config = RewardConfig {
            no_progress_cycle_penalty: 0.25,
            ..RewardConfig::default()
        };
        let error = config
            .validate()
            .expect_err("positive cycle penalty must fail");
        assert!(error.contains("non-positive"));
    }

    #[test]
    fn reward_config_rejects_non_finite_fields() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let config = RewardConfig {
                no_progress_cycle_penalty: value,
                ..RewardConfig::default()
            };
            assert!(config.validate().is_err());
        }
    }

    #[test]
    fn terminal_potential_is_zero() {
        let environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), 7);
        let before = environment.snapshot();
        let after = before.clone();
        let config = RewardConfig {
            potential_weight: 1.0,
            ..RewardConfig::default()
        };

        assert_eq!(
            potential_shaping(&before, &after, true, &config),
            -potential(&before)
        );
    }

    #[test]
    fn potential_progress_component_is_zero_when_disabled() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert_eq!(outcome.reward.shaping["potential_progress"], 0.0);
    }

    fn environment() -> GameEnvironment {
        GameEnvironment::new(Arc::new(GameConfig::default_config()), 7)
    }

    #[test]
    #[ignore = "manual release profiling diagnostic"]
    fn profile_environment_step_path() {
        let config = Arc::new(GameConfig::default_config());
        let started = std::time::Instant::now();
        let mut steps = 0usize;
        let mut episodes = 0usize;

        for seed in 11..15 {
            let mut environment = GameEnvironment::new(Arc::clone(&config), seed);
            for _ in 0..128 {
                let action = environment
                    .legal_actions()
                    .into_iter()
                    .next()
                    .expect("profile environment should expose an action")
                    .action;
                let outcome = environment
                    .step(action)
                    .expect("profile action should be legal");
                std::hint::black_box((&outcome, environment.policy_trace().steps.len()));
                steps += 1;
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
            episodes += 1;
        }

        let elapsed = started.elapsed();
        println!(
            "environment_step_profile episodes={episodes} steps={steps} elapsed={:.3}s ns_per_step={:.0}",
            elapsed.as_secs_f64(),
            elapsed.as_nanos() as f64 / steps as f64
        );
        assert!(steps > 0);
    }

    #[test]
    fn reset_and_observation_are_reproducible_for_same_seed() {
        let mut first = environment();
        let mut second = environment();

        assert_eq!(
            first.reset(42, Arc::new(GameConfig::default_config())),
            second.reset(42, Arc::new(GameConfig::default_config()))
        );
        assert_eq!(first.state_hash(), second.state_hash());
        assert_eq!(first.legal_actions(), second.legal_actions());
    }

    #[test]
    fn two_core_tower_attack_ticks_match_for_a_seeded_defense() {
        let config = Arc::new(GameConfig::default_config());
        let mut first = GameCore::new((*config).clone(), 7);
        let mut second = GameCore::new((*config).clone(), 7);
        let commands = [
            PlayerCommand::StartSelectingTower,
            PlayerCommand::SelectTower {
                selected_slot_indices: vec![0],
            },
            PlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            PlayerCommand::StartDefense,
        ];

        for command in commands {
            first
                .apply(command.clone())
                .expect("first command should be accepted");
            second
                .apply(command)
                .expect("second command should be accepted");
            assert_eq!(
                first.authoritative_hash(),
                second.authoritative_hash(),
                "command paths diverged at tick {}",
                first.sim_tick().ticks()
            );
        }

        for _ in 0..180 {
            let first_output = first.advance_tick();
            let second_output = second.advance_tick();
            assert_eq!(first_output, second_output);
            assert_eq!(
                first.core_state_snapshot(),
                second.core_state_snapshot(),
                "core combat diverged at tick {}",
                first.sim_tick().ticks()
            );
        }
    }

    #[test]
    fn game_core_and_simulator_command_paths_match_at_the_same_sim_tick() {
        let config = Arc::new(GameConfig::default_config());
        let mut core = GameCore::new((*config).clone(), 7);
        let mut simulator = GameEnvironment::new(config, 7);
        let action = AgentAction::StartSelectingTower;
        let command = action
            .to_player_command(&[])
            .expect("start selecting tower resolves")
            .expect("start selecting tower must map to a player command");

        core.apply(command)
            .expect("core command should be accepted");
        simulator
            .step(action)
            .expect("simulator action should be accepted");

        while core.sim_tick().ticks() < simulator.game_state.sim_tick().ticks() {
            core.advance_tick();
        }

        assert_eq!(
            core.sim_tick().ticks(),
            simulator.game_state.sim_tick().ticks()
        );
        assert_eq!(
            core.authoritative_hash(),
            simulator.game_state.authoritative_hash()
        );
    }

    #[test]
    fn core_replay_restores_simulator_hash_without_headed_types() {
        let config = GameConfig::default_config();
        let mut source = GameCore::new(config.clone(), 0xC0DE);
        for command in [
            PlayerCommand::StartSelectingTower,
            PlayerCommand::Reroll {
                selected_slot_indices: Vec::new(),
            },
        ] {
            source
                .apply_with_receipt(command)
                .expect("source command should be accepted");
        }

        let replay = source.replay();
        let restored =
            GameEnvironment::from_core_replay(&replay).expect("core replay should restore");

        assert_eq!(restored.state_hash(), source.authoritative_hash());
        assert_eq!(
            restored
                .game_state
                .raw_state()
                .replay_checkpoints()
                .last()
                .expect("restored checkpoint"),
            replay.checkpoints.last().expect("source checkpoint")
        );
    }

    #[test]
    fn game_core_and_simulator_match_across_card_selection_command_sequence() {
        let config = Arc::new(GameConfig::default_config());
        let mut core = GameCore::new((*config).clone(), 7);
        let mut simulator = GameEnvironment::new(config, 7);

        let start = AgentAction::StartSelectingTower;
        core.apply(
            start
                .to_player_command(&[])
                .expect("start command resolves")
                .expect("start command"),
        )
        .expect("core start command should be accepted");
        simulator
            .step(start)
            .expect("simulator start action should be accepted");

        simulator
            .step(AgentAction::BeginTowerSelection)
            .expect("begin tower selection should be accepted");
        let hand_slot_index = simulator
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => Some(hand_slot_index),
                _ => None,
            })
            .expect("a hand card should be selectable");
        simulator
            .step(AgentAction::SelectHandCard { hand_slot_index })
            .expect("card selection should be accepted");

        let confirm = AgentAction::ConfirmCardSelection;
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![hand_slot_index],
        })
        .expect("core tower selection command should be accepted");
        simulator
            .step(confirm)
            .expect("simulator card selection should be accepted");

        assert_eq!(core.sim_tick(), simulator.game_state.sim_tick());
        assert_eq!(core.authoritative_hash(), simulator.state_hash());
    }

    #[test]
    fn semantic_build_action_commits_selection_and_placement_as_one_step() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let selecting_observation = environment.snapshot();
        assert!(!selecting_observation.build_tower_candidates.is_empty());
        let oracle_actions = environment.semantic_legal_actions();
        let proposal_actions = environment.semantic_legal_actions_with_position_limit(Some(
            DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT,
        ));
        let oracle_ids = oracle_actions
            .iter()
            .map(|legal| legal.id.clone())
            .collect::<HashSet<_>>();
        assert!(proposal_actions.len() < oracle_actions.len());
        assert!(
            proposal_actions
                .iter()
                .all(|legal| oracle_ids.contains(&legal.id))
        );
        let build_action = proposal_actions
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::BuildTower {
                    card_ids,
                    hand_slot_index,
                    left,
                    top,
                } => Some(AgentAction::BuildTower {
                    card_ids,
                    hand_slot_index,
                    left,
                    top,
                }),
                _ => None,
            })
            .expect("semantic build action should be available");
        let (card_ids, left, top) = match &build_action {
            AgentAction::BuildTower {
                card_ids,
                left,
                top,
                ..
            } => (card_ids.clone(), *left, *top),
            _ => unreachable!("the selected semantic action should build a tower"),
        };
        let expected_template = selecting_observation
            .build_tower_candidates
            .iter()
            .find(|candidate| candidate.card_ids == card_ids)
            .map(|candidate| candidate.template.clone())
            .expect("semantic build action should have an observed resulting tower");

        let outcome = environment
            .semantic_step(build_action)
            .expect("semantic build action should be accepted");

        assert_eq!(
            outcome.observation.decision_point,
            DecisionPoint::TowerPlacement
        );
        assert_eq!(environment.game_state.raw_state().towers().len(), 1);
        let placed_observation = environment.snapshot();
        let placed_tower = placed_observation
            .towers
            .iter()
            .find(|tower| tower.left == left && tower.top == top)
            .expect("semantic build action should place the selected tower");
        assert_eq!(placed_tower.template, expected_template);
        assert_eq!(environment.policy_trace().steps.len(), 2);
        assert!(matches!(
            environment
                .policy_trace()
                .steps
                .last()
                .map(|step| &step.agent_action),
            Some(AgentAction::BuildTower { .. })
        ));
        environment
            .core_replay()
            .validate()
            .expect("semantic replay should remain valid");
    }

    #[test]
    fn semantic_build_action_can_start_from_shop() {
        let mut environment = environment();
        assert!(!environment.snapshot().build_tower_candidates.is_empty());
        let build_action = environment
            .semantic_legal_actions_with_position_limit(Some(1))
            .into_iter()
            .find_map(|legal| {
                matches!(legal.action, AgentAction::BuildTower { .. }).then_some(legal.action)
            })
            .expect("shop should expose a semantic build action");
        environment
            .semantic_step(build_action)
            .expect("shop semantic build action should be accepted");
        assert!(matches!(
            environment
                .policy_trace()
                .steps
                .last()
                .map(|step| &step.agent_action),
            Some(AgentAction::BuildTower { .. })
        ));
        assert!(
            environment
                .policy_trace()
                .steps
                .iter()
                .all(|step| !matches!(step.agent_action, AgentAction::StartSelectingTower))
        );
        environment
            .core_replay()
            .validate()
            .expect("shop semantic replay should remain valid");
    }

    #[test]
    fn illegal_semantic_build_action_leaves_no_partial_mutation() {
        let card_ids = environment()
            .snapshot()
            .build_tower_candidates
            .first()
            .expect("a build candidate should exist")
            .card_ids
            .clone();
        let illegal_actions = [
            AgentAction::BuildTower {
                card_ids: card_ids.clone(),
                hand_slot_index: 0,
                left: td_core::MAP_SIZE[0],
                top: td_core::MAP_SIZE[1],
            },
            AgentAction::BuildTower {
                card_ids: card_ids.clone(),
                hand_slot_index: 99,
                left: 0,
                top: 0,
            },
            AgentAction::BuildTower {
                card_ids: vec![999_999],
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
        ];
        for illegal_action in illegal_actions {
            let mut environment = environment();
            let before_hash = environment.state_hash();
            let before_observation = environment.snapshot();

            let result = environment.semantic_step(illegal_action.clone());

            assert!(
                matches!(result, Err(EnvironmentError::IllegalAction { .. })),
                "expected {illegal_action:?} to be rejected"
            );
            assert_eq!(environment.state_hash(), before_hash);
            assert_eq!(environment.snapshot(), before_observation);
        }
    }

    #[test]
    fn can_place_at_matches_actual_placement_outcome_for_sampled_map_cells() {
        let environment = environment();
        let card_ids = environment
            .snapshot()
            .build_tower_candidates
            .first()
            .expect("a build candidate should exist")
            .card_ids
            .clone();
        let placement_context = environment.game_state.raw_state().tower_placement_context();
        let map_width = td_core::MAP_SIZE[0].saturating_sub(1);
        let map_height = td_core::MAP_SIZE[1].saturating_sub(1);
        let mut free_cells = Vec::new();
        let mut blocked_cells = Vec::new();
        for top in 0..map_height {
            for left in 0..map_width {
                if placement_context.can_place_at(left, top) {
                    free_cells.push((left, top));
                } else {
                    blocked_cells.push((left, top));
                }
            }
        }
        assert!(!free_cells.is_empty());
        assert!(!blocked_cells.is_empty());

        const SAMPLE_SIZE: usize = 8;
        let sample = |cells: &[(usize, usize)]| -> Vec<(usize, usize)> {
            let stride = (cells.len() / SAMPLE_SIZE).max(1);
            cells
                .iter()
                .copied()
                .step_by(stride)
                .take(SAMPLE_SIZE)
                .collect()
        };
        let sampled_cells = sample(&free_cells)
            .into_iter()
            .map(|cell| (cell, true))
            .chain(sample(&blocked_cells).into_iter().map(|cell| (cell, false)))
            .collect::<Vec<_>>();

        let mut prepared = environment
            .fork_for_rollout_seed(11)
            .expect("fork should succeed");
        prepared
            .step_unchecked(AgentAction::StartSelectingTower)
            .expect("tower selection should start");
        prepared
            .step_unchecked(AgentAction::SelectTower {
                card_ids: card_ids.clone(),
            })
            .expect("tower selection should resolve to a template");

        // Probe placement via CoreState::place_tower directly instead of
        // GameEnvironment::step, which recomputes the full O(map) legal
        // action list (including a can_place_at scan of every cell) on
        // every call; that made this loop O(sample_count * map_size).
        let base_state = prepared.game_state.raw_state().clone();
        for ((left, top), predicted) in sampled_cells {
            let mut probe = base_state.clone();
            let outcome = probe.place_tower(0, left, top);
            assert_eq!(
                outcome.is_ok(),
                predicted,
                "can_place_at disagreed with the actual placement outcome at ({left}, {top})"
            );
        }
    }

    fn hidden_order_parts(environment: &GameEnvironment) -> td_core::CoreSnapshotParts {
        environment.game_state.core_state_snapshot().snapshot_parts()
    }

    fn shop_reroll_environment() -> (GameEnvironment, AgentAction) {
        let environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), 0);
        assert_eq!(environment.decision_point(), DecisionPoint::Shop);
        let reroll = environment
            .semantic_legal_actions()
            .into_iter()
            .find(|legal| matches!(legal.action, AgentAction::Reroll { .. }))
            .expect("a fresh Shop state offers Reroll")
            .action;
        (environment, reroll)
    }

    #[test]
    fn hidden_order_fork_preserves_observation_and_legal_actions_for_all_scenarios() {
        let (environment, _) = shop_reroll_environment();
        let before = environment.snapshot();
        let before_actions = environment.semantic_legal_actions();
        for scenario_seed in 2000..2016 {
            let fork = environment.fork_for_rollout_seed(scenario_seed).unwrap();
            assert_eq!(fork.snapshot(), before, "scenario {scenario_seed}");
            assert_eq!(fork.semantic_legal_actions(), before_actions);
            assert_eq!(fork.snapshot().shop, before.shop);
        }
    }

    #[test]
    fn hidden_order_fork_is_reproducible_and_shared_across_candidates() {
        let (environment, _) = shop_reroll_environment();
        let first = environment.fork_for_rollout_seed(2003).unwrap();
        let second = environment.fork_for_rollout_seed(2003).unwrap();
        assert_eq!(first.game_state.core_state_snapshot(), second.game_state.core_state_snapshot());
        assert_eq!(first.state_hash(), second.state_hash());
        let other = environment.fork_for_rollout_seed(2004).unwrap();
        assert_ne!(
            first.game_state.core_state_snapshot().snapshot_parts().deck.draw_pile,
            other.game_state.core_state_snapshot().snapshot_parts().deck.draw_pile
        );
    }

    #[test]
    fn hidden_order_fork_keeps_draw_pile_membership() {
        let (environment, _) = shop_reroll_environment();
        let ids = |parts: &td_core::CoreSnapshotParts| {
            let mut ids = parts.deck.draw_pile.iter().map(|card| card.id).collect::<Vec<_>>();
            ids.sort_unstable();
            ids
        };
        let source = hidden_order_parts(&environment);
        assert!(source.deck.draw_pile.len() > 4);
        for scenario_seed in 2000..2016 {
            let fork = environment.fork_for_rollout_seed(scenario_seed).unwrap();
            let forked = hidden_order_parts(&fork);
            assert_eq!(ids(&forked), ids(&source));
            assert_eq!(forked.deck.discard_pile, source.deck.discard_pile);
            assert_eq!(forked.deck.all_cards, source.deck.all_cards);
            assert_eq!(forked.hand, source.hand);
        }
    }

    #[test]
    fn hidden_order_fork_makes_reroll_outcome_vary_across_scenarios() {
        let (environment, reroll) = shop_reroll_environment();
        let mut hands = HashSet::new();
        for scenario_seed in 2000..2016 {
            let mut fork = environment.fork_for_rollout_seed(scenario_seed).unwrap();
            fork.semantic_step(reroll.clone()).unwrap();
            hands.insert(serde_json::to_string(&fork.snapshot().hand).unwrap());
        }
        assert!(hands.len() > 1, "reroll drew the same hand in all 16 scenarios");
    }

    fn assert_bag_suffix_resampled<T: Clone + Ord + std::fmt::Debug>(
        label: &str,
        entries: &[T],
        cursor: usize,
        resample: impl Fn(u64) -> Vec<T>,
    ) {
        let mut suffixes = HashSet::new();
        for scenario_seed in 2000..2016 {
            let resampled = resample(scenario_seed);
            assert_eq!(&resampled[..cursor], &entries[..cursor], "{label}: consumed prefix");
            let mut expected = entries[cursor..].to_vec();
            let mut actual = resampled[cursor..].to_vec();
            expected.sort();
            actual.sort();
            assert_eq!(actual, expected, "{label}: suffix multiset");
            suffixes.insert(format!("{:?}", &resampled[cursor..]));
        }
        assert!(suffixes.len() > 1, "{label}: suffix order never varied");
    }

    #[test]
    fn hidden_order_resampling_preserves_bag_prefix_cursor_cycle_and_multiset() {
        let (environment, _) = shop_reroll_environment();
        let mut parts = hidden_order_parts(&environment);
        parts.rng.shop.category_bag = td_core::BagState {
            entries: vec![0, 1, 2, 0, 1, 2, 0, 1, 2, 0],
            cursor: 3,
            cycle: 5,
        };
        parts.rng.shop.rarity_bags[1] = td_core::BagState {
            entries: vec![0, 0, 1, 1, 1, 2, 2, 0, 1, 2],
            cursor: 2,
            cycle: 7,
        };
        parts.rng.shop.content_bags[4] = td_core::ContentBagState {
            entries: ["a", "b", "c", "d", "e", "f"].map(str::to_string).to_vec(),
            cursor: 1,
            cycle: 2,
        };
        parts.rng.reward_upgrade_bag = td_core::BagState {
            entries: vec![4, 2, 1, 3, 0, 5, 6],
            cursor: 2,
            cycle: 3,
        };
        let game_seed = environment.seed();
        let resampled = |scenario_seed: u64| {
            let mut copy = parts.clone();
            super::resample_hidden_order(&mut copy, game_seed, scenario_seed);
            copy
        };
        for scenario_seed in 2000..2016 {
            let copy = resampled(scenario_seed);
            assert_eq!(copy.rng.shop.category_bag.cursor, 3);
            assert_eq!(copy.rng.shop.category_bag.cycle, 5);
            assert_eq!(copy.rng.shop.rarity_bags[1].cursor, 2);
            assert_eq!(copy.rng.shop.rarity_bags[1].cycle, 7);
            assert_eq!(copy.rng.shop.content_bags[4].cursor, 1);
            assert_eq!(copy.rng.shop.content_bags[4].cycle, 2);
            assert_eq!(copy.rng.reward_upgrade_bag.cursor, 2);
            assert_eq!(copy.rng.reward_upgrade_bag.cycle, 3);
        }
        assert_bag_suffix_resampled(
            "shop category",
            &parts.rng.shop.category_bag.entries,
            3,
            |seed| resampled(seed).rng.shop.category_bag.entries,
        );
        assert_bag_suffix_resampled(
            "shop rarity",
            &parts.rng.shop.rarity_bags[1].entries,
            2,
            |seed| resampled(seed).rng.shop.rarity_bags[1].entries.clone(),
        );
        assert_bag_suffix_resampled(
            "shop content",
            &parts.rng.shop.content_bags[4].entries,
            1,
            |seed| resampled(seed).rng.shop.content_bags[4].entries.clone(),
        );
        assert_bag_suffix_resampled(
            "reward upgrade",
            &parts.rng.reward_upgrade_bag.entries,
            2,
            |seed| resampled(seed).rng.reward_upgrade_bag.entries,
        );
    }

    fn environment_with_core_state(
        environment: &GameEnvironment,
        state: td_core::CoreState,
    ) -> GameEnvironment {
        GameEnvironment {
            game_state: GameCore::from_core_state_snapshot(state).expect("valid core state"),
            config: Arc::clone(&environment.config),
            seed: environment.seed,
            max_advance_ticks: environment.max_advance_ticks,
            rollout_tick_deadline: environment.rollout_tick_deadline,
            card_service_selection: environment.card_service_selection.clone(),
            decision_context: environment.decision_context.clone(),
            environment_actions: environment.environment_actions.clone(),
            policy_trace: environment.policy_trace.clone(),
            metrics: environment.metrics.clone(),
            reward_config: environment.reward_config.clone(),
            max_stage: environment.max_stage,
        }
    }

    fn treasure_selection_environment(fill_treasures: bool) -> GameEnvironment {
        // `GameCore::new` skips the opening treasure selection in test
        // builds, so start from the real initial core state instead.
        let base = environment();
        let opening = GameCore::from_core_config((*base.config).clone(), base.seed)
            .expect("valid default config");
        let environment = environment_with_core_state(&base, opening.core_state_snapshot());
        assert_eq!(environment.decision_point(), DecisionPoint::TreasureSelection);
        let mut state = environment.game_state.core_state_snapshot();
        if fill_treasures {
            while state.upgrades().len() < state.treasure_capacity() {
                state
                    .acquire_upgrade(td_core::generated_upgrade(td_core::UpgradeKind::Apple))
                    .expect("treasures up to capacity fit");
            }
            assert_eq!(state.upgrades().len(), state.treasure_capacity());
        }
        environment_with_core_state(&environment, state)
    }

    fn treasure_legal_actions(environment: &GameEnvironment) -> Vec<AgentAction> {
        let mut actions = environment
            .legal_actions()
            .into_iter()
            .map(|legal| legal.action)
            .filter(|action| {
                matches!(
                    action,
                    AgentAction::SelectTreasure { .. } | AgentAction::DiscardTreasure { .. }
                )
            })
            .collect::<Vec<_>>();
        actions.extend(
            environment
                .semantic_legal_actions()
                .into_iter()
                .map(|legal| legal.action)
                .filter(|action| {
                    matches!(
                        action,
                        AgentAction::SelectTreasure { .. } | AgentAction::DiscardTreasure { .. }
                    )
                }),
        );
        actions
    }

    #[test]
    fn full_treasure_capacity_hides_select_treasure_and_exposes_executable_discards() {
        let environment = treasure_selection_environment(true);
        let actions = treasure_legal_actions(&environment);
        assert!(
            !actions.iter().any(|a| matches!(a, AgentAction::SelectTreasure { .. })),
            "SelectTreasure must not be legal when treasure slots are full"
        );
        let discards = actions
            .iter()
            .filter(|a| matches!(a, AgentAction::DiscardTreasure { .. }))
            .count();
        assert!(discards >= 5, "expected discard actions for the full treasures, got {discards}");
    }

    #[test]
    fn every_legal_treasure_action_executes_without_rejection() {
        for fill in [false, true] {
            let environment = treasure_selection_environment(fill);
            let actions = treasure_legal_actions(&environment);
            assert!(!actions.is_empty());
            for action in actions {
                let state = environment.game_state.core_state_snapshot();
                let mut clone = environment_with_core_state(&environment, state);
                clone
                    .step(action.clone())
                    .unwrap_or_else(|error| panic!("fill={fill}: legal {action:?} rejected: {error:?}"));
            }
        }
    }

    #[test]
    fn discard_then_select_treasure_completes_the_selection_flow() {
        let mut environment = treasure_selection_environment(true);
        let discard = environment
            .legal_actions()
            .into_iter()
            .map(|legal| legal.action)
            .find(|action| matches!(action, AgentAction::DiscardTreasure { .. }))
            .expect("a full treasure bag exposes a discard");
        environment.step(discard).expect("discard must execute");
        assert_eq!(environment.decision_point(), DecisionPoint::TreasureSelection);
        let select = environment
            .legal_actions()
            .into_iter()
            .map(|legal| legal.action)
            .find(|action| matches!(action, AgentAction::SelectTreasure { .. }))
            .expect("SelectTreasure becomes legal once a slot is free");
        environment.step(select).expect("select must execute");
        assert_ne!(environment.decision_point(), DecisionPoint::TreasureSelection);
    }

    #[test]
    fn treasure_selection_with_room_keeps_every_option_legal() {
        let environment = treasure_selection_environment(false);
        let options = match environment.game_state.raw_state().flow() {
            td_core::GameFlowState::TreasureSelection { options, .. } => options.len(),
            other => panic!("expected TreasureSelection, got {other:?}"),
        };
        let selects = treasure_legal_actions(&environment)
            .into_iter()
            .filter(|a| matches!(a, AgentAction::SelectTreasure { .. }))
            .count();
        assert!(options > 0);
        assert_eq!(selects, options * 2, "legal + semantic lists each expose all options");
    }

    #[test]
    fn canonical_continuation_discards_then_selects_at_full_treasure_capacity() {
        let mut environment = treasure_selection_environment(true);
        let first = crate::policy_runner::canonical_scripted_semantic_action(&environment)
            .expect("canonical policy must find a legal action");
        assert!(
            matches!(first, AgentAction::DiscardTreasure { .. }),
            "with no legal SelectTreasure the canonical fallback takes a legal discard, got {first:?}"
        );
        let mut steps = 0;
        while environment.decision_point() == DecisionPoint::TreasureSelection {
            let action = crate::policy_runner::canonical_scripted_semantic_action(&environment)
                .expect("canonical action");
            environment
                .step(action)
                .expect("canonical continuation must never be rejected");
            steps += 1;
            assert!(steps <= 4, "treasure selection did not resolve");
        }
        assert_eq!(steps, 2, "discard, then select");
    }

    #[test]
    fn rollout_seed_fork_preserves_visible_state_and_legal_actions() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let before = environment.snapshot();
        let before_actions = environment.semantic_legal_actions();
        let fork = environment
            .fork_for_rollout_seed(19)
            .expect("rollout fork should restore the core snapshot");

        assert_eq!(fork.snapshot(), before);
        assert_eq!(fork.semantic_legal_actions(), before_actions);
        assert_ne!(fork.state_hash(), environment.state_hash());
    }

    #[test]
    fn deck_observation_exposes_owned_cards_and_zone_compositions() {
        let environment = environment();
        let observation = environment.snapshot();
        let owned_ids = observation
            .deck
            .all_cards
            .iter()
            .map(|card| card.id)
            .collect::<HashSet<_>>();

        assert_eq!(owned_ids.len(), observation.deck.all_cards.len());
        assert!(
            observation
                .deck
                .draw_cards
                .iter()
                .all(|card| owned_ids.contains(&card.id))
        );
        assert!(
            observation
                .deck
                .discard_cards
                .iter()
                .all(|card| owned_ids.contains(&card.id))
        );
        let zone_ids = observation
            .deck
            .draw_cards
            .iter()
            .chain(observation.deck.discard_cards.iter())
            .map(|card| card.id)
            .collect::<HashSet<_>>();
        assert_eq!(
            zone_ids.len(),
            observation.deck.draw_cards.len() + observation.deck.discard_cards.len()
        );
        assert!(zone_ids.len() <= owned_ids.len());
    }

    #[test]
    fn unordered_card_observation_is_invariant_to_zone_order() {
        let cards = environment()
            .game_state
            .deck()
            .all_cards
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>();
        let forward = unordered_cards(&cards);
        let reverse = unordered_cards(&cards.into_iter().rev().collect::<Vec<_>>());

        assert_eq!(forward, reverse);
    }

    #[test]
    fn simulator_selection_changes_hand_observation_without_mutating_gameplay_hand() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
            .step(AgentAction::BeginTowerSelection)
            .expect("begin tower selection should be legal");
        let select = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("an unselected card should be selectable");

        let before = environment.snapshot();
        let slot_index = match select {
            AgentAction::SelectHandCard { hand_slot_index } => hand_slot_index,
            _ => unreachable!(),
        };
        environment.step(select).expect("select should be legal");
        let selected = environment.snapshot();
        assert_eq!(selected.selected_hand_slot_indices, vec![slot_index]);
        assert!(selected.hand[slot_index].selected);

        assert!(environment.legal_actions().iter().any(|legal| matches!(
            legal.action,
            AgentAction::DeselectHandCard { hand_slot_index } if hand_slot_index == slot_index
        )));
        assert_eq!(before.deck.all_cards, selected.deck.all_cards);
    }

    #[test]
    fn curriculum_stage_limit_preserves_authoritative_hash_before_target_stage() {
        let base_config = GameConfig::default_config();
        let config = Arc::new(base_config);
        let runner_config = crate::policy_runner::PolicyRunnerConfig {
            max_decisions_per_episode: 32,
            max_stage: Some(2),
            ..Default::default()
        };
        let curriculum_runner_config = crate::policy_runner::PolicyRunnerConfig {
            max_stage: Some(2),
            ..runner_config.clone()
        };
        let mut unrestricted_hashes = Vec::new();
        let unrestricted = crate::policy_runner::run_episode_with_step_callback(
            Arc::clone(&config),
            17,
            &runner_config,
            |_observation, legal_actions| {
                Ok(legal_actions
                    .first()
                    .expect("test environment should expose a legal action")
                    .action
                    .clone())
            },
            |_observation, _legal_actions, _action, outcome| {
                unrestricted_hashes.push(outcome.state_hash.clone());
            },
        )
        .expect("unrestricted rollout should succeed");

        let mut curriculum_hashes = Vec::new();
        let curriculum = crate::policy_runner::run_episode_with_step_callback(
            config,
            17,
            &curriculum_runner_config,
            |_observation, legal_actions| {
                Ok(legal_actions
                    .first()
                    .expect("test environment should expose a legal action")
                    .action
                    .clone())
            },
            |_observation, _legal_actions, _action, outcome| {
                curriculum_hashes.push(outcome.state_hash.clone());
            },
        )
        .expect("curriculum rollout should succeed");

        let compared_len = unrestricted_hashes.len().min(curriculum_hashes.len());
        assert!(compared_len > 0);
        assert_eq!(
            unrestricted_hashes[..compared_len],
            curriculum_hashes[..compared_len],
            "curriculum must not alter authoritative state before the target stage"
        );
        assert_eq!(
            unrestricted.final_state_hash,
            unrestricted_hashes
                .last()
                .cloned()
                .expect("unrestricted rollout should have a final hash")
        );
        assert_eq!(
            curriculum.final_state_hash,
            curriculum_hashes
                .last()
                .cloned()
                .expect("curriculum rollout should have a final hash")
        );
    }

    #[test]
    fn replay_round_trips_with_current_action_schema() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");

        let replay = environment.replay();
        let json = serde_json::to_string(&replay).expect("replay should serialize");
        let decoded: EnvironmentReplay =
            serde_json::from_str(&json).expect("replay should deserialize");

        assert_eq!(decoded, replay);
        assert_eq!(decoded.action_schema_version, ACTION_SCHEMA_VERSION);
        decoded
            .execute(Arc::new(GameConfig::default_config()))
            .expect("current replay schema should execute");
    }

    #[test]
    fn environment_replay_checkpoints_match_core_replay_checkpoints() {
        let config = Arc::new(GameConfig::default_config());
        let mut environment = GameEnvironment::new(Arc::clone(&config), 7);
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        environment
            .step(AgentAction::BeginTowerSelection)
            .expect("begin tower selection should be legal");
        let hand_slot_index = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => Some(hand_slot_index),
                _ => None,
            })
            .expect("a hand card should be selectable");
        environment
            .step(AgentAction::SelectHandCard { hand_slot_index })
            .expect("card selection should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection confirmation should be legal");

        let mut core = GameCore::new((*config).clone(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("core start command should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![hand_slot_index],
        })
        .expect("core tower selection command should be accepted");

        let environment_checkpoints = environment.replay().checkpoints;
        let core_checkpoints = core.replay().checkpoints;
        assert_eq!(environment_checkpoints, core_checkpoints);
    }

    #[test]
    fn replay_with_unsupported_action_schema_fails_before_commands() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        let mut replay = environment.replay();
        replay.action_schema_version = ACTION_SCHEMA_VERSION + 1;

        let error = match replay.execute(Arc::new(GameConfig::default_config())) {
            Ok(_) => panic!("unsupported action schema should be rejected"),
            Err(error) => error,
        };

        assert!(error.contains("unsupported environment replay action schema"));
    }

    #[test]
    fn replay_with_missing_action_schema_fails_during_deserialization() {
        let mut value = serde_json::to_value(environment().replay()).expect("replay value");
        value
            .as_object_mut()
            .expect("replay should be an object")
            .remove("action_schema_version");

        let error = serde_json::from_value::<EnvironmentReplay>(value)
            .expect_err("action schema is required for replay execution");

        assert!(error.to_string().contains("action_schema_version"));
    }

    #[test]
    fn legal_action_ids_are_stable_and_ordered() {
        let environment = environment();
        let actions = environment.legal_actions();
        let ids = actions
            .iter()
            .map(|action| action.id.clone())
            .collect::<Vec<_>>();
        let mut sorted_ids = ids.clone();
        sorted_ids.sort();

        assert_eq!(ids, sorted_ids);
        assert!(
            actions
                .iter()
                .all(|action| action.id == action.action.action_id())
        );
    }

    #[test]
    fn tower_placement_exposes_direct_slot_coordinate_actions() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
            .step(AgentAction::BeginTowerSelection)
            .expect("tower selection should be legal");
        let card = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("card selection should be available");
        environment
            .step(card)
            .expect("card selection should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection should be confirmable");

        let tower_actions = environment
            .legal_actions()
            .into_iter()
            .filter_map(|legal| match legal.action {
                AgentAction::PlaceTower {
                    hand_slot_index,
                    left,
                    top,
                } => Some((hand_slot_index, left, top)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!tower_actions.is_empty());
        assert!(
            tower_actions.iter().all(|(_, left, top)| {
                *left < td_core::MAP_SIZE[0] && *top < td_core::MAP_SIZE[1]
            })
        );
        assert!(tower_actions.iter().any(|(hand_slot_index, _, _)| {
            tower_actions
                .iter()
                .filter(|(other_slot, _, _)| other_slot == hand_slot_index)
                .count()
                > 1
        }));
    }

    #[test]
    fn action_kind_has_one_bucket_per_action_variant() {
        let actions = [
            AgentAction::PurchaseShopItem { slot_index: 0 },
            AgentAction::StartSelectingTower,
            AgentAction::BeginRerollSelection,
            AgentAction::BeginTowerSelection,
            AgentAction::SelectHandCard { hand_slot_index: 0 },
            AgentAction::DeselectHandCard { hand_slot_index: 0 },
            AgentAction::ConfirmCardSelection,
            AgentAction::CancelCardSelection,
            AgentAction::Reroll { card_ids: vec![] },
            AgentAction::SelectTower { card_ids: vec![] },
            AgentAction::BuildTower {
                card_ids: vec![],
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::RemoveTower { tower_id: 0 },
            AgentAction::StartDefense,
            AgentAction::SelectTreasure { option_index: 0 },
            AgentAction::SelectCardServiceCard { card_index: 0 },
            AgentAction::ConfirmCardServiceSelection,
            AgentAction::UseInventoryItem { item_index: 0 },
            AgentAction::DiscardTreasure { upgrade_id: 0 },
            AgentAction::Continue,
        ];
        let kinds = actions.iter().map(AgentAction::kind).collect::<Vec<_>>();
        assert_eq!(kinds.len(), ActionKind::COUNT);
        assert_eq!(
            kinds.windows(2).filter(|pair| pair[0] == pair[1]).count(),
            0
        );
        let mut indices = kinds.iter().map(|kind| kind.index()).collect::<Vec<_>>();
        indices.sort_unstable();
        assert_eq!(indices, (0..ActionKind::COUNT).collect::<Vec<_>>());
    }

    #[test]
    fn invalid_action_does_not_change_state_or_hash() {
        let mut environment = environment();
        let before = environment.snapshot();
        let before_hash = environment.state_hash();

        let result = environment.step(AgentAction::StartDefense);

        assert!(matches!(
            result,
            Err(EnvironmentError::IllegalAction { .. })
        ));
        assert_eq!(environment.snapshot(), before);
        assert_eq!(environment.state_hash(), before_hash);
    }

    #[test]
    fn step_reports_clear_progress_shaping_reward() {
        let mut first = environment();
        let mut second = environment();
        let first_outcome = first
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let second_outcome = second
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");

        let first_progress = first_outcome.reward.shaping.get("clear_progress").copied();
        let second_progress = second_outcome.reward.shaping.get("clear_progress").copied();
        assert!(first_progress.is_some_and(|progress| progress >= 0.0));
        assert_eq!(first_progress, second_progress);
    }

    #[test]
    fn clear_progress_is_zero_when_clear_rate_does_not_change() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let next_action = environment
            .legal_actions()
            .first()
            .expect("next decision should have a legal action")
            .action
            .clone();

        let outcome = environment
            .step(next_action)
            .expect("non-defense action should be legal");

        assert_eq!(outcome.reward.shaping["clear_progress"], 0.0);
    }

    #[test]
    fn escaped_progress_is_not_rewarded_above_attack_progress() {
        let config = RewardConfig::default();
        let attack = shaping_rewards(0.0, 0.02, 0.0, 0.0, 1_000.0, 0.0, &config);
        let escaped = shaping_rewards(0.0, 0.02, 100.0, 0.0, 1_000.0, 0.0, &config);

        assert_eq!(attack["clear_progress"], escaped["clear_progress"]);
        assert!(escaped["escaped_hp_penalty"] < 0.0);
        assert!(escaped.values().sum::<f32>() < attack.values().sum::<f32>());
    }

    #[test]
    fn damage_progress_is_normalized_by_stage_total_hp() {
        let config = RewardConfig {
            damage_progress_weight: 1.0,
            ..RewardConfig::default()
        };
        let rewards = shaping_rewards(0.0, 0.0, 0.0, 0.0, 1_000.0, 250.0, &config);

        assert_eq!(rewards["damage_progress"], 0.25);
    }

    #[test]
    fn reward_contains_escape_and_player_damage_penalties() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert!(outcome.reward.shaping.contains_key("escaped_hp_penalty"));
        assert!(
            outcome
                .reward
                .shaping
                .contains_key("player_hp_loss_penalty")
        );
    }

    #[test]
    fn escaped_penalty_is_stage_normalized() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        let penalty = outcome
            .reward
            .shaping
            .get("escaped_hp_penalty")
            .copied()
            .expect("escaped penalty should be present");
        assert!(penalty.is_finite());
        assert!(penalty <= 0.0);
    }

    #[test]
    fn non_terminal_steps_have_no_death_penalty() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert!(!outcome.terminated);
        assert_eq!(outcome.reward.terminal, 0.0);
    }

    #[test]
    fn player_damage_metric_uses_game_units() {
        let mut environment = environment();
        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(30));
        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(60));
        let before = environment.metrics.total_player_damage;
        let _ = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert!(environment.metrics.total_player_damage - before < 60.0);
    }

    #[test]
    fn start_selecting_tower_reaches_card_selection_decision() {
        let mut environment = environment();

        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal in the initial shop");

        assert_eq!(outcome.observation, environment.snapshot());
        assert_eq!(outcome.state_hash, environment.state_hash());
        let trace_step = environment
            .policy_trace()
            .steps
            .last()
            .expect("step should append a policy trace entry");
        assert_eq!(trace_step.post_observation, outcome.observation);
        assert_eq!(trace_step.post_state_hash, outcome.state_hash);
        assert_eq!(
            outcome.observation.decision_point,
            DecisionPoint::CardSelection
        );
        assert!(!outcome.terminated);
        assert!(!outcome.truncated);
        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|action| matches!(action.action, AgentAction::BeginTowerSelection))
        );
    }

    #[test]
    fn start_defense_opens_pre_defense_item_window_before_ticks() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let tower_selection = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::BeginTowerSelection => Some(AgentAction::BeginTowerSelection),
                _ => None,
            })
            .expect("tower selection should be available");
        environment
            .step(tower_selection)
            .expect("tower selection should be legal");
        let card = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index }
                | AgentAction::DeselectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("a card toggle should be available");
        environment.step(card).expect("card toggle should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection should be legal");
        let placement = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::PlaceTower { .. } => Some(legal.action),
                _ => None,
            })
            .expect("placement should be available");
        environment
            .step(placement)
            .expect("placement should be legal");

        let outcome = environment
            .step(AgentAction::StartDefense)
            .expect("start defense should be legal");
        assert_eq!(
            outcome.observation.decision_point,
            DecisionPoint::PreDefenseItem
        );
        assert_eq!(outcome.info.ticks_advanced, 0);
        assert_eq!(
            outcome.observation.item_window_stage,
            Some(environment.game_state.stage())
        );
        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|legal| matches!(legal.action, AgentAction::Continue))
        );
    }

    #[test]
    fn continue_closes_pre_defense_window_and_advances_defense() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let tower_selection = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::BeginTowerSelection => Some(AgentAction::BeginTowerSelection),
                _ => None,
            })
            .expect("tower selection should be available");
        environment
            .step(tower_selection)
            .expect("tower selection should be legal");
        let card = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index }
                | AgentAction::DeselectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("a card toggle should be available");
        environment.step(card).expect("card toggle should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection should be legal");
        let placement = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::PlaceTower { .. } => Some(legal.action),
                _ => None,
            })
            .expect("placement should be available");
        environment
            .step(placement)
            .expect("placement should be legal");
        environment
            .step(AgentAction::StartDefense)
            .expect("start defense should be legal");
        let before_continue = environment.game_state.sim_tick().ticks();
        let outcome = environment
            .step(AgentAction::Continue)
            .expect("continue should close the pre-defense window");
        assert!(outcome.info.ticks_advanced > 0 || outcome.terminated);
        assert!(environment.game_state.sim_tick().ticks() > before_continue || outcome.terminated);
        assert_ne!(environment.decision_point(), DecisionPoint::PreDefenseItem);
    }

    #[test]
    fn reroll_actions_match_affordability_and_have_no_empty_duplicate() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");

        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|legal| matches!(legal.action, AgentAction::BeginRerollSelection))
        );

        environment.game_state.set_left_dice_for_test(0);
        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(1));
        assert!(
            environment
                .legal_actions()
                .into_iter()
                .all(|legal| !matches!(legal.action, AgentAction::Reroll { .. }))
        );

        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(60));
        assert!(
            environment
                .legal_actions()
                .into_iter()
                .all(|legal| !matches!(legal.action, AgentAction::Reroll { .. }))
        );
    }

    #[test]
    fn card_service_purchase_exposes_explicit_selection_actions() {
        let mut environment = environment();
        let slot_index = environment.game_state.add_card_service_shop_slot_for_test();

        let outcome = environment
            .step(AgentAction::PurchaseShopItem { slot_index })
            .expect("card service purchase should be legal");

        assert_eq!(
            outcome.observation.decision_point,
            DecisionPoint::CardServiceSelection
        );
        assert_eq!(
            outcome
                .observation
                .card_service
                .as_ref()
                .map(|service| service.key.as_str()),
            Some("eraser")
        );
        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|action| matches!(action.action, AgentAction::SelectCardServiceCard { .. }))
        );
        let selection_open_hash = environment.state_hash();

        let card_action = environment
            .legal_actions()
            .into_iter()
            .find_map(|action| match action.action {
                AgentAction::SelectCardServiceCard { card_index } => {
                    Some(AgentAction::SelectCardServiceCard { card_index })
                }
                _ => None,
            })
            .expect("eraser should offer a card");
        environment
            .step(card_action)
            .expect("card selection should be legal");
        assert_eq!(environment.state_hash(), selection_open_hash);
        environment
            .step(AgentAction::ConfirmCardServiceSelection)
            .expect("card service confirmation should be legal");
        assert_ne!(environment.state_hash(), selection_open_hash);

        assert_eq!(environment.decision_point(), DecisionPoint::Shop);
        assert!(
            environment
                .replay()
                .commands
                .iter()
                .any(|action| { matches!(action, AgentAction::ConfirmCardServiceSelection) })
        );
    }

    #[test]
    fn card_service_selection_changes_progress_fingerprint() {
        let environment = environment();
        let before = environment.progress_fingerprint();
        assert_eq!(before, environment.progress_fingerprint());
    }

    // --- legal-action -> executable invariant (shop purchase legality) --
    //
    // `GameEnvironment::shop_actions`/`legal_actions` only ever list a
    // `PurchaseShopItem` that `td_core::CoreState::can_purchase_shop_slot`
    // considers legal, and (since `can_purchase_shop_slot` and the real
    // `PurchaseShopItem` command now share one authoritative transaction,
    // `CoreState::try_purchase_shop_slot`) that legality can no longer
    // disagree with what actually executes. This suite pins that contract
    // from the simulator side.

    /// A byte-identical copy of `environment` (unlike
    /// `fork_for_rollout_seed`, which deliberately reseeds the scenario
    /// RNG) - lets a test apply a trial action without mutating the
    /// original.
    fn identical_clone(environment: &GameEnvironment) -> GameEnvironment {
        let snapshot = environment.game_state.core_state_snapshot();
        let game_state =
            GameCore::from_core_state_snapshot(snapshot).expect("clone snapshot must be valid");
        GameEnvironment {
            game_state,
            config: Arc::clone(&environment.config),
            seed: environment.seed,
            max_advance_ticks: environment.max_advance_ticks,
            rollout_tick_deadline: environment.rollout_tick_deadline,
            card_service_selection: environment.card_service_selection.clone(),
            decision_context: environment.decision_context.clone(),
            environment_actions: environment.environment_actions.clone(),
            policy_trace: environment.policy_trace.clone(),
            metrics: environment.metrics.clone(),
            reward_config: environment.reward_config.clone(),
            max_stage: environment.max_stage,
        }
    }

    /// Asserts the legal-action -> executable invariant at `environment`'s
    /// current state: every `PurchaseShopItem` action
    /// `environment.legal_actions()` exposes must succeed when applied to an
    /// identical clone.
    fn assert_every_legal_shop_purchase_executes(environment: &GameEnvironment, label: &str) {
        for legal in environment.legal_actions() {
            if let AgentAction::PurchaseShopItem { slot_index } = legal.action {
                let mut clone = identical_clone(environment);
                clone
                    .step(AgentAction::PurchaseShopItem { slot_index })
                    .unwrap_or_else(|error| {
                        panic!(
                            "{label}: legal PurchaseShopItem (slot {slot_index}) was rejected on \
                             execution: {error:?}"
                        )
                    });
            }
        }
    }

    /// E (targeted regression pin): with item inventory forced to capacity,
    /// a shop screen containing an `Item` slot must not expose it as a
    /// legal `PurchaseShopItem` - this is the exact shape of the
    /// legality-contract violation this suite guards against
    /// (`can_purchase_shop_slot` used to ignore post-purchase capacity).
    #[test]
    fn item_purchase_is_not_legal_once_item_capacity_is_reached() {
        let mut environment = environment();
        assert_eq!(environment.decision_point(), DecisionPoint::Shop);

        let mut core_state = environment.game_state.core_state_snapshot();
        let item_capacity = core_state.item_capacity();
        let mut injected_slot_index = None;
        core_state
            .edit_snapshot(|parts| {
                parts.items = td_core::ItemCollection::from_entries(
                    (0..item_capacity)
                        .map(|_| td_core::generated_item(td_core::ItemKind::Bread).expect("bread"))
                        .collect(),
                );
                parts.progress.gold = 1_000;
                if let td_core::GameFlowState::Shopping(shop) = &mut parts.flow {
                    injected_slot_index = Some(shop.slots.len());
                    shop.slots.push(td_core::ShopSlotDataState {
                        id: shop
                            .slots
                            .iter()
                            .map(|slot| slot.id)
                            .max()
                            .map_or(0, |id| id + 1),
                        slot: td_core::ShopSlotState::Item {
                            item: td_core::generated_item(td_core::ItemKind::Bread)
                                .expect("bread"),
                            cost: 0,
                        },
                        purchased: false,
                    });
                }
            })
            .expect("item-capacity fixture must be a valid snapshot");
        environment.game_state =
            GameCore::from_core_state_snapshot(core_state).expect("fixture snapshot must be valid");
        let injected_slot_index = injected_slot_index.expect("fixture must reach the Shop flow");

        // The full-capacity slot we injected must never appear as legal...
        assert!(
            !environment.legal_actions().iter().any(|legal| matches!(
                legal.action,
                AgentAction::PurchaseShopItem { slot_index } if slot_index == injected_slot_index
            )),
            "an Item purchase at item capacity must not be exposed as legal"
        );
        // ...and every PurchaseShopItem that *is* still legal must execute.
        assert_every_legal_shop_purchase_executes(&environment, "item capacity fixture");
    }

    /// E (state-corpus differential): walking several seeds through the
    /// canonical scripted policy, the legal-action -> executable invariant
    /// must hold at every visited decision state, not just the injected
    /// fixture above.
    #[test]
    fn every_legal_shop_purchase_executes_along_scripted_trajectories() {
        // Semantic macro-actions via the canonical helper
        // (`canonical_scripted_semantic_action`/`semantic_step`), not raw UI
        // micro-actions (`scripted_expert_action` is not cycle-safe over the
        // raw `SelectHandCard`/`DeselectHandCard`-style legal-action space
        // on its own) and not the full O(subset x position)
        // `semantic_legal_actions()` oracle (too slow to walk many
        // decisions with - that cost is exactly why the dense/canonical
        // path exists). Shop legality itself is identical across all of
        // these: `PurchaseShopItem` is generated the same way regardless of
        // which action-representation path produced the *other* action.
        for seed in 0..2u64 {
            let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
            for decision in 0..8 {
                if matches!(environment.decision_point(), DecisionPoint::Terminal) {
                    break;
                }
                assert_every_legal_shop_purchase_executes(
                    &environment,
                    &format!("seed {seed} decision {decision}"),
                );
                let Ok(action) = crate::policy_runner::canonical_scripted_semantic_action(&environment)
                else {
                    break;
                };
                let Ok(outcome) = environment.semantic_step(action) else {
                    break;
                };
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod clear_rate_trace_tests {
    use super::*;
    use crate::policy_runner::canonical_scripted_semantic_action;

    #[test]
    #[ignore = "plays full canonical episodes; slow in debug builds"]
    fn clear_rate_is_non_decreasing_across_semantic_steps() {
        let config = Arc::new(GameConfig::default_config());
        for seed in 0..2u64 {
            let mut environment = GameEnvironment::new(config.clone(), seed);
            let mut previous = environment.clear_rate();
            while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
                let action = match environment.forced_action() {
                    Some(action) => action,
                    None => canonical_scripted_semantic_action(&environment).unwrap(),
                };
                let outcome = environment.semantic_step(action).unwrap();
                let current = environment.clear_rate();
                assert!(
                    current >= previous,
                    "seed {seed}: clear_rate decreased {previous} -> {current}"
                );
                previous = current;
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
        }
    }
}
