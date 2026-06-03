use crate::game_state::GameState;
use crate::game_state::upgrade::Upgrade;
use rand::RngCore;

/// Strategy for selecting treasure options.
pub trait TreasureStrategy: Send + Sync {
    fn name(&self) -> &str;
    fn select_treasure(
        &self,
        _game_state: &GameState,
        options: &[Upgrade],
        rng: &mut dyn RngCore,
    ) -> usize;
}

/// Strategy that picks treasures based on current upgrade state and stage economy.
pub struct SynergyTreasureStrategy;

impl TreasureStrategy for SynergyTreasureStrategy {
    fn name(&self) -> &str {
        "synergy_treasure"
    }

    fn select_treasure(
        &self,
        _game_state: &GameState,
        options: &[Upgrade],
        rng: &mut dyn RngCore,
    ) -> usize {
        if options.is_empty() {
            return 0;
        }

        let mut total_weight = 0u32;
        let mut weights = Vec::with_capacity(options.len());

        for option in options {
            let weight = Self::rarity_weight(option);
            weights.push(weight);
            total_weight += weight;
        }

        if total_weight == 0 {
            return 0;
        }

        let mut choice = (rng.next_u64() % total_weight as u64) as u32;
        for (idx, weight) in weights.iter().enumerate() {
            if choice < *weight {
                return idx;
            }
            choice -= *weight;
        }

        options.len() - 1
    }
}

impl SynergyTreasureStrategy {
    fn rarity_weight(option: &Upgrade) -> u32 {
        match option.discriminant().rarity() {
            crate::Rarity::Legendary => 50,
            crate::Rarity::Epic => 25,
            crate::Rarity::Rare => 10,
            crate::Rarity::Common => 5,
        }
    }
}
