use crate::game_state::effect::Effect;
use namui::*;
use rand::{Rng, seq::SliceRandom};

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, State)]
pub enum DifficultyGroup {
    StrongTaunt,
    Taunt,
    Normal,
    Peace,
    BigPeace,
}

impl DifficultyGroup {
    pub fn all() -> &'static [DifficultyGroup] {
        &[
            DifficultyGroup::StrongTaunt,
            DifficultyGroup::Taunt,
            DifficultyGroup::Normal,
            DifficultyGroup::Peace,
            DifficultyGroup::BigPeace,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            DifficultyGroup::StrongTaunt => "강한 도발",
            DifficultyGroup::Taunt => "도발",
            DifficultyGroup::Normal => "일반",
            DifficultyGroup::Peace => "평화",
            DifficultyGroup::BigPeace => "큰 평화",
        }
    }

    pub fn difficulty_rank(&self) -> u8 {
        match self {
            DifficultyGroup::BigPeace => 0,
            DifficultyGroup::Peace => 1,
            DifficultyGroup::Normal => 2,
            DifficultyGroup::Taunt => 3,
            DifficultyGroup::StrongTaunt => 4,
        }
    }

    pub fn flavor_names(&self) -> &'static [&'static str] {
        match self {
            DifficultyGroup::StrongTaunt => &["심한 욕하기"],
            DifficultyGroup::Taunt => &["바보라고 놀리기"],
            DifficultyGroup::Normal => &["티타임을 즐기기", "꽃에 물주기"],
            DifficultyGroup::Peace => &["상납금을 바치기", "화해의 선물 주기"],
            DifficultyGroup::BigPeace => &["울면서 봐달라고 빌기", "물구나무서서 미안하다하기"],
        }
    }

    fn difficulty_increase(&self, stage_factor: f32, rng: &mut impl Rng) -> Effect {
        match rng.gen_range(0..6) {
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
            4 => Effect::IncreaseEnemySpeed {
                multiplier: 1.0 + (0.05 + rng.gen_range(0.0..0.10)) * stage_factor,
            },
            _ => {
                let ranks = [
                    crate::card::Rank::Seven,
                    crate::card::Rank::Eight,
                    crate::card::Rank::Nine,
                    crate::card::Rank::Ten,
                    crate::card::Rank::Jack,
                    crate::card::Rank::Queen,
                    crate::card::Rank::King,
                    crate::card::Rank::Ace,
                ];
                let rank = *ranks.choose(rng).unwrap();
                Effect::RankTowerDisable { rank }
            }
        }
    }

    fn big_difficulty_increase(&self, stage_factor: f32, rng: &mut impl Rng) -> Effect {
        match rng.gen_range(0..5) {
            0 => Effect::DecreaseAllTowersDamage {
                multiplier: 1.0 - (0.15 + rng.gen_range(0.0..0.15)) * stage_factor,
            },
            1 => Effect::IncreaseIncomingDamage {
                multiplier: 1.0 + (1.0 + rng.gen_range(0.0..1.0)) * stage_factor,
            },
            2 => Effect::IncreaseEnemyHealthPercent {
                percentage: (15.0 + rng.gen_range(0.0..15.0)) * stage_factor,
            },
            3 => Effect::IncreaseEnemySpeed {
                multiplier: 1.0 + (0.15 + rng.gen_range(0.0..0.15)) * stage_factor,
            },
            _ => {
                let suits = [
                    crate::card::Suit::Hearts,
                    crate::card::Suit::Diamonds,
                    crate::card::Suit::Clubs,
                    crate::card::Suit::Spades,
                ];
                let suit = *suits.choose(rng).unwrap();
                Effect::SuitTowerDisable { suit }
            }
        }
    }

    fn difficulty_decrease(&self, stage_factor: f32, rng: &mut impl Rng) -> Effect {
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

    fn reward_increase(&self, stage_factor: f32, rng: &mut impl Rng, high: bool) -> Effect {
        let select_rarity = |high: bool, roll: f32| {
            if high {
                if roll < 0.5 {
                    crate::rarity::Rarity::Common
                } else if roll < 0.75 {
                    crate::rarity::Rarity::Rare
                } else if roll < 0.95 {
                    crate::rarity::Rarity::Epic
                } else {
                    crate::rarity::Rarity::Legendary
                }
            } else if roll < 0.75 {
                crate::rarity::Rarity::Common
            } else if roll < 0.875 {
                crate::rarity::Rarity::Rare
            } else if roll < 0.998 {
                crate::rarity::Rarity::Epic
            } else {
                crate::rarity::Rarity::Legendary
            }
        };

        match rng.gen_range(0..4) {
            0 => {
                let amount = if high {
                    (7.0 + rng.gen_range(0.0..5.0)) * stage_factor
                } else {
                    (2.0 + rng.gen_range(0.0..5.0)) * stage_factor
                };
                Effect::Heal { amount }
            }
            1 => {
                let rarity = select_rarity(high, rng.gen_range(0.0..1.0));
                Effect::GrantUpgrade { rarity }
            }
            2 => {
                let rarity = select_rarity(high, rng.gen_range(0.0..1.0));
                Effect::GrantItem { rarity }
            }
            _ => {
                let multiplier = if high {
                    1.0 + (0.25 + rng.gen_range(0.0..0.25)) * stage_factor
                } else {
                    1.0 + (0.05 + rng.gen_range(0.0..0.20)) * stage_factor
                };
                Effect::IncreaseGoldGain { multiplier }
            }
        }
    }

    pub(crate) fn reward_decrease(
        &self,
        stage_factor: f32,
        rng: &mut impl Rng,
        high: bool,
    ) -> Effect {
        let reduction_percentage = if high {
            rng.gen_range(0.35..0.75)
        } else {
            rng.gen_range(0.05..0.35)
        };
        Effect::DecreaseGoldGainPercent {
            reduction_percentage: reduction_percentage * stage_factor,
        }
    }

    pub fn to_difficulty_option(self, stage: usize, rng: &mut impl Rng) -> super::DifficultyOption {
        let stage_factor = 0.5 + (stage as f32 / 50.0).clamp(0.1, 1.0) * 0.5;
        let mut effects = vec![];

        match self {
            DifficultyGroup::StrongTaunt => {
                effects.push(self.big_difficulty_increase(stage_factor, rng));
                effects.push(self.difficulty_increase(stage_factor, rng));
                effects.push(self.reward_increase(stage_factor, rng, true));
            }
            DifficultyGroup::Taunt => {
                effects.push(self.difficulty_increase(stage_factor, rng));
                effects.push(self.reward_increase(stage_factor, rng, false));
            }
            DifficultyGroup::Normal => {
                // no gameplay modifiers
            }
            DifficultyGroup::Peace => {
                effects.push(self.difficulty_decrease(stage_factor, rng));
                effects.push(self.reward_decrease(stage_factor, rng, false));
            }
            DifficultyGroup::BigPeace => {
                effects.push(self.difficulty_decrease(stage_factor, rng));
                effects.push(self.difficulty_decrease(stage_factor, rng));
                effects.push(self.reward_decrease(stage_factor, rng, true));
            }
        }

        let name = {
            let flavor_options = self.flavor_names();
            flavor_options.choose(rng).unwrap().to_string()
        };

        super::DifficultyOption {
            group: self,
            name,
            effects,
        }
    }
}
