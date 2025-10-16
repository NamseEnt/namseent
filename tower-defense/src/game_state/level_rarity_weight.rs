use super::GameState;
use crate::{rarity::Rarity, *};
use std::num::NonZero;

pub const LEVEL_RARITY_WEIGHT: [[usize; 4]; 10] = [
    [90, 10, 1, 0],
    [75, 25, 5, 0],
    [55, 30, 10, 1],
    [45, 45, 20, 2],
    [25, 40, 30, 5],
    [20, 30, 35, 15],
    [15, 30, 40, 20],
    [10, 25, 35, 25],
    [5, 25, 30, 30],
    [5, 20, 30, 40],
];

#[derive(Debug, Clone, Copy, Default, State)]
pub struct RarityGenerationOption {
    pub no_common: bool,
}

impl GameState {
    pub fn generate_rarity(&self, option: RarityGenerationOption) -> Rarity {
        let mut weights = level_rarity_weight(self.level);
        if option.no_common {
            weights[0] = 0;
        }
        let total_weight: usize = weights.iter().sum();
        let random_value = rand::random::<usize>() % total_weight;

        let mut cumulative_weight = 0;
        const RARITIES: [Rarity; 4] = [
            Rarity::Common,
            Rarity::Rare,
            Rarity::Epic,
            Rarity::Legendary,
        ];
        for (i, &weight) in weights.iter().enumerate() {
            cumulative_weight += weight;
            if random_value < cumulative_weight {
                return RARITIES[i];
            }
        }
        unreachable!()
    }
}

pub fn level_rarity_weight(level: NonZero<usize>) -> [usize; 4] {
    LEVEL_RARITY_WEIGHT[level.get() - 1]
}
