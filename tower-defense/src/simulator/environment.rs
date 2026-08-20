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
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

pub const ENVIRONMENT_VERSION: u32 = 1;
pub const ACTION_SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_MAX_ADVANCE_TICKS: u64 = 60 * 60 * 5;

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

impl AgentAction {
    pub fn action_id(&self) -> String {
        match self {
            Self::PurchaseShopItem { slot_index } => format!("purchase_shop_item:{slot_index}"),
            Self::StartSelectingTower => "start_selecting_tower".to_string(),
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
    pub key: String,
    pub cost: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryObservation {
    pub index: usize,
    pub key: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TowerObservation {
    pub id: u64,
    pub left: usize,
    pub top: usize,
    pub template: TowerTemplateObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonsterObservation {
    pub id: u64,
    pub kind: String,
    pub route_index: usize,
    pub route_progress_raw: i64,
    pub hp_raw: i64,
    pub max_hp_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeckObservation {
    pub cards: Vec<CardObservation>,
    pub draw_count: usize,
    pub discard_count: usize,
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
    pub towers: Vec<TowerObservation>,
    pub tower_grid: Vec<Option<u64>>,
    pub map_width: usize,
    pub map_height: usize,
    pub monsters: Vec<MonsterObservation>,
    pub treasure_options: Vec<String>,
    pub card_service: Option<CardServiceObservation>,
    pub stage_modifiers: StageModifiersObservation,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepInfo {
    pub reason: StepReason,
    pub ticks_advanced: u64,
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
    pub replay_schema_version: u32,
    pub config_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
    pub seed: u64,
    pub commands: Vec<AgentAction>,
    pub checkpoints: Vec<EnvironmentReplayCheckpoint>,
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
    environment_actions: Vec<AgentAction>,
}

impl GameEnvironment {
    pub fn new(config: Arc<GameConfig>, seed: u64) -> Self {
        let mut environment = Self {
            game_state: crate::game_state::create_game_state_with_config(config, seed),
            seed,
            max_advance_ticks: DEFAULT_MAX_ADVANCE_TICKS,
            pending_card_service: None,
            environment_actions: Vec::new(),
        };
        environment.game_state.headless = true;
        environment.game_state.defer_card_service_selection = true;
        environment
    }

    pub fn reset(&mut self, seed: u64, config: Arc<GameConfig>) -> Observation {
        self.game_state = crate::game_state::create_game_state_with_config(config, seed);
        self.game_state.headless = true;
        self.game_state.defer_card_service_selection = true;
        self.seed = seed;
        self.pending_card_service = None;
        self.environment_actions.clear();
        self.snapshot()
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn set_max_advance_ticks(&mut self, max_advance_ticks: u64) {
        self.max_advance_ticks = max_advance_ticks;
    }

    pub fn decision_point(&self) -> DecisionPoint {
        if self.pending_card_service.is_some() {
            return DecisionPoint::CardServiceSelection;
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
        observation_from_game_state(&self.game_state, self.pending_card_service.as_ref())
    }

    pub fn state_hash(&self) -> String {
        self.game_state.authoritative_hash()
    }

    pub fn replay(&self) -> EnvironmentReplay {
        let replay = crate::game_state::replay::Replay::from_game_state(&self.game_state);
        EnvironmentReplay {
            environment_version: ENVIRONMENT_VERSION,
            replay_schema_version: replay.schema_version,
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
            DecisionPoint::Defense => vec![AgentAction::Continue],
            DecisionPoint::TreasureSelection => self.treasure_actions(),
            DecisionPoint::Terminal
            | DecisionPoint::PreDefenseItem
            | DecisionPoint::DamageResponseItem => Vec::new(),
        };

        if !matches!(
            self.decision_point(),
            DecisionPoint::Defense | DecisionPoint::Terminal | DecisionPoint::CardServiceSelection
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
        if let Some(card_service) = deferred_card_service {
            self.pending_card_service = Some(card_service);
        }
        self.environment_actions.push(action);

        let ticks_before = self.game_state.sim_tick().ticks();
        let reason = self.advance_until_decision_or_terminal();
        let ticks_advanced = self
            .game_state
            .sim_tick()
            .ticks()
            .saturating_sub(ticks_before);
        let terminated = matches!(self.decision_point(), DecisionPoint::Terminal);
        let truncated = matches!(reason, StepReason::MaxTicks);
        let terminal_reward = match &self.game_state.flow {
            GameFlow::Result { clear_rate } if *clear_rate == crate::ClearRate::FULL => 1.0,
            GameFlow::Result { .. } => -1.0,
            _ => 0.0,
        };

        Ok(StepOutcome {
            observation: self.snapshot(),
            reward: RewardComponents {
                terminal: terminal_reward,
                shaping: BTreeMap::new(),
            },
            terminated,
            truncated,
            info: StepInfo {
                reason,
                ticks_advanced,
            },
            state_hash: self.state_hash(),
        })
    }

    pub fn advance_until_decision_or_terminal(&mut self) -> StepReason {
        let mut ticks_advanced = 0;
        while matches!(self.game_state.flow, GameFlow::Defense(_)) {
            if ticks_advanced >= self.max_advance_ticks {
                return StepReason::MaxTicks;
            }
            crate::game_state::tick::tick_headless(&mut self.game_state);
            ticks_advanced += 1;
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
        let card_indices = self.card_hand_indices();
        let mut actions = Vec::new();
        if !card_indices.is_empty() {
            for mask in 0..(1usize << card_indices.len()) {
                let selected_slot_indices = card_indices
                    .iter()
                    .enumerate()
                    .filter_map(|(position, slot_index)| {
                        (mask & (1 << position) != 0).then_some(*slot_index)
                    })
                    .collect();
                actions.push(AgentAction::Reroll {
                    selected_slot_indices,
                });
            }
            for mask in 1..(1usize << card_indices.len()) {
                let selected_slot_indices = card_indices
                    .iter()
                    .enumerate()
                    .filter_map(|(position, slot_index)| {
                        (mask & (1 << position) != 0).then_some(*slot_index)
                    })
                    .collect();
                actions.push(AgentAction::SelectTower {
                    selected_slot_indices,
                });
            }
        }
        actions
    }

    fn tower_placement_actions(&self) -> Vec<AgentAction> {
        let mut actions = Vec::new();
        for hand_slot_index in self.tower_hand_indices() {
            for top in 0..MAP_SIZE.height.saturating_sub(1) {
                for left in 0..MAP_SIZE.width.saturating_sub(1) {
                    if can_place_tower_at(&self.game_state, left, top) {
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
                .towers
                .iter()
                .map(|tower| AgentAction::RemoveTower {
                    tower_id: tower.id().raw(),
                }),
        );
        actions.push(AgentAction::StartDefense);
        actions
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
            AgentAction::Continue => Ok(()),
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
) -> Observation {
    let decision_point = if pending_card_service.is_some() {
        DecisionPoint::CardServiceSelection
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
    let selected_slot_ids = game_state.hand.selected_slot_ids();
    let hand = active_slot_ids
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
                selected: selected_slot_ids.contains(slot_id),
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
            route_index: monster.move_on_route.route_index(),
            route_progress_raw: monster.move_on_route.route_progress().raw(),
            hp_raw: monster.hp.raw(),
            max_hp_raw: monster.max_hp.raw(),
        })
        .collect::<Vec<_>>();
    monsters.sort_by_key(|monster| monster.id);

    Observation {
        environment_version: ENVIRONMENT_VERSION,
        action_schema_version: ACTION_SCHEMA_VERSION,
        decision_point,
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
            cards: game_state
                .deck
                .all_cards()
                .iter()
                .map(card_observation)
                .collect(),
            draw_count: game_state.deck.draw_pile().len(),
            discard_count: game_state.deck.discard_pile().len(),
        },
        shop,
        inventory: game_state
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| InventoryObservation {
                index,
                key: item.item.key().to_string(),
            })
            .collect(),
        towers,
        tower_grid,
        map_width: MAP_SIZE.width,
        map_height: MAP_SIZE.height,
        active_monster_count: game_state.monsters.len(),
        monsters,
        treasure_options,
        card_service: pending_card_service
            .and_then(|card_service| card_service_observation(game_state, card_service)),
        stage_modifiers: stage_modifiers_observation(game_state),
    }
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
    }
}

fn shop_slot_observation(
    game_state: &GameState,
    index: usize,
    slot: &crate::shop::ShopSlotData,
) -> ShopSlotObservation {
    let (kind, key, cost) = match &slot.slot {
        crate::shop::ShopSlot::Item { item, cost } => (
            "item",
            item.key().to_string(),
            effective_cost(game_state, *cost),
        ),
        crate::shop::ShopSlot::Upgrade { upgrade, cost } => (
            "upgrade",
            upgrade.key().to_string(),
            effective_cost(game_state, *cost),
        ),
        crate::shop::ShopSlot::CardService {
            card_service, cost, ..
        } => (
            "card_service",
            card_service.key().to_string(),
            effective_cost(game_state, *cost),
        ),
    };
    ShopSlotObservation {
        index,
        purchased: slot.purchased,
        kind: kind.to_string(),
        key,
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
                .any(|action| matches!(action.action, AgentAction::SelectTower { .. }))
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
}
