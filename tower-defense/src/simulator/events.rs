//! Simulation event types for recording and analysis.

#[cfg(feature = "simulator")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "simulator", derive(Serialize, Deserialize))]
pub enum SimEvent {
    GameStart,
    StageStart { stage: usize },
    ShopReroll { stage: usize },
    ShopPurchase { stage: usize, cost: usize, item_kind: String },
    CardReroll { stage: usize, reroll_number: usize },
    TowerSelected { stage: usize, tower_kind: String, rank: String, suit: String },
    TowerPlaced { stage: usize, tower_kind: String, x: usize, y: usize },
    TowerRemoved { stage: usize, x: usize, y: usize },
    DefenseStart { stage: usize },
    DefenseEnd { stage: usize, victory: bool },
    DamageTaken { stage: usize, amount: f32, remaining_hp: f32 },
    MonsterKilled { stage: usize, monster_kind: String },
    ItemUsed { stage: usize, item_kind: String },
    TreasureSelected { stage: usize, upgrade_kind: String },
    GameEnd { final_stage: usize, victory: bool, clear_rate: f32 },
}
