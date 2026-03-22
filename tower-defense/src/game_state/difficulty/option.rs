use crate::game_state::effect::Effect;
use crate::game_state::stage_modifiers::StageModifiers;
use namui::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone, Debug, State)]
pub struct DifficultyOption {
    pub group: super::DifficultyGroup,
    pub name: String,
    pub effects: Vec<Effect>,
}

impl DifficultyOption {
    pub fn apply(&self, modifiers: &mut StageModifiers) {
        for effect in &self.effects {
            effect.apply_to_stage_modifiers(modifiers);
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
            name: "기본 작전".into(),
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
    use crate::game_state::difficulty::DifficultyGroup;
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
                choices.low.name.as_str(),
                "티타임을 즐기기"
                    | "꽃에 물주기"
                    | "상납금을 바치기"
                    | "화해의 선물 주기"
                    | "울면서 봐달라고 빌기"
                    | "물구나무서서 미안하다하기"
                    | "심한 욕하기"
                    | "바보라고 놀리기"
            ),
            "low name wrong"
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
