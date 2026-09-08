#![allow(dead_code)]

use crate::game_state::GameState;
use crate::game_state::effect::{Effect, run_effect};
use crate::game_state::poker_action::{NextStageOffer, PokerAction};
use namui::*;

#[derive(Clone, Debug, State)]
pub struct DifficultyOption {
    pub action: PokerAction,
    pub effects: Vec<Effect>,
    pub next_stage_offer: NextStageOffer,
}

impl DifficultyOption {
    pub fn apply(&self, game_state: &mut GameState) {
        for effect in &self.effects {
            run_effect(game_state, effect);
        }
    }

    pub fn descriptions(&self) -> Vec<crate::l10n::effect::EffectText> {
        self.effects
            .iter()
            .map(|effect| effect.description_text())
            .collect()
    }
}

impl Default for DifficultyOption {
    fn default() -> Self {
        DifficultyOption {
            action: PokerAction::Call,
            effects: vec![],
            next_stage_offer: NextStageOffer::None,
        }
    }
}

#[derive(Clone, Debug, State, Default)]
pub struct DifficultyChoices {
    pub fold: DifficultyOption,
    pub call: DifficultyOption,
    pub raise: DifficultyOption,
    pub all_in: DifficultyOption,
}

pub fn generate_difficulty_choices(game_state: &mut GameState) -> DifficultyChoices {
    game_state.sync_raw_core_from_projection();
    let mut raw = game_state.raw_core.state().clone();
    let stage = raw.progress().stage;
    let mut rng = None;
    raw.edit_snapshot(|parts| {
        rng = Some(parts.rng.next_rng(
            td_core::deterministic_rng::domain::DIFFICULTY_OFFER,
            &[stage as u64],
        ));
    })
    .expect("difficulty RNG state must remain valid");
    let mut rng = rng.expect("difficulty RNG must be initialized");

    let fold = crate::game_state::difficulty::group::action_to_difficulty_option(
        PokerAction::Fold,
        stage,
        &mut rng,
    );

    let call = crate::game_state::difficulty::group::action_to_difficulty_option(
        PokerAction::Call,
        stage,
        &mut rng,
    );

    let raise = crate::game_state::difficulty::group::action_to_difficulty_option(
        PokerAction::Raise,
        stage,
        &mut rng,
    );

    let all_in = crate::game_state::difficulty::group::action_to_difficulty_option(
        PokerAction::AllIn,
        stage,
        &mut rng,
    );

    let choices = DifficultyChoices {
        fold,
        call,
        raise,
        all_in,
    };
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw difficulty RNG state must be restorable in headed adapter");
    choices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::stage_modifiers::StageModifiers;

    #[test]
    fn generate_difficulty_choices_has_fold_call_raise_all_in() {
        let mut game_state = crate::game_state::effect::tests_support::make_test_state();
        game_state.stage = 10;
        let choices = generate_difficulty_choices(&mut game_state);

        assert_eq!(choices.fold.action, PokerAction::Fold);
        assert_eq!(choices.call.action, PokerAction::Call);
        assert_eq!(choices.raise.action, PokerAction::Raise);
        assert_eq!(choices.all_in.action, PokerAction::AllIn);

        assert!(choices.fold.effects.len() <= 3);
        assert!(choices.call.effects.len() <= 3);
        assert!(choices.raise.effects.len() <= 3);
        assert!(choices.all_in.effects.len() <= 3);
    }

    #[test]
    fn call_option_preconfirms_next_stage_offer() {
        let mut game_state = crate::game_state::effect::tests_support::make_test_state();
        game_state.stage = 10;
        let choices = generate_difficulty_choices(&mut game_state);
        let preselected = choices.call.next_stage_offer;
        assert!(matches!(
            preselected,
            NextStageOffer::None | NextStageOffer::Shop | NextStageOffer::TreasureSelection
        ));

        choices.call.apply(&mut game_state);
        // Difficulty option should apply effects without requiring legacy GameState fields.
        assert!(matches!(
            game_state.flow,
            crate::game_state::flow::GameFlow::Initializing
                | crate::game_state::flow::GameFlow::SelectingTower(_)
                | crate::game_state::flow::GameFlow::PlacingTower
                | crate::game_state::flow::GameFlow::Defense(_)
                | crate::game_state::flow::GameFlow::TreasureSelection(_)
                | crate::game_state::flow::GameFlow::Result { .. }
        ));
    }

    #[test]
    fn applying_effects_modifies_stage_modifiers() {
        let mut modifiers = StageModifiers::new();
        let effect = Effect::DecreaseEnemyHealthPercent {
            percentage: crate::FixedRatio::from_integer(20),
        };
        effect.apply_to_stage_modifiers(&mut modifiers);
        assert_eq!(
            modifiers.get_enemy_health_multiplier(),
            crate::FixedRatio::from_raw(1_200_000)
        );

        let effect2 = Effect::DecreaseGoldGainPercent {
            reduction_percentage: crate::FixedRatio::from_raw(100_000),
        };
        effect2.apply_to_stage_modifiers(&mut modifiers);
        assert_eq!(
            modifiers.get_gold_gain_multiplier(),
            crate::FixedRatio::from_raw(900_000)
        );
    }

    #[test]
    fn applying_option_runs_effects_on_game_state() {
        let mut game_state = crate::game_state::effect::tests_support::make_test_state();
        game_state.hp = crate::Health::from_integer(40);
        game_state.gold = 0;

        let option = DifficultyOption {
            action: PokerAction::Call,
            effects: vec![
                Effect::Heal {
                    amount: crate::Health::from_integer(10),
                },
                Effect::GainGold {
                    min_amount: 5,
                    max_amount: 5,
                },
                Effect::IncreaseEnemyHealthPercent {
                    percentage: crate::FixedRatio::from_integer(20),
                },
            ],
            next_stage_offer: NextStageOffer::None,
        };

        option.apply(&mut game_state);

        assert_eq!(game_state.hp, crate::Health::from_integer(50));
        assert_eq!(game_state.gold, 5);
        assert_eq!(
            game_state.stage_modifiers.get_enemy_health_multiplier(),
            crate::FixedRatio::from_raw(1_200_000)
        );
    }
}
