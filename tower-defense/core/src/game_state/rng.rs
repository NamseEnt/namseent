#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShopGenerationConfig {
    pub category_bag_size: usize,
    pub category_weights: Vec<u32>,
    pub rarity_bag_size: usize,
    pub item_rarity_weights: Vec<u32>,
    pub card_service_rarity_weights: Vec<u32>,
    pub upgrade_rarity_weights: Vec<u32>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BagState {
    pub entries: Vec<u8>,
    pub cursor: usize,
    pub cycle: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContentBagState {
    pub entries: Vec<String>,
    pub cursor: usize,
    pub cycle: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShopBagState {
    pub generation_sequence: u64,
    pub config: ShopGenerationConfig,
    pub category_bag: BagState,
    pub rarity_bags: Vec<BagState>,
    pub content_bags: Vec<ContentBagState>,
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RngState {
    pub seed: u64,
    pub shop: ShopBagState,
    pub domain_sequences: std::collections::BTreeMap<u64, u64>,
}

impl RngState {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            shop: ShopBagState::default(),
            domain_sequences: std::collections::BTreeMap::new(),
        }
    }

    pub fn rng_for(&self, domain: u64, coordinates: &[u64]) -> rand_chacha::ChaCha8Rng {
        crate::deterministic_rng::rng_for(self.seed, domain, coordinates)
    }

    pub fn next_rng(&mut self, domain: u64, coordinates: &[u64]) -> rand_chacha::ChaCha8Rng {
        let sequence = self.domain_sequences.entry(domain).or_default();
        let sequence_value = *sequence;
        *sequence = sequence.wrapping_add(1);

        let mut derived_coordinates = Vec::with_capacity(coordinates.len() + 1);
        derived_coordinates.push(sequence_value);
        derived_coordinates.extend_from_slice(coordinates);
        crate::deterministic_rng::rng_for(self.seed, domain, &derived_coordinates)
    }

    pub fn next_shop_generation_sequence(&mut self) -> u64 {
        let sequence = self.shop.generation_sequence;
        self.shop.generation_sequence = self.shop.generation_sequence.wrapping_add(1);
        sequence
    }
}
