use crate::{
    Rarity,
    game_state::card_service::{CardService, CardServiceDiscriminants},
};
use strum::IntoEnumIterator;

pub struct CandidateRow {
    pub weight: f32,
    pub card_service: CardService,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct RarityWeights {
    pub common: f32,
    pub rare: f32,
    pub epic: f32,
    pub legendary: f32,
}

impl RarityWeights {
    pub(super) fn shop() -> Self {
        Self {
            common: 50.0,
            rare: 25.0,
            epic: 10.0,
            legendary: 5.0,
        }
    }
}

impl RarityWeights {
    pub fn weight(&self, rarity: Rarity) -> f32 {
        match rarity {
            Rarity::Common => self.common,
            Rarity::Rare => self.rare,
            Rarity::Epic => self.epic,
            Rarity::Legendary => self.legendary,
        }
    }
}

pub(super) fn generate_candidate_table(rarity_weights: RarityWeights) -> Vec<CandidateRow> {
    CardServiceDiscriminants::iter()
        .map(|discriminant| {
            let rarity = discriminant.rarity();
            let weight = rarity_weights.weight(rarity);

            CandidateRow {
                weight,
                card_service: discriminant.generate(),
            }
        })
        .collect()
}
