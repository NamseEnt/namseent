use crate::deterministic_rng;
use namui::*;
use rand_chacha::ChaCha8Rng;

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
}

impl GameRngState {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            seed,
            shop: ShopBagState::default(),
        }
    }

    pub(crate) fn rng_for(&self, domain: u64, coordinates: &[u64]) -> ChaCha8Rng {
        deterministic_rng::rng_for(self.seed, domain, coordinates)
    }

    pub(crate) fn next_shop_generation_sequence(&mut self) -> u64 {
        let sequence = self.shop.generation_sequence;
        self.shop.generation_sequence = self.shop.generation_sequence.wrapping_add(1);
        sequence
    }
}
