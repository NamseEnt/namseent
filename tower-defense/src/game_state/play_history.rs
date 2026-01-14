use super::*;
use crate::{
    card::{Rank, Suit},
    game_state::{
        contract::Contract,
        item::{Effect, Item},
        upgrade::Upgrade,
    },
    *,
};

#[derive(Debug, Clone, Default, State)]
pub struct PlayHistory {
    pub events: Vec<HistoryEvent>,
}

#[derive(Debug, Clone, State)]
pub struct HistoryEvent {
    pub stage: usize,
    pub timestamp: Instant,
    pub event_type: HistoryEventType,
}

#[derive(Debug, Clone, State)]
pub enum HistoryEventType {
    GameStart,

    StageStart {
        stage: usize,
    },

    TowerPlaced {
        tower_kind: TowerKind,
        rank: Rank,
        suit: Suit,
        left_top: MapCoord,
    },

    DamageTaken {
        amount: f32,
    },

    ItemPurchased {
        item: Item,
        cost: usize,
    },

    ItemUsed {
        item_effect: Effect,
    },

    UpgradeSelected {
        upgrade: Upgrade,
    },

    UpgradePurchased {
        upgrade: Upgrade,
        cost: usize,
    },

    ContractPurchased {
        contract: Contract,
        cost: usize,
    },

    GameOver,
}

impl PlayHistory {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
}

impl GameState {
    pub fn record_event(&mut self, event_type: HistoryEventType) {
        self.play_history.events.push(HistoryEvent {
            stage: self.stage,
            timestamp: self.now(),
            event_type,
        });
    }
}
