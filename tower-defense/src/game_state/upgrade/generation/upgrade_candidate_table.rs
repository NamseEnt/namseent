use super::*;
use crate::game_state::GameState;
use crate::game_state::upgrade::UpgradeDiscriminants;
use crate::rarity::Rarity;
use strum::IntoEnumIterator;

pub struct CandidateRow {
    pub weight: f32,
    pub upgrade: Upgrade,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct UpgradeRarityWeights {
    pub common: f32,
    pub rare: f32,
    pub epic: f32,
    pub legendary: f32,
}

impl UpgradeRarityWeights {
    pub(super) fn shop() -> Self {
        Self {
            common: 50.0,
            rare: 25.0,
            epic: 10.0,
            legendary: 5.0,
        }
    }

    pub(super) fn boss_reward() -> Self {
        Self {
            common: 5.0,
            rare: 10.0,
            epic: 25.0,
            legendary: 50.0,
        }
    }
}

impl UpgradeRarityWeights {
    pub fn weight(&self, rarity: Rarity) -> f32 {
        match rarity {
            Rarity::Common => self.common,
            Rarity::Rare => self.rare,
            Rarity::Epic => self.epic,
            Rarity::Legendary => self.legendary,
        }
    }
}

pub(super) fn generate_upgrade_candidate_table(
    game_state: &GameState,
    rarity_weights: UpgradeRarityWeights,
) -> Vec<CandidateRow> {
    let upgrade_state = &game_state.upgrade_state;

    UpgradeDiscriminants::iter()
        .map(|discriminant| {
            let rarity = discriminant.rarity();
            let weight = rarity_weights.weight(rarity);
            let actual_weight =
                if let Some((current, max)) = discriminant.current_and_max(upgrade_state) {
                    if current >= max { 0.0 } else { weight }
                } else {
                    weight
                };

            CandidateRow {
                weight: actual_weight,
                upgrade: discriminant.generate(upgrade_state),
            }
        })
        .collect()
}
