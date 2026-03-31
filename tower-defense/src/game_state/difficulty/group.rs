use crate::game_state::effect::Effect;
use crate::game_state::poker_action::{NextStageOffer, PokerAction, roll_call_offer};
use namui::*;
use rand::Rng;

fn stage_factor(stage: usize) -> f32 {
    0.5 + (stage as f32 / 50.0).clamp(0.1, 1.0) * 0.5
}

fn random_buff_effect(stage_factor: f32, rng: &mut impl Rng) -> Effect {
    match rng.gen_range(0..5) {
        0 => Effect::Shield {
            amount: 5.0 + rng.gen_range(0.0..10.0),
        },
        1 => Effect::IncreaseAllTowersDamage {
            multiplier: 1.0 + (0.05 + rng.gen_range(0.0..0.10)) * stage_factor,
        },
        2 => Effect::DecreaseIncomingDamage {
            multiplier: 1.0 - (0.1 + rng.gen_range(0.0..0.15)) * stage_factor,
        },
        3 => Effect::DecreaseEnemyHealthPercent {
            percentage: (5.0 + rng.gen_range(0.0..10.0)) * stage_factor,
        },
        _ => Effect::DecreaseEnemySpeed {
            multiplier: 1.0 - (0.05 + rng.gen_range(0.0..0.10)) * stage_factor,
        },
    }
}

fn random_debuff_effect(stage_factor: f32, rng: &mut impl Rng) -> Effect {
    match rng.gen_range(0..5) {
        0 => Effect::DecreaseAllTowersDamage {
            multiplier: 1.0 - (0.05 + rng.gen_range(0.0..0.10)) * stage_factor,
        },
        1 => Effect::IncreaseIncomingDamage {
            multiplier: 1.0 + (0.1 + rng.gen_range(0.0..0.9)) * stage_factor,
        },
        2 => Effect::DisableItemUse,
        3 => Effect::IncreaseEnemyHealthPercent {
            percentage: (5.0 + rng.gen_range(0.0..10.0)) * stage_factor,
        },
        _ => Effect::IncreaseEnemySpeed {
            multiplier: 1.0 + (0.05 + rng.gen_range(0.0..0.10)) * stage_factor,
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
                percentage: 10.0 + rng.gen_range(0.0..10.0),
            });
            effects.push(random_debuff_effect(stage_factor, rng));
            NextStageOffer::None
        }
        PokerAction::AllIn => {
            effects.push(Effect::IncreaseEnemyHealthPercent {
                percentage: 20.0 + rng.gen_range(0.0..20.0),
            });
            effects.push(random_debuff_effect(stage_factor, rng));
            NextStageOffer::TreasureSelection
        }
    };

    super::DifficultyOption {
        action,
        effects,
        next_stage_offer,
        dopamine_delta: action.dopamine_delta(),
        token_delta: action.token_delta(),
    }
}
