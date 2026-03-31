use crate::game_state::GameState;
use crate::game_state::effect::{Effect, run_effect};
use crate::game_state::poker_action::{NextStageOffer, PokerAction};
use namui::*;
use rand::thread_rng;

#[derive(Clone, Debug, State)]
pub struct DifficultyOption {
    pub action: PokerAction,
    pub effects: Vec<Effect>,
    pub next_stage_offer: NextStageOffer,
    pub dopamine_delta: i8,
    pub token_delta: i8,
}

impl DifficultyOption {
    pub fn apply(&self, game_state: &mut GameState) {
        game_state.apply_dopamine_delta(self.dopamine_delta);
        if self.token_delta > 0 {
            game_state.add_treasure_token(self.token_delta as u8);
        }

        game_state.pending_next_stage_offer = self.next_stage_offer;

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
            dopamine_delta: PokerAction::Call.dopamine_delta(),
            token_delta: PokerAction::Call.token_delta(),
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

pub fn generate_difficulty_choices(stage: usize) -> DifficultyChoices {
    let mut rng = thread_rng();

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

    DifficultyChoices {
        fold,
        call,
        raise,
        all_in,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::stage_modifiers::StageModifiers;

    #[test]
    fn generate_difficulty_choices_has_fold_call_raise_all_in() {
        let choices = generate_difficulty_choices(10);

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
        let choices = generate_difficulty_choices(10);
        let preselected = choices.call.next_stage_offer;
        assert!(matches!(
            preselected,
            NextStageOffer::None | NextStageOffer::Shop | NextStageOffer::TreasureSelection
        ));

        let mut game_state = crate::game_state::effect::tests_support::make_test_state();
        choices.call.apply(&mut game_state);
        assert_eq!(game_state.pending_next_stage_offer, preselected);
    }

    #[test]
    fn applying_effects_modifies_stage_modifiers() {
        let mut modifiers = StageModifiers::new();
        let effect = Effect::DecreaseEnemyHealthPercent { percentage: 20.0 };
        effect.apply_to_stage_modifiers(&mut modifiers);
        assert!((modifiers.get_enemy_health_multiplier() - 1.2).abs() < 0.0001);

        let effect2 = Effect::DecreaseGoldGainPercent {
            reduction_percentage: 0.10,
        };
        effect2.apply_to_stage_modifiers(&mut modifiers);
        assert!((modifiers.get_gold_gain_multiplier() - 0.9).abs() < 0.0001);
    }

    #[test]
    fn applying_option_runs_effects_on_game_state() {
        let mut game_state = crate::game_state::effect::tests_support::make_test_state();
        game_state.hp = 80.0;
        game_state.gold = 0;

        let option = DifficultyOption {
            action: PokerAction::Call,
            effects: vec![
                Effect::Heal { amount: 10.0 },
                Effect::GainGold {
                    min_amount: 5.0,
                    max_amount: 5.0,
                },
                Effect::IncreaseEnemyHealthPercent { percentage: 20.0 },
            ],
            next_stage_offer: NextStageOffer::None,
            dopamine_delta: PokerAction::Call.dopamine_delta(),
            token_delta: PokerAction::Call.token_delta(),
        };

        option.apply(&mut game_state);

        assert!((game_state.hp - 90.0).abs() < 0.0001);
        assert_eq!(game_state.gold, 5);
        assert!((game_state.stage_modifiers.get_enemy_health_multiplier() - 1.2).abs() < 0.0001);
    }
}
