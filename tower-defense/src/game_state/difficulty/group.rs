#![allow(dead_code)]

use crate::game_state::effect::Effect;
use crate::game_state::poker_action::{NextStageOffer, PokerAction, roll_call_offer};
use namui::*;
use rand::Rng;

fn stage_factor(stage: usize) -> crate::FixedRatio {
    let progress = ((stage.min(50) as i64 * 1_000_000) / 50).clamp(100_000, 1_000_000);
    crate::FixedRatio::from_raw(500_000 + progress / 2)
}

fn scale(ratio: crate::FixedRatio, factor: crate::FixedRatio) -> crate::FixedRatio {
    crate::FixedRatio::from_raw(
        crate::RatioProduct::one()
            .with(ratio)
            .apply_raw(factor.raw()),
    )
}

fn add(lhs: crate::FixedRatio, rhs: crate::FixedRatio) -> crate::FixedRatio {
    crate::FixedRatio::from_raw(lhs.raw().saturating_add(rhs.raw()))
}

fn random_buff_effect(stage_factor: crate::FixedRatio, rng: &mut impl Rng) -> Effect {
    match rng.gen_range(0..5) {
        0 => Effect::Shield {
            amount: crate::Shield::from_raw(5_000 + rng.gen_range(0..=10_000)),
        },
        1 => Effect::IncreaseAllTowersDamage {
            multiplier: add(
                crate::FixedRatio::ONE,
                scale(
                    crate::FixedRatio::from_raw(50_000 + rng.gen_range(0..=100_000)),
                    stage_factor,
                ),
            ),
        },
        2 => Effect::DecreaseIncomingDamage {
            multiplier: crate::FixedRatio::from_raw(
                crate::FixedRatio::ONE.raw().saturating_sub(
                    scale(
                        crate::FixedRatio::from_raw(100_000 + rng.gen_range(0..=150_000)),
                        stage_factor,
                    )
                    .raw(),
                ),
            ),
        },
        3 => Effect::DecreaseEnemyHealthPercent {
            percentage: scale(
                crate::FixedRatio::from_raw(5_000_000 + rng.gen_range(0..=10_000_000)),
                stage_factor,
            ),
        },
        _ => Effect::DecreaseEnemySpeed {
            multiplier: crate::FixedRatio::from_raw(
                crate::FixedRatio::ONE.raw().saturating_sub(
                    scale(
                        crate::FixedRatio::from_raw(50_000 + rng.gen_range(0..=100_000)),
                        stage_factor,
                    )
                    .raw(),
                ),
            ),
        },
    }
}

fn random_debuff_effect(stage_factor: crate::FixedRatio, rng: &mut impl Rng) -> Effect {
    match rng.gen_range(0..5) {
        0 => Effect::DecreaseAllTowersDamage {
            multiplier: crate::FixedRatio::from_raw(
                crate::FixedRatio::ONE.raw().saturating_sub(
                    scale(
                        crate::FixedRatio::from_raw(50_000 + rng.gen_range(0..=100_000)),
                        stage_factor,
                    )
                    .raw(),
                ),
            ),
        },
        1 => Effect::IncreaseIncomingDamage {
            multiplier: add(
                crate::FixedRatio::ONE,
                scale(
                    crate::FixedRatio::from_raw(100_000 + rng.gen_range(0..=900_000)),
                    stage_factor,
                ),
            ),
        },
        2 => Effect::DisableItemUse,
        3 => Effect::IncreaseEnemyHealthPercent {
            percentage: scale(
                crate::FixedRatio::from_raw(5_000_000 + rng.gen_range(0..=10_000_000)),
                stage_factor,
            ),
        },
        _ => Effect::IncreaseEnemySpeed {
            multiplier: add(
                crate::FixedRatio::ONE,
                scale(
                    crate::FixedRatio::from_raw(50_000 + rng.gen_range(0..=100_000)),
                    stage_factor,
                ),
            ),
        },
    }
}

pub fn action_to_difficulty_option(
    action: PokerAction,
    stage: usize,
    rng: &mut impl Rng,
) -> super::DifficultyOption {
    let stage_factor = stage_factor(stage);
    let mut effects = vec![];
    let next_stage_offer = match action {
        PokerAction::Fold => {
            effects.push(random_buff_effect(stage_factor, rng));
            NextStageOffer::None
        }
        PokerAction::Call => roll_call_offer(rng),
        PokerAction::Raise => {
            effects.push(Effect::IncreaseEnemyHealthPercent {
                percentage: crate::FixedRatio::from_raw(10_000_000 + rng.gen_range(0..=10_000_000)),
            });
            effects.push(random_debuff_effect(stage_factor, rng));
            NextStageOffer::None
        }
        PokerAction::AllIn => {
            effects.push(Effect::IncreaseEnemyHealthPercent {
                percentage: crate::FixedRatio::from_raw(20_000_000 + rng.gen_range(0..=20_000_000)),
            });
            effects.push(random_debuff_effect(stage_factor, rng));
            NextStageOffer::TreasureSelection
        }
    };

    super::DifficultyOption {
        action,
        effects,
        next_stage_offer,
    }
}
