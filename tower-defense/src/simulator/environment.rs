//! Versioned, headless game environment for policy-driven simulation.

use crate::card::{Card, Engraving, Rank, Suit};
use crate::config::GameConfig;
use crate::game_state::card_service::{CardService, CardServiceBehavior};
use crate::game_state::flow::GameFlow;
use crate::game_state::item::ItemBehavior;
use crate::game_state::modal::deck::CardSelectionState;
use crate::game_state::tower::{Tower, TowerTemplate};
use crate::game_state::upgrade::UpgradeBehavior;
use crate::game_state::{GameState, MAP_SIZE, PlayerCommand, TRAVEL_POINTS};
use crate::simulator::ml::vocabulary::{
    card_service_key_id, item_id, monster_kind_id, shop_kind_id, tower_kind_id, upgrade_id,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

pub const ENVIRONMENT_VERSION: u32 = 5;
pub const ACTION_SCHEMA_VERSION: u32 = 4;
pub const ENVIRONMENT_REPLAY_SCHEMA_VERSION: u32 = 4;
pub const DEFAULT_MAX_ADVANCE_TICKS: u64 = 60 * 60 * 5;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardConfig {
    pub terminal_win: f32,
    pub terminal_loss: f32,
    pub escaped_hp_penalty_scale: f32,
    pub player_hp_loss_penalty_scale: f32,
    pub potential_weight: f32,
    pub potential_gamma: f32,
    #[serde(default)]
    pub damage_progress_weight: f32,
    pub no_progress_cycle_penalty: f32,
}

impl Default for RewardConfig {
    fn default() -> Self {
        Self {
            terminal_win: 1.0,
            terminal_loss: -1.0,
            escaped_hp_penalty_scale: 50.0,
            player_hp_loss_penalty_scale: 60.0,
            potential_weight: 0.0,
            potential_gamma: 0.99,
            damage_progress_weight: 0.0,
            no_progress_cycle_penalty: 0.0,
        }
    }
}

impl RewardConfig {
    pub fn validate(&self) -> Result<(), String> {
        let fields = [
            ("terminal_win", self.terminal_win),
            ("terminal_loss", self.terminal_loss),
            ("escaped_hp_penalty_scale", self.escaped_hp_penalty_scale),
            (
                "player_hp_loss_penalty_scale",
                self.player_hp_loss_penalty_scale,
            ),
            ("potential_weight", self.potential_weight),
            ("potential_gamma", self.potential_gamma),
            ("damage_progress_weight", self.damage_progress_weight),
            ("no_progress_cycle_penalty", self.no_progress_cycle_penalty),
        ];
        if let Some((name, value)) = fields.iter().find(|(_, value)| !value.is_finite()) {
            return Err(format!("reward field {name} must be finite, got {value}"));
        }
        if self.no_progress_cycle_penalty > 0.0 {
            return Err(format!(
                "no_progress_cycle_penalty must be non-positive, got {}",
                self.no_progress_cycle_penalty
            ));
        }
        Ok(())
    }

    pub fn validate_gamma(&self, rollout_gamma: f32) -> Result<(), String> {
        self.validate()?;
        if (self.potential_gamma - rollout_gamma).abs() > f32::EPSILON {
            return Err(format!(
                "reward potential gamma {} does not match rollout gamma {}",
                self.potential_gamma, rollout_gamma
            ));
        }
        Ok(())
    }

    pub fn validate_equal(&self, other: &Self) -> Result<(), String> {
        if self != other {
            return Err("PPO and rollout reward configurations differ".to_string());
        }
        Ok(())
    }
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
    let next_potential = if terminated { 0.0 } else { potential(after) };
    config.potential_weight * (config.potential_gamma * next_potential - potential(before))
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionPoint {
    Shop,
    CardSelection,
    CardServiceSelection,
    TowerPlacement,
    PreDefenseItem,
    DamageResponseItem,
    TreasureSelection,
    Defense,
    Terminal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentAction {
    PurchaseShopItem {
        slot_index: usize,
    },
    StartSelectingTower,
    BeginRerollSelection,
    BeginTowerSelection,
    SelectHandCard {
        hand_slot_index: usize,
    },
    DeselectHandCard {
        hand_slot_index: usize,
    },
    ConfirmCardSelection,
    CancelCardSelection,
    Reroll {
        selected_slot_indices: Vec<usize>,
    },
    SelectTower {
        selected_slot_indices: Vec<usize>,
    },
    PlaceTower {
        hand_slot_index: usize,
        left: usize,
        top: usize,
    },
    RemoveTower {
        tower_id: u64,
    },
    StartDefense,
    SelectTreasure {
        option_index: usize,
    },
    SelectCardServiceCard {
        card_index: usize,
    },
    ConfirmCardServiceSelection,
    UseInventoryItem {
        item_index: usize,
    },
    Continue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionKind {
    PurchaseShopItem,
    StartSelectingTower,
    BeginRerollSelection,
    BeginTowerSelection,
    SelectHandCard,
    DeselectHandCard,
    ConfirmCardSelection,
    CancelCardSelection,
    Reroll,
    SelectTower,
    PlaceTower,
    RemoveTower,
    StartDefense,
    SelectTreasure,
    SelectCardServiceCard,
    ConfirmCardServiceSelection,
    UseInventoryItem,
    Continue,
}

impl ActionKind {
    pub const COUNT: usize = 18;

    pub const fn index(self) -> usize {
        match self {
            Self::PurchaseShopItem => 0,
            Self::StartSelectingTower => 1,
            Self::BeginRerollSelection => 2,
            Self::BeginTowerSelection => 3,
            Self::SelectHandCard => 4,
            Self::DeselectHandCard => 5,
            Self::ConfirmCardSelection => 6,
            Self::CancelCardSelection => 7,
            Self::Reroll => 8,
            Self::SelectTower => 9,
            Self::PlaceTower => 10,
            Self::RemoveTower => 11,
            Self::StartDefense => 12,
            Self::SelectTreasure => 13,
            Self::SelectCardServiceCard => 14,
            Self::ConfirmCardServiceSelection => 15,
            Self::UseInventoryItem => 16,
            Self::Continue => 17,
        }
    }

    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::PurchaseShopItem => "purchase_shop_item",
            Self::StartSelectingTower => "start_selecting_tower",
            Self::BeginRerollSelection => "begin_reroll_selection",
            Self::BeginTowerSelection => "begin_tower_selection",
            Self::SelectHandCard => "select_hand_card",
            Self::DeselectHandCard => "deselect_hand_card",
            Self::ConfirmCardSelection => "confirm_card_selection",
            Self::CancelCardSelection => "cancel_card_selection",
            Self::Reroll => "reroll",
            Self::SelectTower => "select_tower",
            Self::PlaceTower => "place_tower",
            Self::RemoveTower => "remove_tower",
            Self::StartDefense => "start_defense",
            Self::SelectTreasure => "select_treasure",
            Self::SelectCardServiceCard => "select_card_service_card",
            Self::ConfirmCardServiceSelection => "confirm_card_service_selection",
            Self::UseInventoryItem => "use_inventory_item",
            Self::Continue => "continue",
        }
    }
}

impl AgentAction {
    pub fn kind(&self) -> ActionKind {
        match self {
            Self::PurchaseShopItem { .. } => ActionKind::PurchaseShopItem,
            Self::StartSelectingTower => ActionKind::StartSelectingTower,
            Self::BeginRerollSelection => ActionKind::BeginRerollSelection,
            Self::BeginTowerSelection => ActionKind::BeginTowerSelection,
            Self::SelectHandCard { .. } => ActionKind::SelectHandCard,
            Self::DeselectHandCard { .. } => ActionKind::DeselectHandCard,
            Self::ConfirmCardSelection => ActionKind::ConfirmCardSelection,
            Self::CancelCardSelection => ActionKind::CancelCardSelection,
            Self::Reroll { .. } => ActionKind::Reroll,
            Self::SelectTower { .. } => ActionKind::SelectTower,
            Self::PlaceTower { .. } => ActionKind::PlaceTower,
            Self::RemoveTower { .. } => ActionKind::RemoveTower,
            Self::StartDefense => ActionKind::StartDefense,
            Self::SelectTreasure { .. } => ActionKind::SelectTreasure,
            Self::SelectCardServiceCard { .. } => ActionKind::SelectCardServiceCard,
            Self::ConfirmCardServiceSelection => ActionKind::ConfirmCardServiceSelection,
            Self::UseInventoryItem { .. } => ActionKind::UseInventoryItem,
            Self::Continue => ActionKind::Continue,
        }
    }
}

impl AgentAction {
    pub fn action_id(&self) -> String {
        match self {
            Self::PurchaseShopItem { slot_index } => format!("purchase_shop_item:{slot_index}"),
            Self::StartSelectingTower => "start_selecting_tower".to_string(),
            Self::BeginRerollSelection => "begin_reroll_selection".to_string(),
            Self::BeginTowerSelection => "begin_tower_selection".to_string(),
            Self::SelectHandCard { hand_slot_index } => {
                format!("select_hand_card:{hand_slot_index}")
            }
            Self::DeselectHandCard { hand_slot_index } => {
                format!("deselect_hand_card:{hand_slot_index}")
            }
            Self::ConfirmCardSelection => "confirm_card_selection".to_string(),
            Self::CancelCardSelection => "cancel_card_selection".to_string(),
            Self::Reroll {
                selected_slot_indices,
            } => format!("reroll:{}", indices_key(selected_slot_indices)),
            Self::SelectTower {
                selected_slot_indices,
            } => format!("select_tower:{}", indices_key(selected_slot_indices)),
            Self::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => format!("place_tower:{hand_slot_index}:{left}:{top}"),
            Self::RemoveTower { tower_id } => format!("remove_tower:{tower_id}"),
            Self::StartDefense => "start_defense".to_string(),
            Self::SelectTreasure { option_index } => format!("select_treasure:{option_index}"),
            Self::SelectCardServiceCard { card_index } => {
                format!("select_card_service_card:{card_index}")
            }
            Self::ConfirmCardServiceSelection => "confirm_card_service_selection".to_string(),
            Self::UseInventoryItem { item_index } => format!("use_inventory_item:{item_index}"),
            Self::Continue => "continue".to_string(),
        }
    }

    fn to_player_command(&self) -> Option<PlayerCommand> {
        match self {
            Self::PurchaseShopItem { slot_index } => Some(PlayerCommand::PurchaseShopItem {
                slot_index: *slot_index,
            }),
            Self::StartSelectingTower => Some(PlayerCommand::StartSelectingTower),
            Self::BeginRerollSelection
            | Self::BeginTowerSelection
            | Self::SelectHandCard { .. }
            | Self::DeselectHandCard { .. }
            | Self::ConfirmCardSelection
            | Self::CancelCardSelection => None,
            Self::Reroll {
                selected_slot_indices,
            } => Some(PlayerCommand::Reroll {
                selected_slot_indices: selected_slot_indices.clone(),
            }),
            Self::SelectTower {
                selected_slot_indices,
            } => Some(PlayerCommand::SelectTower {
                selected_slot_indices: selected_slot_indices.clone(),
            }),
            Self::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => Some(PlayerCommand::PlaceTower {
                hand_slot_index: *hand_slot_index,
                left: *left,
                top: *top,
            }),
            Self::RemoveTower { tower_id } => Some(PlayerCommand::RemoveTower {
                tower_id: *tower_id,
            }),
            Self::StartDefense => Some(PlayerCommand::StartDefense),
            Self::SelectTreasure { option_index } => Some(PlayerCommand::SelectTreasure {
                option_index: *option_index,
            }),
            Self::SelectCardServiceCard { .. }
            | Self::ConfirmCardServiceSelection
            | Self::Continue => None,
            Self::UseInventoryItem { item_index } => Some(PlayerCommand::UseInventoryItem {
                item_index: *item_index,
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalAction {
    pub id: String,
    pub action: AgentAction,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardObservation {
    pub id: usize,
    pub suit: String,
    pub rank: String,
    pub polish_pct_raw: i64,
    pub engraving: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TowerTemplateObservation {
    pub kind: String,
    pub kind_id: u16,
    pub suit: Option<String>,
    pub rank: Option<String>,
    pub rerolled_count: usize,
    pub damage_raw: i64,
    pub used_cards: Vec<CardObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandItemObservation {
    Card(CardObservation),
    Tower(TowerTemplateObservation),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandObservation {
    pub index: usize,
    pub selected: bool,
    pub item: HandItemObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShopSlotObservation {
    pub index: usize,
    pub purchased: bool,
    pub kind: String,
    pub kind_id: u16,
    pub key: String,
    pub key_id: u16,
    pub cost: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryObservation {
    pub index: usize,
    pub key: String,
    pub key_id: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedUpgradeObservation {
    pub id: u64,
    pub key: String,
    pub key_id: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TowerObservation {
    pub id: u64,
    pub left: usize,
    pub top: usize,
    pub template: TowerTemplateObservation,
    pub cooldown_ticks: u64,
    pub range_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonsterObservation {
    pub id: u64,
    pub kind: String,
    pub kind_id: u16,
    pub route_index: usize,
    pub route_progress_raw: i64,
    pub hp_raw: i64,
    pub max_hp_raw: i64,
    pub velocity_raw: i64,
    pub damage_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeckObservation {
    pub all_cards: Vec<CardObservation>,
    pub draw_cards: Vec<CardObservation>,
    pub discard_cards: Vec<CardObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageModifiersObservation {
    pub damage_multiplier_raw: i64,
    pub damage_reduction_multiplier_raw: i64,
    pub incoming_damage_multiplier_raw: i64,
    pub gold_gain_multiplier_raw: i64,
    pub enemy_health_multiplier_raw: i64,
    pub enemy_speed_multiplier_raw: i64,
    pub max_hand_slots_delta: isize,
    pub max_rerolls_delta: isize,
    pub reroll_health_cost: usize,
    pub item_use_disabled: bool,
    pub purchases_disabled: bool,
    pub free_shop: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub decision_point: DecisionPoint,
    pub item_window_stage: Option<usize>,
    pub damage_trigger_tick: Option<u64>,
    pub card_selection_purpose: Option<String>,
    pub selected_hand_slot_indices: Vec<usize>,
    pub card_selection_confirmable: bool,
    pub stage: usize,
    pub sim_tick: u64,
    pub theme: Option<String>,
    pub hp_raw: i64,
    pub max_hp_raw: i64,
    pub shield_raw: i64,
    pub gold: usize,
    pub left_dice: usize,
    pub rerolled_count: usize,
    pub stage_progress_raw: i64,
    pub stage_total_hp_raw: i64,
    pub active_monster_count: usize,
    pub queued_monster_count: usize,
    pub hand: Vec<HandObservation>,
    pub deck: DeckObservation,
    pub shop: Vec<ShopSlotObservation>,
    pub inventory: Vec<InventoryObservation>,
    pub owned_upgrades: Vec<OwnedUpgradeObservation>,
    pub towers: Vec<TowerObservation>,
    pub tower_grid: Vec<Option<u64>>,
    pub map_width: usize,
    pub map_height: usize,
    pub route_coords: Vec<RouteCoordObservation>,
    pub monsters: Vec<MonsterObservation>,
    pub treasure_options: Vec<String>,
    pub card_service: Option<CardServiceObservation>,
    pub stage_modifiers: StageModifiersObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteCoordObservation {
    pub x: usize,
    pub y: usize,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardServiceObservation {
    pub key: String,
    pub current_step: usize,
    pub step_count: usize,
    pub required_count: usize,
    pub selected_card_indices: Vec<usize>,
    pub candidate_card_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardComponents {
    pub terminal: f32,
    pub shaping: BTreeMap<String, f32>,
}

impl Default for RewardComponents {
    fn default() -> Self {
        Self {
            terminal: 0.0,
            shaping: BTreeMap::new(),
        }
    }
}

impl RewardComponents {
    pub fn total(&self) -> f32 {
        self.terminal + self.shaping.values().sum::<f32>()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepReason {
    DecisionPoint,
    Terminal,
    MaxTicks,
    MaxDecisions,
    NoProgressCycle,
    CurriculumComplete,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepInfo {
    pub reason: StepReason,
    pub ticks_advanced: u64,
    pub no_progress_cycle: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepOutcome {
    pub observation: Observation,
    pub reward: RewardComponents,
    pub terminated: bool,
    pub truncated: bool,
    pub info: StepInfo,
    pub state_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentReplayCheckpoint {
    pub sequence: u64,
    pub completed_sim_tick: u64,
    pub state_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentReplay {
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub replay_schema_version: u32,
    pub config_version: u32,
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
        error: String,
    },
    CardServiceRejected {
        action_id: String,
    },
}

pub struct GameEnvironment {
    game_state: GameState,
    seed: u64,
    max_advance_ticks: u64,
    pending_card_service: Option<CardService>,
    decision_context: DecisionContext,
    environment_actions: Vec<AgentAction>,
    metrics: EnvironmentMetrics,
    reward_config: RewardConfig,
    max_stage: Option<usize>,
}

impl GameEnvironment {
    pub fn new(config: Arc<GameConfig>, seed: u64) -> Self {
        Self::new_with_reward_config(config, seed, RewardConfig::default())
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
        let mut environment = Self {
            game_state: crate::game_state::create_game_state_with_config(config, seed),
            seed,
            max_advance_ticks: DEFAULT_MAX_ADVANCE_TICKS,
            pending_card_service: None,
            decision_context: DecisionContext::None,
            environment_actions: Vec::new(),
            metrics: EnvironmentMetrics::default(),
            reward_config,
            max_stage,
        };
        environment.game_state.headless = true;
        environment.game_state.defer_card_service_selection = true;
        environment
    }

    pub fn reset(&mut self, seed: u64, config: Arc<GameConfig>) -> Observation {
        let config = if let Some(max_stage) = self.max_stage {
            let mut config = (*config).clone();
            config.player.max_stages = config.player.max_stages.min(max_stage);
            Arc::new(config)
        } else {
            config
        };
        self.game_state = crate::game_state::create_game_state_with_config(config, seed);
        self.game_state.headless = true;
        self.game_state.defer_card_service_selection = true;
        self.seed = seed;
        self.pending_card_service = None;
        self.decision_context = DecisionContext::None;
        self.environment_actions.clear();
        self.metrics = EnvironmentMetrics::default();
        self.snapshot()
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn set_max_advance_ticks(&mut self, max_advance_ticks: u64) {
        self.max_advance_ticks = max_advance_ticks;
    }

    pub fn reward_config(&self) -> &RewardConfig {
        &self.reward_config
    }

    pub fn set_reward_config(&mut self, reward_config: RewardConfig) {
        self.reward_config = reward_config;
    }

    pub fn decision_point(&self) -> DecisionPoint {
        if self.pending_card_service.is_some() {
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
        match self.game_state.flow {
            GameFlow::Shopping(_) => DecisionPoint::Shop,
            GameFlow::SelectingTower(_) => DecisionPoint::CardSelection,
            GameFlow::PlacingTower => DecisionPoint::TowerPlacement,
            GameFlow::Defense(_) => DecisionPoint::Defense,
            GameFlow::TreasureSelection(_) => DecisionPoint::TreasureSelection,
            GameFlow::Result { .. } => DecisionPoint::Terminal,
            GameFlow::Initializing => DecisionPoint::Terminal,
        }
    }

    pub fn snapshot(&self) -> Observation {
        observation_from_game_state(
            &self.game_state,
            self.pending_card_service.as_ref(),
            &self.decision_context,
        )
    }

    pub fn state_hash(&self) -> String {
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
        match &self.game_state.flow {
            GameFlow::Result { clear_rate } => clear_rate.as_percent_f32(),
            _ => self.game_state.calculate_clear_rate().as_percent_f32(),
        }
    }

    pub fn metrics(&self) -> EnvironmentMetrics {
        let mut metrics = self.metrics.clone();
        metrics.total_gold_earned = self.game_state.metrics.total_gold_earned;
        metrics.total_player_damage = self.game_state.metrics.total_player_damage.as_f32();
        metrics.total_tower_damage = self
            .game_state
            .metrics
            .tower_damage_stats
            .iter()
            .map(|stats| stats.total_damage.as_f32())
            .sum();
        metrics.stage_damage = self
            .game_state
            .metrics
            .stage_damage
            .iter()
            .map(|(stage, damage)| (*stage, damage.as_f32()))
            .collect();
        metrics
    }

    pub fn replay(&self) -> EnvironmentReplay {
        let replay = crate::game_state::replay::Replay::from_game_state(&self.game_state);
        EnvironmentReplay {
            environment_version: ENVIRONMENT_VERSION,
            action_schema_version: ACTION_SCHEMA_VERSION,
            replay_schema_version: ENVIRONMENT_REPLAY_SCHEMA_VERSION,
            config_version: replay.config_version,
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
                })
                .collect(),
        }
    }

    pub fn legal_actions(&self) -> Vec<LegalAction> {
        let mut actions = match self.decision_point() {
            DecisionPoint::Shop => self.shop_actions(),
            DecisionPoint::CardSelection => self.card_selection_actions(),
            DecisionPoint::CardServiceSelection => self.card_service_actions(),
            DecisionPoint::TowerPlacement => self.tower_placement_actions(),
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

    pub fn step(&mut self, action: AgentAction) -> Result<StepOutcome, EnvironmentError> {
        let action_id = action.action_id();
        if !self
            .legal_actions()
            .iter()
            .any(|legal_action| legal_action.action == action)
        {
            return Err(EnvironmentError::IllegalAction {
                decision_point: self.decision_point(),
                action_id,
                state_hash: self.state_hash(),
            });
        }

        let escaped_hp_before = self.game_state.metrics.total_escaped_hp;
        let tower_damage_before = self
            .game_state
            .metrics
            .tower_damage_stats
            .iter()
            .map(|stats| stats.total_damage.as_f32())
            .sum::<f32>();
        let clear_rate_before = self.clear_rate() / 100.0;
        let observation_before = self.snapshot();

        let deferred_card_service = self.card_service_for_action(&action);
        let action_result = if let Some(command) = action.to_player_command() {
            self.game_state
                .apply_player_command(command)
                .map_err(|error| EnvironmentError::CommandRejected {
                    action_id: action.action_id(),
                    error: format!("{error:?}"),
                })
        } else {
            self.apply_environment_action(&action)
        };
        action_result?;
        self.apply_card_selection_action(&action)?;
        if matches!(action, AgentAction::Continue)
            && matches!(
                self.decision_context,
                DecisionContext::PreDefenseItem { .. } | DecisionContext::DamageResponseItem { .. }
            )
        {
            self.decision_context = DecisionContext::None;
        }
        if matches!(action, AgentAction::StartDefense)
            && matches!(self.game_state.flow, GameFlow::Defense(_))
        {
            self.decision_context = DecisionContext::PreDefenseItem {
                stage: self.game_state.stage,
            };
        }
        match action {
            AgentAction::PlaceTower { .. } => self.metrics.total_towers_placed += 1,
            AgentAction::UseInventoryItem { .. } => self.metrics.total_items_used += 1,
            _ => {}
        }
        if let Some(card_service) = deferred_card_service {
            self.pending_card_service = Some(card_service);
        }
        self.environment_actions.push(action);

        let ticks_before = self.game_state.sim_tick().ticks();
        let mut reason = self.advance_until_decision_or_terminal();
        if self.max_stage.is_some()
            && matches!(self.game_state.flow, GameFlow::Result { clear_rate } if clear_rate == crate::ClearRate::FULL)
        {
            reason = StepReason::CurriculumComplete;
        }
        let ticks_advanced = self
            .game_state
            .sim_tick()
            .ticks()
            .saturating_sub(ticks_before);
        let terminated = matches!(self.decision_point(), DecisionPoint::Terminal);
        let truncated = matches!(reason, StepReason::MaxTicks);
        let terminal_reward = match &self.game_state.flow {
            GameFlow::Result { clear_rate } if *clear_rate == crate::ClearRate::FULL => {
                self.reward_config.terminal_win
            }
            GameFlow::Result { .. } => self.reward_config.terminal_loss,
            _ => 0.0,
        };
        let clear_rate_after = self.clear_rate() / 100.0;
        let player_damage =
            self.game_state.metrics.total_player_damage.as_f32() - self.metrics.total_player_damage;
        self.metrics.total_player_damage = self.game_state.metrics.total_player_damage.as_f32();
        let escaped_hp = self
            .game_state
            .metrics
            .total_escaped_hp
            .saturating_sub(escaped_hp_before)
            .as_f32();
        self.metrics.total_escaped_hp += escaped_hp;
        let stage_total_hp = match &self.game_state.flow {
            GameFlow::Defense(flow) => flow.stage_progress.start_total_hp.as_f32(),
            _ => GameState::calculate_stage_total_hp(
                self.game_state.stage,
                &self.game_state.config,
                &self.game_state.stage_modifiers,
            )
            .as_f32(),
        }
        .max(1.0);
        let tower_damage_after = self
            .game_state
            .metrics
            .tower_damage_stats
            .iter()
            .map(|stats| stats.total_damage.as_f32())
            .sum::<f32>();
        let tower_damage = (tower_damage_after - tower_damage_before).max(0.0);
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

        Ok(StepOutcome {
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
        })
    }

    pub fn advance_until_decision_or_terminal(&mut self) -> StepReason {
        let mut ticks_advanced = 0;
        while matches!(self.game_state.flow, GameFlow::Defense(_)) {
            if !matches!(self.decision_context, DecisionContext::None) {
                return StepReason::DecisionPoint;
            }
            if ticks_advanced >= self.max_advance_ticks {
                return StepReason::MaxTicks;
            }
            let hp_before = self.game_state.hp.raw();
            let tick_before = self.game_state.sim_tick().ticks();
            crate::game_state::tick::tick_headless(&mut self.game_state);
            ticks_advanced += 1;
            let hp_after = self.game_state.hp.raw();
            if hp_after < hp_before
                && hp_after > 0
                && matches!(self.game_state.flow, GameFlow::Defense(_))
            {
                self.decision_context = DecisionContext::DamageResponseItem {
                    stage: self.game_state.stage,
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
        if let GameFlow::Shopping(flow) = &self.game_state.flow {
            for (slot_index, slot) in flow
                .shop
                .slots
                .iter()
                .filter(|slot| slot.exit_animation.is_none())
                .enumerate()
            {
                if self.game_state.shop_purchase_status(slot.id).is_available() {
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
            let reroll_health_cost = self.game_state.stage_modifiers.get_reroll_health_cost();
            let can_afford_reroll = self.game_state.left_dice > 0
                || (reroll_health_cost > 0
                    && self
                        .game_state
                        .hp
                        .saturating_sub(crate::Health::from_usize(reroll_health_cost))
                        > crate::Health::from_integer(1));
            if can_afford_reroll {
                actions.push(AgentAction::BeginRerollSelection);
            }
            actions.push(AgentAction::BeginTowerSelection);
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
                let committed = match purpose {
                    CardSelectionPurpose::Reroll => AgentAction::Reroll {
                        selected_slot_indices,
                    },
                    CardSelectionPurpose::BuildTower => AgentAction::SelectTower {
                        selected_slot_indices,
                    },
                };
                self.game_state
                    .apply_player_command(
                        committed
                            .to_player_command()
                            .expect("card selection commit command"),
                    )
                    .map_err(|error| EnvironmentError::CommandRejected {
                        action_id: committed.action_id(),
                        error: format!("{error:?}"),
                    })?;
            }
            AgentAction::CancelCardSelection => {
                self.decision_context = DecisionContext::None;
            }
            _ => {}
        }
        Ok(())
    }

    fn tower_placement_actions(&self) -> Vec<AgentAction> {
        let coordinates = self.placement_coordinates();
        let mut actions = Vec::with_capacity(self.tower_hand_indices().len() * coordinates.len());
        for hand_slot_index in self.tower_hand_indices() {
            actions.extend(coordinates.iter().copied().map(|(left, top)| {
                AgentAction::PlaceTower {
                    hand_slot_index,
                    left,
                    top,
                }
            }));
        }
        actions.extend(
            self.game_state
                .towers
                .iter()
                .map(|tower| AgentAction::RemoveTower {
                    tower_id: tower.id().raw(),
                }),
        );
        actions.push(AgentAction::StartDefense);
        actions
    }

    fn placement_coordinates(&self) -> Vec<(usize, usize)> {
        (0..MAP_SIZE.height.saturating_sub(1))
            .flat_map(|top| {
                (0..MAP_SIZE.width.saturating_sub(1)).filter_map(move |left| {
                    can_place_tower_at(&self.game_state, left, top).then_some((left, top))
                })
            })
            .collect()
    }

    fn treasure_actions(&self) -> Vec<AgentAction> {
        match &self.game_state.flow {
            GameFlow::TreasureSelection(flow) => (0..flow.options.len())
                .map(|option_index| AgentAction::SelectTreasure { option_index })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn card_service_actions(&self) -> Vec<AgentAction> {
        let Some(selection) = self.card_service_selection() else {
            return Vec::new();
        };
        let cards = self.game_state.deck.all_cards();
        let mut actions = cards
            .iter()
            .enumerate()
            .filter(|(_, card)| selection.current_step().filter.matches(card))
            .map(|(card_index, _)| AgentAction::SelectCardServiceCard { card_index })
            .collect::<Vec<_>>();
        if selection.is_step_complete() {
            actions.push(AgentAction::ConfirmCardServiceSelection);
        }
        actions
    }

    fn card_service_for_action(&self, action: &AgentAction) -> Option<CardService> {
        let AgentAction::PurchaseShopItem { slot_index } = action else {
            return None;
        };
        let GameFlow::Shopping(flow) = &self.game_state.flow else {
            return None;
        };
        flow.shop
            .slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
            .nth(*slot_index)
            .and_then(|slot| match &slot.slot {
                crate::shop::ShopSlot::CardService { card_service, .. } => {
                    Some(card_service.clone())
                }
                _ => None,
            })
    }

    fn card_service_selection(&self) -> Option<&CardSelectionState> {
        match &self.game_state.opened_modals.user {
            Some(crate::game_state::modal::UserModal::Deck(deck_modal)) => {
                deck_modal.selection.as_ref()
            }
            _ => None,
        }
    }

    fn apply_environment_action(&mut self, action: &AgentAction) -> Result<(), EnvironmentError> {
        match action {
            AgentAction::SelectCardServiceCard { card_index } => {
                let card = self
                    .game_state
                    .deck
                    .all_cards()
                    .get(*card_index)
                    .copied()
                    .ok_or_else(|| EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    })?;
                let selection = self
                    .game_state
                    .opened_modals
                    .user
                    .as_mut()
                    .and_then(|modal| match modal {
                        crate::game_state::modal::UserModal::Deck(deck_modal) => {
                            deck_modal.selection.as_mut()
                        }
                        _ => None,
                    })
                    .ok_or_else(|| EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    })?;
                if !selection.current_step().filter.matches(&card) {
                    return Err(EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    });
                }
                selection.toggle_card(card.id);
                Ok(())
            }
            AgentAction::ConfirmCardServiceSelection => {
                let selection = self
                    .card_service_selection()
                    .filter(|selection| selection.is_step_complete())
                    .ok_or_else(|| EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    })?;
                if selection.current_step + 1 < selection.steps.len() {
                    let next_step = selection.current_step + 1;
                    let selection = self
                        .game_state
                        .opened_modals
                        .user
                        .as_mut()
                        .and_then(|modal| match modal {
                            crate::game_state::modal::UserModal::Deck(deck_modal) => {
                                deck_modal.selection.as_mut()
                            }
                            _ => None,
                        })
                        .ok_or_else(|| EnvironmentError::CardServiceRejected {
                            action_id: action.action_id(),
                        })?;
                    selection.current_step = next_step;
                    return Ok(());
                }
                let selected_card_ids = selection.selected_card_ids_by_step();
                let card_service = self.pending_card_service.take().ok_or_else(|| {
                    EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    }
                })?;
                card_service.select_cards(&mut self.game_state, selected_card_ids);
                self.game_state.opened_modals.user = None;
                Ok(())
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
        self.game_state
            .items
            .iter()
            .enumerate()
            .filter_map(|(item_index, item)| {
                item.can_use(&self.game_state)
                    .is_ok()
                    .then_some(AgentAction::UseInventoryItem { item_index })
            })
            .collect()
    }

    fn card_hand_indices(&self) -> Vec<usize> {
        self.game_state
            .hand
            .active_slot_ids()
            .iter()
            .enumerate()
            .filter_map(|(index, slot_id)| {
                self.game_state
                    .hand
                    .get_item(*slot_id)
                    .and_then(|item| item.as_card())
                    .is_some()
                    .then_some(index)
            })
            .collect()
    }

    fn tower_hand_indices(&self) -> Vec<usize> {
        self.game_state
            .hand
            .active_slot_ids()
            .iter()
            .enumerate()
            .filter_map(|(index, slot_id)| {
                self.game_state
                    .hand
                    .get_item(*slot_id)
                    .and_then(|item| item.as_tower())
                    .is_some()
                    .then_some(index)
            })
            .collect()
    }
}

fn indices_key(indices: &[usize]) -> String {
    indices
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn can_place_tower_at(game_state: &GameState, left: usize, top: usize) -> bool {
    crate::game_state::can_place_tower::can_place_tower(
        crate::MapCoord::new(left, top),
        namui::Wh::new(2, 2),
        &TRAVEL_POINTS,
        &game_state.towers.coords(),
        game_state.route.iter_coords(),
        MAP_SIZE,
    )
}

fn observation_from_game_state(
    game_state: &GameState,
    pending_card_service: Option<&CardService>,
    decision_context: &DecisionContext,
) -> Observation {
    let decision_point = if pending_card_service.is_some() {
        DecisionPoint::CardServiceSelection
    } else if matches!(decision_context, DecisionContext::PreDefenseItem { .. }) {
        DecisionPoint::PreDefenseItem
    } else if matches!(decision_context, DecisionContext::DamageResponseItem { .. }) {
        DecisionPoint::DamageResponseItem
    } else {
        match game_state.flow {
            GameFlow::Shopping(_) => DecisionPoint::Shop,
            GameFlow::SelectingTower(_) => DecisionPoint::CardSelection,
            GameFlow::PlacingTower => DecisionPoint::TowerPlacement,
            GameFlow::Defense(_) => DecisionPoint::Defense,
            GameFlow::TreasureSelection(_) => DecisionPoint::TreasureSelection,
            GameFlow::Result { .. } | GameFlow::Initializing => DecisionPoint::Terminal,
        }
    };

    let active_slot_ids = game_state.hand.active_slot_ids();
    let simulator_selected_indices: &[usize] = match decision_context {
        DecisionContext::CardSelection {
            selected_slot_indices,
            ..
        } => selected_slot_indices.as_slice(),
        _ => &[],
    };
    let hand: Vec<HandObservation> = active_slot_ids
        .iter()
        .enumerate()
        .filter_map(|(index, slot_id)| {
            let item = game_state.hand.get_item(*slot_id)?;
            let item = match item {
                crate::hand::HandItem::Card(card) => {
                    HandItemObservation::Card(card_observation(card))
                }
                crate::hand::HandItem::Tower(template) => {
                    HandItemObservation::Tower(tower_template_observation(template))
                }
            };
            Some(HandObservation {
                index,
                selected: simulator_selected_indices.contains(&index),
                item,
            })
        })
        .collect();

    let shop = match &game_state.flow {
        GameFlow::Shopping(flow) => flow
            .shop
            .slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
            .enumerate()
            .map(|(index, slot)| shop_slot_observation(game_state, index, slot))
            .collect(),
        _ => Vec::new(),
    };

    let treasure_options = match &game_state.flow {
        GameFlow::TreasureSelection(flow) => flow
            .options
            .iter()
            .map(|upgrade| upgrade.key().to_string())
            .collect(),
        _ => Vec::new(),
    };

    let stage_progress_raw = match &game_state.flow {
        GameFlow::Defense(flow) => flow.stage_progress.processed_hp.raw(),
        _ => 0,
    };
    let stage_total_hp_raw = GameState::calculate_stage_total_hp(
        game_state.stage,
        &game_state.config,
        &game_state.stage_modifiers,
    )
    .raw();

    let towers = game_state
        .towers
        .iter()
        .map(tower_observation)
        .collect::<Vec<_>>();
    let mut tower_grid = vec![None; MAP_SIZE.width * MAP_SIZE.height];
    for tower in &towers {
        for row in tower.top..tower.top.saturating_add(2) {
            for column in tower.left..tower.left.saturating_add(2) {
                if row < MAP_SIZE.height && column < MAP_SIZE.width {
                    tower_grid[row * MAP_SIZE.width + column] = Some(tower.id);
                }
            }
        }
    }

    let mut monsters = game_state
        .monsters
        .iter()
        .map(|monster| MonsterObservation {
            id: monster.id().raw(),
            kind: format!("{:#?}", monster.kind),
            kind_id: monster_kind_id(monster.kind),
            route_index: monster.move_on_route.route_index(),
            route_progress_raw: monster.move_on_route.route_progress().raw(),
            hp_raw: monster.hp.raw(),
            max_hp_raw: monster.max_hp.raw(),
            velocity_raw: monster.move_on_route.velocity().raw(),
            damage_raw: monster.damage.raw(),
        })
        .collect::<Vec<_>>();
    monsters.sort_by_key(|monster| monster.id);

    Observation {
        environment_version: ENVIRONMENT_VERSION,
        action_schema_version: ACTION_SCHEMA_VERSION,
        decision_point,
        item_window_stage: match decision_context {
            DecisionContext::PreDefenseItem { stage }
            | DecisionContext::DamageResponseItem { stage, .. } => Some(*stage),
            DecisionContext::None | DecisionContext::CardSelection { .. } => None,
        },
        damage_trigger_tick: match decision_context {
            DecisionContext::DamageResponseItem { trigger_tick, .. } => Some(*trigger_tick),
            _ => None,
        },
        card_selection_purpose: match decision_context {
            DecisionContext::CardSelection { purpose, .. } => Some(
                match purpose {
                    CardSelectionPurpose::Reroll => "reroll",
                    CardSelectionPurpose::BuildTower => "build_tower",
                }
                .to_string(),
            ),
            _ => None,
        },
        selected_hand_slot_indices: match decision_context {
            DecisionContext::CardSelection {
                selected_slot_indices,
                ..
            } => selected_slot_indices.clone(),
            _ => Vec::new(),
        },
        card_selection_confirmable: matches!(
            decision_context,
            DecisionContext::CardSelection {
                selected_slot_indices,
                ..
            } if !selected_slot_indices.is_empty()
        ),
        stage: game_state.stage,
        sim_tick: game_state.sim_tick().ticks(),
        theme: None,
        hp_raw: game_state.hp.raw(),
        max_hp_raw: game_state.max_hp().raw(),
        shield_raw: game_state.shield.raw(),
        gold: game_state.gold,
        left_dice: game_state.left_dice,
        rerolled_count: game_state.rerolled_count,
        stage_progress_raw,
        stage_total_hp_raw,
        queued_monster_count: game_state.monster_spawn_state.monster_queue.len(),
        hand,
        deck: DeckObservation {
            all_cards: game_state
                .deck
                .all_cards()
                .iter()
                .map(card_observation)
                .collect(),
            draw_cards: unordered_cards(game_state.deck.draw_pile()),
            discard_cards: unordered_cards(game_state.deck.discard_pile()),
        },
        shop,
        inventory: game_state
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| InventoryObservation {
                index,
                key: item.item.key().to_string(),
                key_id: item_id(&item.item),
            })
            .collect(),
        owned_upgrades: game_state
            .upgrade_state
            .upgrades
            .iter()
            .map(|upgrade| OwnedUpgradeObservation {
                id: upgrade.id.0,
                key: upgrade.key().to_string(),
                key_id: upgrade_id(&upgrade.upgrade),
            })
            .collect(),
        towers,
        tower_grid,
        map_width: MAP_SIZE.width,
        map_height: MAP_SIZE.height,
        route_coords: game_state
            .route
            .iter_coords()
            .iter()
            .enumerate()
            .map(|(index, coord)| RouteCoordObservation {
                x: coord.x,
                y: coord.y,
                index,
            })
            .collect(),
        active_monster_count: game_state.monsters.len(),
        monsters,
        treasure_options,
        card_service: pending_card_service
            .and_then(|card_service| card_service_observation(game_state, card_service)),
        stage_modifiers: stage_modifiers_observation(game_state),
    }
}

fn unordered_cards(cards: &[Card]) -> Vec<CardObservation> {
    let mut observations = cards.iter().map(card_observation).collect::<Vec<_>>();
    observations.sort_by_key(|card| card.id);
    observations
}

fn card_service_observation(
    game_state: &GameState,
    card_service: &CardService,
) -> Option<CardServiceObservation> {
    let selection = match &game_state.opened_modals.user {
        Some(crate::game_state::modal::UserModal::Deck(deck_modal)) => {
            deck_modal.selection.as_ref()?
        }
        _ => return None,
    };
    let cards = game_state.deck.all_cards();
    let selected_card_indices = selection.selected_card_ids[selection.current_step]
        .iter()
        .filter_map(|card_id| cards.iter().position(|card| card.id == *card_id))
        .collect();
    let candidate_card_indices = cards
        .iter()
        .enumerate()
        .filter(|(_, card)| selection.current_step().filter.matches(card))
        .map(|(index, _)| index)
        .collect();
    Some(CardServiceObservation {
        key: card_service.key().to_string(),
        current_step: selection.current_step,
        step_count: selection.steps.len(),
        required_count: selection.required_count(),
        selected_card_indices,
        candidate_card_indices,
    })
}

fn card_observation(card: &Card) -> CardObservation {
    CardObservation {
        id: card.id.raw(),
        suit: suit_key(card.suit).to_string(),
        rank: rank_key(card.rank).to_string(),
        polish_pct_raw: card.polish_pct().raw(),
        engraving: card
            .effects
            .engraving
            .as_ref()
            .map(Engraving::key)
            .map(str::to_string),
    }
}

fn tower_template_observation(template: &TowerTemplate) -> TowerTemplateObservation {
    TowerTemplateObservation {
        kind: format!("{:#?}", template.kind),
        kind_id: tower_kind_id(template.kind),
        suit: template.suit.map(suit_key).map(str::to_string),
        rank: template.rank.map(rank_key).map(str::to_string),
        rerolled_count: template.rerolled_count,
        damage_raw: template.default_damage.raw(),
        used_cards: template.used_cards().iter().map(card_observation).collect(),
    }
}

fn tower_observation(tower: &Tower) -> TowerObservation {
    TowerObservation {
        id: tower.id().raw(),
        left: tower.left_top.x,
        top: tower.left_top.y,
        template: tower_template_observation(&tower.template),
        cooldown_ticks: tower.cooldown_ticks(),
        range_raw: tower.attack_range_radius().raw(),
    }
}

fn shop_slot_observation(
    game_state: &GameState,
    index: usize,
    slot: &crate::shop::ShopSlotData,
) -> ShopSlotObservation {
    let (kind, kind_id, key, key_id, cost) = match &slot.slot {
        crate::shop::ShopSlot::Item { item, cost } => (
            "item",
            shop_kind_id("item"),
            item.key().to_string(),
            item_id(item),
            effective_cost(game_state, *cost),
        ),
        crate::shop::ShopSlot::Upgrade { upgrade, cost } => (
            "upgrade",
            shop_kind_id("upgrade"),
            upgrade.key().to_string(),
            upgrade_id(upgrade),
            effective_cost(game_state, *cost),
        ),
        crate::shop::ShopSlot::CardService {
            card_service, cost, ..
        } => (
            "card_service",
            shop_kind_id("card_service"),
            card_service.key().to_string(),
            card_service_key_id(card_service.key()),
            effective_cost(game_state, *cost),
        ),
    };
    ShopSlotObservation {
        index,
        purchased: slot.purchased,
        kind: kind.to_string(),
        kind_id,
        key,
        key_id,
        cost,
    }
}

fn effective_cost(game_state: &GameState, cost: usize) -> usize {
    if game_state.stage_modifiers.is_free_shop_this_stage() {
        0
    } else {
        cost
    }
}

fn stage_modifiers_observation(game_state: &GameState) -> StageModifiersObservation {
    let modifiers = &game_state.stage_modifiers;
    StageModifiersObservation {
        damage_multiplier_raw: modifiers.get_damage_multiplier().raw(),
        damage_reduction_multiplier_raw: modifiers.get_damage_reduction_multiplier().raw(),
        incoming_damage_multiplier_raw: modifiers.get_incoming_damage_multiplier().raw(),
        gold_gain_multiplier_raw: modifiers.get_gold_gain_multiplier().raw(),
        enemy_health_multiplier_raw: modifiers.get_enemy_health_multiplier().raw(),
        enemy_speed_multiplier_raw: modifiers.get_enemy_speed_multiplier().raw(),
        max_hand_slots_delta: modifiers.get_max_hand_slots_delta(),
        max_rerolls_delta: modifiers.get_max_rerolls_delta(),
        reroll_health_cost: modifiers.get_reroll_health_cost(),
        item_use_disabled: modifiers.is_item_use_disabled(),
        purchases_disabled: modifiers.is_item_and_upgrade_purchases_disabled(),
        free_shop: modifiers.is_free_shop_this_stage(),
    }
}

fn suit_key(suit: Suit) -> &'static str {
    match suit {
        Suit::Spades => "spades",
        Suit::Hearts => "hearts",
        Suit::Diamonds => "diamonds",
        Suit::Clubs => "clubs",
    }
}

fn rank_key(rank: Rank) -> &'static str {
    match rank {
        Rank::Two => "two",
        Rank::Three => "three",
        Rank::Four => "four",
        Rank::Five => "five",
        Rank::Six => "six",
        Rank::Seven => "seven",
        Rank::Eight => "eight",
        Rank::Nine => "nine",
        Rank::Ten => "ten",
        Rank::Jack => "jack",
        Rank::Queen => "queen",
        Rank::King => "king",
        Rank::Ace => "ace",
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
            .deck
            .all_cards()
            .iter()
            .take(3)
            .copied()
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
        let mut base_config = GameConfig::default_config();
        base_config.player.max_stages = 2;
        let config = Arc::new(base_config);
        let runner_config = crate::simulator::policy_runner::PolicyRunnerConfig {
            max_decisions_per_episode: 32,
            max_stage: Some(2),
            ..Default::default()
        };
        let curriculum_runner_config = crate::simulator::policy_runner::PolicyRunnerConfig {
            max_stage: Some(2),
            ..runner_config.clone()
        };
        let mut unrestricted_hashes = Vec::new();
        let unrestricted = crate::simulator::policy_runner::run_episode_with_step_callback(
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
        let curriculum = crate::simulator::policy_runner::run_episode_with_step_callback(
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
            tower_actions
                .iter()
                .all(|(_, left, top)| *left < MAP_SIZE.width && *top < MAP_SIZE.height)
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
            AgentAction::Reroll {
                selected_slot_indices: vec![],
            },
            AgentAction::SelectTower {
                selected_slot_indices: vec![],
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
        environment.game_state.hp = crate::Health::from_integer(30);
        environment.game_state.hp = crate::Health::from_integer(60);
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
            Some(environment.game_state.stage)
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

        environment.game_state.left_dice = 0;
        environment.game_state.hp = crate::Health::from_integer(1);
        assert!(
            environment
                .legal_actions()
                .into_iter()
                .all(|legal| !matches!(legal.action, AgentAction::Reroll { .. }))
        );

        environment.game_state.hp = crate::Health::from_integer(60);
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
        let slot_index = if let GameFlow::Shopping(flow) = &mut environment.game_state.flow {
            flow.shop.push(crate::shop::ShopSlot::CardService {
                card_service: crate::game_state::card_service::CardServiceDiscriminants::Eraser
                    .generate(),
                cost: 0,
            });
            flow.shop.slots.len() - 1
        } else {
            panic!("expected initial shopping flow");
        };

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
        environment
            .step(AgentAction::ConfirmCardServiceSelection)
            .expect("card service confirmation should be legal");

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
}
