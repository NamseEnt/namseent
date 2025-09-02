use crate::rarity::Rarity;
use namui::*;

#[derive(Clone, Debug)]
pub enum ContractEffect {
    OnSign { effect: Effect },
    WhileActive { effect: Effect },
    OnStageStart { effect: Effect },
    OnExpire { effect: Effect },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContractStatus {
    Pending {
        starts_at_stage: usize,
        duration_stages: usize,
    },
    Active {
        ends_at_stage: usize,
    },
    Expired,
}

#[derive(Clone, Debug)]
pub struct Contract {
    pub id: u64,
    pub rarity: Rarity,
    pub duration_stages: usize,
    pub status: ContractStatus,
    pub risk: ContractEffect,
    pub reward: ContractEffect,
}

#[derive(Clone, Debug)]
pub enum Effect {
    UserDamageReduction { multiply: f32, duration: Duration },
    Heal { amount: f32 },
    Shield { amount: f32 },
    EarnGold { amount: usize },
    ExtraReroll,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ContractState {
    // TODO: like upgrade state
}
