use super::super::{Effect, EffectExecutionError};
use crate::game_state::{
    effect::tests_support::make_test_state,
    flow::{DefenseFlow, GameFlow},
};
use crate::{
    card::{Rank, Suit},
    rarity::Rarity,
};

#[test]
fn test_extra_reroll_in_selecting_tower_flow() {
    let mut state = make_test_state();
    state.flow = GameFlow::SelectingTower(crate::game_state::flow::SelectingTowerFlow::new(&state));

    let effect = Effect::ExtraReroll;
    let result = effect.can_execute(&state);

    assert!(result.is_ok());
}

#[test]
fn test_extra_reroll_in_wrong_flow() {
    let mut state = make_test_state();
    state.flow = GameFlow::Defense(DefenseFlow::new(&state));

    let effect = Effect::ExtraReroll;
    let result = effect.can_execute(&state);

    assert_eq!(
        result,
        Err(EffectExecutionError::InvalidFlow {
            required: "SelectingTower".to_string()
        })
    );
}

#[test]
fn test_item_use_disabled_by_stage_modifier() {
    let mut state = make_test_state();
    state.stage_modifiers.disable_item_use();

    let effect = Effect::EarnGold { amount: 10 };
    let result = effect.can_execute(&state);

    assert_eq!(result, Err(EffectExecutionError::ItemUseDisabled));
}

#[test]
fn test_all_other_effects_always_allowed() {
    let state = make_test_state();

    let effects = vec![
        Effect::Heal { amount: 10.0 },
        Effect::Shield { amount: 10.0 },
        Effect::EarnGold { amount: 10 },
        Effect::LoseGold { amount: 10 },
        Effect::LoseHealth { amount: 10.0 },
        Effect::GrantUpgrade {
            rarity: Rarity::Common,
        },
        Effect::GrantItem {
            rarity: Rarity::Common,
        },
        Effect::IncreaseAllTowersDamage { multiplier: 1.5 },
        Effect::DecreaseAllTowersDamage { multiplier: 0.8 },
        Effect::IncreaseIncomingDamage { multiplier: 1.2 },
        Effect::IncreaseAllTowersAttackSpeed { multiplier: 1.3 },
        Effect::IncreaseAllTowersRange { multiplier: 1.1 },
        Effect::DecreaseIncomingDamage { multiplier: 0.9 },
        Effect::IncreaseGoldGain { multiplier: 1.5 },
        Effect::RankTowerDisable { rank: Rank::Ace },
        Effect::SuitTowerDisable { suit: Suit::Spades },
        Effect::HealHealth {
            min_amount: 5.0,
            max_amount: 10.0,
        },
        Effect::GainShield {
            min_amount: 5.0,
            max_amount: 10.0,
        },
        Effect::GainGold {
            min_amount: 5.0,
            max_amount: 10.0,
        },
        Effect::LoseHealthRange {
            min_amount: 5.0,
            max_amount: 10.0,
        },
        Effect::LoseGoldRange {
            min_amount: 5.0,
            max_amount: 10.0,
        },
    ];

    for effect in effects {
        let result = effect.can_execute(&state);
        assert!(result.is_ok(), "Effect {:?} should be allowed", effect);
    }
}
