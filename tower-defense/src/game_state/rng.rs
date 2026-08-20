use crate::deterministic_rng;
use namui::*;
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;

#[derive(Clone, Debug, State)]
pub(crate) struct ShopGenerationConfig {
    pub(crate) category_bag_size: usize,
    pub(crate) category_weights: Vec<u32>,
    pub(crate) rarity_bag_size: usize,
    pub(crate) item_rarity_weights: Vec<u32>,
    pub(crate) card_service_rarity_weights: Vec<u32>,
    pub(crate) upgrade_rarity_weights: Vec<u32>,
}

impl Default for ShopGenerationConfig {
    fn default() -> Self {
        Self {
            category_bag_size: 10,
            category_weights: vec![50, 30, 20],
            rarity_bag_size: 10,
            item_rarity_weights: vec![150, 175, 20, 0],
            card_service_rarity_weights: vec![150, 175, 60, 0],
            upgrade_rarity_weights: vec![450, 375, 70, 30],
        }
    }
}

#[derive(Clone, Debug, Default, State)]
pub(crate) struct BagState {
    pub(crate) entries: Vec<u8>,
    pub(crate) cursor: usize,
    pub(crate) cycle: u64,
}

#[derive(Clone, Debug, Default, State)]
pub(crate) struct ContentBagState {
    pub(crate) entries: Vec<String>,
    pub(crate) cursor: usize,
    pub(crate) cycle: u64,
}

#[derive(Clone, Debug, State)]
pub(crate) struct ShopBagState {
    pub(crate) generation_sequence: u64,
    pub(crate) config: ShopGenerationConfig,
    pub(crate) category_bag: BagState,
    pub(crate) rarity_bags: Vec<BagState>,
    pub(crate) content_bags: Vec<ContentBagState>,
}

impl Default for ShopBagState {
    fn default() -> Self {
        Self {
            generation_sequence: 0,
            config: ShopGenerationConfig::default(),
            category_bag: BagState::default(),
            rarity_bags: vec![BagState::default(); 3],
            content_bags: vec![ContentBagState::default(); 12],
        }
    }
}

#[derive(Clone, Debug, State)]
pub(crate) struct GameRngState {
    pub(crate) seed: u64,
    pub(crate) shop: ShopBagState,
    pub(crate) domain_sequences: BTreeMap<u64, u64>,
}

impl GameRngState {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            seed,
            shop: ShopBagState::default(),
            domain_sequences: BTreeMap::new(),
        }
    }

    pub(crate) fn rng_for(&self, domain: u64, coordinates: &[u64]) -> ChaCha8Rng {
        deterministic_rng::rng_for(self.seed, domain, coordinates)
    }

    pub(crate) fn next_rng(&mut self, domain: u64, coordinates: &[u64]) -> ChaCha8Rng {
        let sequence = self.domain_sequences.entry(domain).or_default();
        let sequence_value = *sequence;
        *sequence = sequence.wrapping_add(1);

        let mut derived_coordinates = Vec::with_capacity(coordinates.len() + 1);
        derived_coordinates.push(sequence_value);
        derived_coordinates.extend_from_slice(coordinates);
        deterministic_rng::rng_for(self.seed, domain, &derived_coordinates)
    }

    pub(crate) fn next_shop_generation_sequence(&mut self) -> u64 {
        let sequence = self.shop.generation_sequence;
        self.shop.generation_sequence = self.shop.generation_sequence.wrapping_add(1);
        sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;

    #[test]
    fn next_rng_is_reproducible_and_advances_per_domain() {
        let mut left = GameRngState::new(42);
        let mut right = GameRngState::new(42);

        let left_first = left
            .next_rng(crate::deterministic_rng::domain::CARD_REROLL, &[7])
            .next_u64();
        let left_second = left
            .next_rng(crate::deterministic_rng::domain::CARD_REROLL, &[7])
            .next_u64();
        let right_first = right
            .next_rng(crate::deterministic_rng::domain::CARD_REROLL, &[7])
            .next_u64();

        assert_eq!(left_first, right_first);
        assert_ne!(left_first, left_second);
        assert_eq!(
            left.domain_sequences
                .get(&crate::deterministic_rng::domain::CARD_REROLL),
            Some(&2)
        );
    }

    #[test]
    fn next_rng_keeps_domains_independent() {
        let mut left = GameRngState::new(42);
        let mut right = GameRngState::new(42);

        let left_value = left
            .next_rng(crate::deterministic_rng::domain::DECK_SHUFFLE, &[1])
            .next_u64();
        let right_value = right
            .next_rng(crate::deterministic_rng::domain::DECK_DRAW, &[1])
            .next_u64();

        assert_ne!(left_value, right_value);
    }
}
