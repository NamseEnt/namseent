use crate::CoreEvent;

/// Version of the stable action kind wire names used by policy datasets.
pub const ACTION_WIRE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommandOutput {
    Accepted { events: Vec<CoreEvent> },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum PlayerCommand {
    Reroll {
        selected_slot_indices: Vec<usize>,
    },
    PurchaseShopItem {
        slot_index: usize,
    },
    UseInventoryItem {
        item_index: usize,
    },
    StartSelectingTower,
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
    ConfirmCardServiceSelection {
        selected_card_ids: Vec<Vec<u64>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecordedPlayerCommand {
    pub sequence: u64,
    pub completed_sim_tick: u64,
    pub command: PlayerCommand,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommandReceipt {
    pub sequence: u64,
    pub completed_sim_tick: u64,
    pub command: PlayerCommand,
    pub state_hash: String,
    pub events: Vec<CoreEvent>,
    #[serde(default)]
    pub event_count: u64,
    #[serde(default)]
    pub event_digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommandError {
    InvalidFlow,
    InvalidIndex,
    InvalidItemKind { raw: u8 },
    InvalidSelection,
    InvalidPlacement,
    UnknownTower,
    Rejected,
    InvalidCardServiceKind { raw: u8 },
    InvalidUpgradeKind { raw: u8 },
}
