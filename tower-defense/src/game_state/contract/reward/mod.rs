pub mod on_expire;
pub mod on_sign;
pub mod on_stage_start;
pub mod while_active;

use super::effect_kinds::ContractEffectType;
use crate::{game_state::effect::Effect, rarity::Rarity};
use namui::*;
use rand::prelude::*;

#[derive(Clone, Copy)]
pub(crate) enum OnSignEffectKind {
    HealHealth,
    GainGold,
    GrantUpgrade,
    GrantItem,
}

#[derive(Clone, Copy)]
pub(crate) enum WhileActiveEffectKind {
    IncreaseAllTowersDamage,
    IncreaseAllTowersAttackSpeed,
    IncreaseAllTowersRange,
    DecreaseIncomingDamage,
    IncreaseGoldGain,
    IncreaseCardSelectionHandMaxSlots,
    IncreaseCardSelectionHandMaxRerolls,
    IncreaseShopMaxRerolls,
}

#[derive(Clone, Copy)]
pub(crate) enum OnStageStartEffectKind {
    AddBarricadeCardsToTowerPlacementHand,
    GainShield,
    HealHealth,
    GainGold,
}

#[derive(Clone, Copy)]
pub(crate) enum OnExpireEffectKind {
    HealHealth,
    GainGold,
    GrantUpgrade,
    GrantItem,
}

pub fn generate_reward_effect(effect_type: &ContractEffectType, rarity: Rarity) -> Effect {
    match effect_type {
        ContractEffectType::OnSign => {
            let kinds = on_sign::kinds();
            let kind = kinds.choose(&mut thread_rng()).unwrap();
            effect_from_on_sign_kind(*kind, rarity)
        }
        ContractEffectType::WhileActive => {
            let kinds = while_active::kinds();
            let kind = kinds.choose(&mut thread_rng()).unwrap();
            effect_from_while_active_kind(*kind, rarity)
        }
        ContractEffectType::OnStageStart => {
            let kinds = on_stage_start::kinds();
            let kind = kinds.choose(&mut thread_rng()).unwrap();
            effect_from_on_stage_start_kind(*kind, rarity)
        }
        ContractEffectType::OnExpire => {
            let kinds = on_expire::kinds();
            let kind = kinds.choose(&mut thread_rng()).unwrap();
            effect_from_on_expire_kind(*kind, rarity)
        }
    }
}

fn effect_from_on_sign_kind(kind: OnSignEffectKind, rarity: Rarity) -> Effect {
    match kind {
        OnSignEffectKind::HealHealth => Effect::Heal {
            amount: rarity_based_amount(rarity, 10.0, 20.0, 30.0, 50.0),
        },
        OnSignEffectKind::GainGold => Effect::EarnGold {
            amount: rarity_based_amount(rarity, 100.0, 200.0, 400.0, 1000.0) as usize,
        },
        OnSignEffectKind::GrantUpgrade => Effect::ExtraReroll, // placeholder
        OnSignEffectKind::GrantItem => Effect::ExtraReroll,    // placeholder
    }
}

fn effect_from_while_active_kind(kind: WhileActiveEffectKind, rarity: Rarity) -> Effect {
    match kind {
        WhileActiveEffectKind::IncreaseAllTowersDamage => Effect::UserDamageReduction {
            multiply: rarity_based_amount(rarity, 0.9, 0.85, 0.8, 0.7),
            duration: rarity_based_duration(rarity, 3, 4, 5, 6),
        }, // placeholder, actually increase damage
        WhileActiveEffectKind::IncreaseAllTowersAttackSpeed => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::IncreaseAllTowersRange => Effect::ExtraReroll,       // placeholder
        WhileActiveEffectKind::DecreaseIncomingDamage => Effect::UserDamageReduction {
            multiply: rarity_based_amount(rarity, 0.9, 0.85, 0.8, 0.7),
            duration: rarity_based_duration(rarity, 3, 4, 5, 6),
        },
        WhileActiveEffectKind::IncreaseGoldGain => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::IncreaseCardSelectionHandMaxSlots => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::IncreaseCardSelectionHandMaxRerolls => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::IncreaseShopMaxRerolls => Effect::ExtraReroll, // placeholder
    }
}

fn effect_from_on_stage_start_kind(kind: OnStageStartEffectKind, rarity: Rarity) -> Effect {
    match kind {
        OnStageStartEffectKind::AddBarricadeCardsToTowerPlacementHand => Effect::ExtraReroll, // placeholder
        OnStageStartEffectKind::GainShield => Effect::Shield {
            amount: rarity_based_amount(rarity, 20.0, 40.0, 60.0, 100.0),
        },
        OnStageStartEffectKind::HealHealth => Effect::Heal {
            amount: rarity_based_amount(rarity, 5.0, 10.0, 15.0, 25.0),
        },
        OnStageStartEffectKind::GainGold => Effect::EarnGold {
            amount: rarity_based_amount(rarity, 50.0, 100.0, 200.0, 500.0) as usize,
        },
    }
}

fn effect_from_on_expire_kind(kind: OnExpireEffectKind, rarity: Rarity) -> Effect {
    match kind {
        OnExpireEffectKind::HealHealth => Effect::Heal {
            amount: rarity_based_amount(rarity, 20.0, 40.0, 60.0, 100.0),
        },
        OnExpireEffectKind::GainGold => Effect::EarnGold {
            amount: rarity_based_amount(rarity, 200.0, 400.0, 800.0, 2000.0) as usize,
        },
        OnExpireEffectKind::GrantUpgrade => Effect::ExtraReroll, // placeholder
        OnExpireEffectKind::GrantItem => Effect::ExtraReroll,    // placeholder
    }
}

fn rarity_based_amount(rarity: Rarity, common: f32, rare: f32, epic: f32, legendary: f32) -> f32 {
    match rarity {
        Rarity::Common => common,
        Rarity::Rare => rare,
        Rarity::Epic => epic,
        Rarity::Legendary => legendary,
    }
}

fn rarity_based_duration(
    rarity: Rarity,
    common: i64,
    rare: i64,
    epic: i64,
    legendary: i64,
) -> Duration {
    Duration::from_secs(match rarity {
        Rarity::Common => common,
        Rarity::Rare => rare,
        Rarity::Epic => epic,
        Rarity::Legendary => legendary,
    })
}
