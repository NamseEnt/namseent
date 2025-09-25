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
    DecreaseAllTowersDamagePercentDuringContract,
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
    GainGoldEachStageDuringContract,
}

#[derive(Clone, Copy)]
pub(crate) enum OnExpireEffectKind {
    HealHealthOnContractEnd,
    GainGoldOnContractEnd,
    GrantUpgradeOnContractEnd,
    GrantItem,
}

pub fn generate_reward_effect(
    effect_type: &ContractEffectType,
    rarity: Rarity,
    duration_stages: usize,
) -> Effect {
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
            effect_from_on_stage_start_kind(*kind, rarity, duration_stages)
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
            amount: rarity_based_random_amount(
                rarity,
                10.0..15.0,
                20.0..25.0,
                30.0..35.0,
                40.0..46.0,
            ),
        },
        OnSignEffectKind::GainGold => Effect::EarnGold {
            amount: rarity_based_random_amount(
                rarity,
                225.0..251.0,
                500.0..551.0,
                1000.0..1251.0,
                2000.0..2501.0,
            ) as usize,
        },
        OnSignEffectKind::GrantUpgrade => Effect::GrantUpgrade { rarity },
        OnSignEffectKind::GrantItem => Effect::GrantItem { rarity },
    }
}

fn effect_from_while_active_kind(kind: WhileActiveEffectKind, rarity: Rarity) -> Effect {
    match kind {
        WhileActiveEffectKind::IncreaseAllTowersDamage => Effect::IncreaseAllTowersDamage {
            multiplier: rarity_based_random_amount(
                rarity,
                1.01..1.06, // 1% ~ 5%
                1.05..1.11, // 5% ~ 10%
                1.10..1.26, // 10% ~ 25%
                1.25..1.76, // 25% ~ 75%
            ),
        },
        WhileActiveEffectKind::DecreaseAllTowersDamagePercentDuringContract => {
            Effect::DecreaseAllTowersDamage {
                multiplier: rarity_based_random_amount(
                    rarity,
                    0.95..0.99, // 1% ~ 5% decrease
                    0.90..0.95, // 5% ~ 10% decrease
                    0.75..0.90, // 10% ~ 25% decrease
                    0.25..0.75, // 25% ~ 75% decrease
                ),
            }
        }
        WhileActiveEffectKind::IncreaseAllTowersAttackSpeed => {
            Effect::IncreaseAllTowersAttackSpeed {
                multiplier: rarity_based_random_amount(
                    rarity,
                    1.01..1.06, // 1% ~ 5%
                    1.05..1.11, // 5% ~ 10%
                    1.10..1.26, // 10% ~ 25%
                    1.25..1.76, // 25% ~ 75%
                ),
            }
        }
        WhileActiveEffectKind::IncreaseAllTowersRange => Effect::IncreaseAllTowersRange {
            multiplier: rarity_based_random_amount(
                rarity,
                1.01..1.06, // 1% ~ 5%
                1.05..1.11, // 5% ~ 10%
                1.10..1.26, // 10% ~ 25%
                1.25..1.51, // 25% ~ 50%
            ),
        },
        WhileActiveEffectKind::DecreaseIncomingDamage => Effect::DecreaseIncomingDamage {
            multiplier: rarity_based_random_amount(
                rarity,
                0.9..0.95, // 5% ~ 10% reduction
                0.8..0.9,  // 10% ~ 20% reduction
                0.65..0.8, // 20% ~ 35% reduction
                0.5..0.65, // 35% ~ 50% reduction
            ),
        },
        WhileActiveEffectKind::IncreaseGoldGain => Effect::IncreaseGoldGain {
            multiplier: rarity_based_random_amount(
                rarity,
                1.25..1.35, // 25% ~ 35%
                1.35..1.5,  // 35% ~ 50%
                1.5..1.75,  // 50% ~ 75%
                1.75..2.25, // 75% ~ 125%
            ),
        },
        WhileActiveEffectKind::IncreaseCardSelectionHandMaxSlots => {
            Effect::IncreaseCardSelectionHandMaxSlots { bonus: 1 }
        }
        WhileActiveEffectKind::IncreaseCardSelectionHandMaxRerolls => {
            Effect::IncreaseCardSelectionHandMaxRerolls { bonus: 1 }
        }
        WhileActiveEffectKind::IncreaseShopMaxRerolls => {
            Effect::IncreaseShopMaxRerolls { bonus: 1 }
        }
    }
}

fn effect_from_on_stage_start_kind(
    kind: OnStageStartEffectKind,
    rarity: Rarity,
    duration_stages: usize,
) -> Effect {
    match kind {
        OnStageStartEffectKind::AddBarricadeCardsToTowerPlacementHand => {
            Effect::AddBarricadeCardsToTowerPlacementHandEachStageDuringContract {
                count: rarity_based_amount(rarity, 1.0, 2.0, 3.0, 4.0) as usize,
            }
        }
        OnStageStartEffectKind::GainShield => Effect::GainShieldEachStageDuringContract {
            min_amount: match rarity {
                Rarity::Common => 1.0,
                Rarity::Rare => 2.0,
                Rarity::Epic => 4.0,
                Rarity::Legendary => 7.0,
            },
            max_amount: match rarity {
                Rarity::Common => 2.0,
                Rarity::Rare => 3.0,
                Rarity::Epic => 6.0,
                Rarity::Legendary => 10.0,
            },
        },
        OnStageStartEffectKind::HealHealth => Effect::HealHealthEachStageDuringContract {
            min_amount: match rarity {
                Rarity::Common => 10.0,
                Rarity::Rare => 20.0,
                Rarity::Epic => 30.0,
                Rarity::Legendary => 40.0,
            },
            max_amount: match rarity {
                Rarity::Common => 14.0,
                Rarity::Rare => 24.0,
                Rarity::Epic => 34.0,
                Rarity::Legendary => 45.0,
            },
        },
        OnStageStartEffectKind::GainGoldEachStageDuringContract => {
            let total_gold = rarity_based_random_amount(
                rarity,
                225.0..251.0,
                500.0..551.0,
                1000.0..1251.0,
                2000.0..2501.0,
            );
            let base_amount = (total_gold / duration_stages as f32).max(1.0);
            let min_amount = (base_amount * 0.8).floor();
            let max_amount = (base_amount * 1.2).ceil();
            Effect::GainGoldEachStageDuringContract {
                min_amount,
                max_amount,
            }
        }
    }
}

fn effect_from_on_expire_kind(kind: OnExpireEffectKind, rarity: Rarity) -> Effect {
    match kind {
        OnExpireEffectKind::HealHealthOnContractEnd => Effect::Heal {
            amount: rarity_based_random_amount(
                rarity,
                10.0..15.0, // Common: 10~14
                20.0..25.0, // Rare: 20~24
                30.0..35.0, // Epic: 30~34
                40.0..46.0, // Legendary: 40~45
            ),
        },
        OnExpireEffectKind::GainGoldOnContractEnd => Effect::EarnGold {
            amount: rarity_based_random_amount(
                rarity,
                225.0..251.0,   // Common: 225~250
                500.0..551.0,   // Rare: 500~550
                1000.0..1251.0, // Epic: 1000~1250
                2000.0..2501.0, // Legendary: 2000~2500
            ) as usize,
        },
        OnExpireEffectKind::GrantUpgradeOnContractEnd => Effect::GrantUpgrade { rarity },
        OnExpireEffectKind::GrantItem => Effect::ExtraReroll, // placeholder
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
