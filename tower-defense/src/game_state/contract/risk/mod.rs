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
    LoseHealth,
    LoseGold,
    AddChallengeMonsterNextRound,
}

#[derive(Clone, Copy)]
pub(crate) enum WhileActiveEffectKind {
    DecreaseAllTowersDamage,
    IncreaseIncomingDamage,
    DecreaseGoldGain,
    DisableItemAndUpgradePurchases,
    DisableItemUse,
    DecreaseCardSelectionHandMaxSlots,
    DecreaseCardSelectionHandMaxRerolls,
    DecreaseShopMaxRerolls,
    AddCardSelectionHandRerollHealthCost,
    AddShopRerollHealthCost,
    DecreaseEnemyHealth,
    RankTowerDisable,
    SuitTowerDisable,
}

#[derive(Clone, Copy)]
pub(crate) enum OnStageStartEffectKind {
    LoseHealth,
    LoseGold,
}

#[derive(Clone, Copy)]
pub(crate) enum OnExpireEffectKind {
    LoseHealth,
    LoseGold,
}

pub fn generate_risk_effect(effect_type: &ContractEffectType, rarity: Rarity) -> Effect {
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
        OnSignEffectKind::LoseHealth => Effect::LoseHealth {
            amount: rarity_based_random_amount(
                rarity,
                5.0..10.0,
                10.0..15.0,
                15.0..20.0,
                20.0..26.0,
            ),
        },
        OnSignEffectKind::LoseGold => Effect::LoseGold {
            amount: rarity_based_random_amount(
                rarity,
                125.0..151.0,
                250.0..301.0,
                500.0..751.0,
                1000.0..1501.0,
            ) as usize,
        },
        OnSignEffectKind::AddChallengeMonsterNextRound => Effect::AddChallengeMonster,
    }
}

fn effect_from_while_active_kind(kind: WhileActiveEffectKind, rarity: Rarity) -> Effect {
    match kind {
        WhileActiveEffectKind::DecreaseAllTowersDamage => Effect::DamageReduction {
            damage_multiply: rarity_based_amount(rarity, 1.1, 1.15, 1.2, 1.3),
            duration: rarity_based_duration(rarity, 3, 4, 5, 6),
        },
        WhileActiveEffectKind::IncreaseIncomingDamage => Effect::DamageReduction {
            damage_multiply: rarity_based_amount(rarity, 1.1, 1.15, 1.2, 1.3),
            duration: rarity_based_duration(rarity, 3, 4, 5, 6),
        },
        WhileActiveEffectKind::DecreaseGoldGain => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::DisableItemAndUpgradePurchases => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::DisableItemUse => Effect::ExtraReroll,   // placeholder
        WhileActiveEffectKind::DecreaseCardSelectionHandMaxSlots => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::DecreaseCardSelectionHandMaxRerolls => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::DecreaseShopMaxRerolls => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::AddCardSelectionHandRerollHealthCost => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::AddShopRerollHealthCost => Effect::ExtraReroll, // placeholder
        WhileActiveEffectKind::DecreaseEnemyHealth => Effect::ExtraReroll,     // placeholder
        WhileActiveEffectKind::RankTowerDisable => Effect::ExtraReroll,        // placeholder
        WhileActiveEffectKind::SuitTowerDisable => Effect::ExtraReroll,        // placeholder
    }
}

fn effect_from_on_stage_start_kind(kind: OnStageStartEffectKind, rarity: Rarity) -> Effect {
    match kind {
        OnStageStartEffectKind::LoseHealth => Effect::Lottery {
            amount: rarity_based_amount(rarity, 25.0, 50.0, 100.0, 250.0),
            probability: 0.3,
        }, // placeholder
        OnStageStartEffectKind::LoseGold => Effect::Lottery {
            amount: rarity_based_amount(rarity, 25.0, 50.0, 100.0, 250.0),
            probability: 0.3,
        }, // placeholder
    }
}

fn effect_from_on_expire_kind(kind: OnExpireEffectKind, rarity: Rarity) -> Effect {
    match kind {
        OnExpireEffectKind::LoseHealth => Effect::Lottery {
            amount: rarity_based_amount(rarity, 100.0, 200.0, 400.0, 1000.0),
            probability: 0.3,
        }, // placeholder
        OnExpireEffectKind::LoseGold => Effect::Lottery {
            amount: rarity_based_amount(rarity, 100.0, 200.0, 400.0, 1000.0),
            probability: 0.3,
        }, // placeholder
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

fn rarity_based_random_amount(
    rarity: Rarity,
    common: std::ops::Range<f32>,
    rare: std::ops::Range<f32>,
    epic: std::ops::Range<f32>,
    legendary: std::ops::Range<f32>,
) -> f32 {
    use rand::Rng;
    let mut rng = thread_rng();
    match rarity {
        Rarity::Common => rng.gen_range(common),
        Rarity::Rare => rng.gen_range(rare),
        Rarity::Epic => rng.gen_range(epic),
        Rarity::Legendary => rng.gen_range(legendary),
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
