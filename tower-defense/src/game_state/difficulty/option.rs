use crate::game_state::GameState;
use crate::game_state::effect::{Effect, run_effect};
use namui::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone, Debug, State)]
pub struct DifficultyOption {
    pub group: super::DifficultyGroup,
    pub operation: super::OperationKind,
    pub effects: Vec<Effect>,
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
            group: super::DifficultyGroup::Normal,
            operation: super::OperationKind::TeaTime,
            effects: vec![],
        }
    }
}

#[derive(Clone, Debug, State, Default)]
pub struct DifficultyChoices {
    pub low: DifficultyOption,
    pub high: DifficultyOption,
}

pub fn generate_difficulty_choices(stage: usize) -> DifficultyChoices {
    let mut rng = thread_rng();
    let mut groups = super::DifficultyGroup::all().to_vec();
    groups.shuffle(&mut rng);

    let mut options = Vec::with_capacity(2);
    for group in groups.into_iter().take(2) {
        options.push(group.to_difficulty_option(stage, &mut rng));
    }

    options.sort_by(|a, b| a.group.difficulty_rank().cmp(&b.group.difficulty_rank()));

    DifficultyChoices {
        low: options.remove(0),
        high: options.remove(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{
        difficulty::{DifficultyGroup, OperationKind},
        stage_modifiers::StageModifiers,
    };
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generate_difficulty_choices_returns_sorted_low_high() {
        let mut rng = StdRng::seed_from_u64(123);
        let choices = {
            let mut groups = DifficultyGroup::all().to_vec();
            groups.shuffle(&mut rng);
            let mut opts = vec![];
            for group in groups.into_iter().take(2) {
                opts.push(group.to_difficulty_option(10, &mut rng));
            }
            opts.sort_by(|a, b| a.group.difficulty_rank().cmp(&b.group.difficulty_rank()));
            DifficultyChoices {
                low: opts.remove(0),
                high: opts.remove(0),
            }
        };

        assert!(choices.low.group.difficulty_rank() <= choices.high.group.difficulty_rank());
        assert!(
            matches!(
                choices.low.operation,
                OperationKind::StrongTaunt
                    | OperationKind::Taunt
                    | OperationKind::TeaTime
                    | OperationKind::FlowerWatering
                    | OperationKind::Tribute
                    | OperationKind::PeaceGift
                    | OperationKind::Plead
                    | OperationKind::HandstandApology
            ),
            "low operation wrong"
        );

        assert!(choices.low.effects.len() <= 3);
        assert!(choices.high.effects.len() <= 3);
    }

    #[test]
    fn group_effect_counts_by_group() {
        let mut rng = StdRng::seed_from_u64(333);

        let strong = DifficultyGroup::StrongTaunt.to_difficulty_option(10, &mut rng);
        assert_eq!(strong.effects.len(), 3);

        let taunt = DifficultyGroup::Taunt.to_difficulty_option(10, &mut rng);
        assert_eq!(taunt.effects.len(), 2);

        let normal = DifficultyGroup::Normal.to_difficulty_option(10, &mut rng);
        assert!(normal.effects.is_empty());

        let peace = DifficultyGroup::Peace.to_difficulty_option(10, &mut rng);
        assert_eq!(peace.effects.len(), 2);

        let big_peace = DifficultyGroup::BigPeace.to_difficulty_option(10, &mut rng);
        assert_eq!(big_peace.effects.len(), 3);
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
            group: DifficultyGroup::Normal,
            operation: OperationKind::TeaTime,
            effects: vec![
                Effect::Heal { amount: 10.0 },
                Effect::GainGold {
                    min_amount: 5.0,
                    max_amount: 5.0,
                },
                Effect::IncreaseEnemyHealthPercent { percentage: 20.0 },
            ],
        };

        option.apply(&mut game_state);

        assert!((game_state.hp - 90.0).abs() < 0.0001);
        assert_eq!(game_state.gold, 5);
        assert!((game_state.stage_modifiers.get_enemy_health_multiplier() - 1.2).abs() < 0.0001);
    }

    #[test]
    fn reward_decrease_ranges_for_low_and_high() {
        let mut rng = StdRng::seed_from_u64(12345);

        for _ in 0..100 {
            let low_effect = DifficultyGroup::Peace.reward_decrease(1.0, &mut rng, false);
            if let Effect::DecreaseGoldGainPercent {
                reduction_percentage,
            } = low_effect
            {
                assert!((0.05..=0.35).contains(&reduction_percentage));
            } else {
                panic!("expected DecreaseGoldGainPercent from low reward_decrease");
            }

            let high_effect = DifficultyGroup::BigPeace.reward_decrease(1.0, &mut rng, true);
            if let Effect::DecreaseGoldGainPercent {
                reduction_percentage,
            } = high_effect
            {
                assert!((0.35..=0.75).contains(&reduction_percentage));
            } else {
                panic!("expected DecreaseGoldGainPercent from high reward_decrease");
            }
        }
    }
}
