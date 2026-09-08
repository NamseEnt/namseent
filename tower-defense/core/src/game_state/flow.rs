use crate::{ItemEntry, UpgradeEntry};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DefenseEndTransitionState {
    GameOver,
    TreasureSelection,
    StartStage { stage: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefenseEndOutputState {
    pub perfect_clear: bool,
    pub gold: usize,
    pub item_count: usize,
    pub card_count: usize,
    pub transition: DefenseEndTransitionState,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DefenseFlowState {
    pub start_total_hp_raw: i64,
    pub processed_hp_raw: i64,
    pub took_damage: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ShopSlotState {
    Item { item: ItemEntry, cost: usize },
    Upgrade { upgrade: UpgradeEntry, cost: usize },
    CardService { kind: u8, cost: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShopSlotDataState {
    pub id: usize,
    pub slot: ShopSlotState,
    pub purchased: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShopState {
    pub slots: Vec<ShopSlotDataState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShopPurchaseOutput {
    pub slot_id: usize,
    pub slot: ShopSlotState,
    pub cost: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GameFlowState {
    Initializing,
    Shopping(ShopState),
    SelectingTower,
    PlacingTower,
    Defense(DefenseFlowState),
    TreasureSelection {
        options: Vec<UpgradeEntry>,
        pending_selection: Option<usize>,
    },
    Result {
        clear_rate_raw: i64,
    },
}

pub(crate) fn start_selecting_tower(flow: &mut GameFlowState) -> bool {
    if !matches!(flow, GameFlowState::Shopping(_)) {
        return false;
    }
    *flow = GameFlowState::SelectingTower;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selecting_tower_transition_only_accepts_shopping() {
        let mut flow = GameFlowState::Shopping(ShopState { slots: Vec::new() });

        assert!(start_selecting_tower(&mut flow));
        assert_eq!(flow, GameFlowState::SelectingTower);
        assert!(!start_selecting_tower(&mut flow));
    }
}
