use crate::{Rarity, game_state::item::ItemDiscriminants};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug)]
pub(super) struct ItemRarityWeights {
    pub common: f32,
    pub rare: f32,
    pub epic: f32,
    pub legendary: f32,
}

impl ItemRarityWeights {
    pub(super) fn shop() -> Self {
        Self {
            common: 50.0,
            rare: 25.0,
            epic: 10.0,
            legendary: 5.0,
        }
    }

    fn weight(&self, rarity: Rarity) -> f32 {
        match rarity {
            Rarity::Common => self.common,
            Rarity::Rare => self.rare,
            Rarity::Epic => self.epic,
            Rarity::Legendary => self.legendary,
        }
    }
}

pub(super) fn generate_item_candidate_table(
    rarity_weights: ItemRarityWeights,
) -> Vec<(ItemDiscriminants, f32)> {
    ItemDiscriminants::iter()
        .map(|discriminant| {
            let weight = rarity_weights.weight(discriminant.rarity());
            (discriminant, weight)
        })
        .collect()
}

pub(super) fn generate_item_rarity_candidate_table(rarity: Rarity) -> Vec<ItemDiscriminants> {
    ItemDiscriminants::iter()
        .filter(|discriminant| discriminant.rarity() == rarity)
        .collect()
}
