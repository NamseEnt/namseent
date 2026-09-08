use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub const RNG_ALGORITHM_VERSION: u32 = 1;

pub mod domain {
    pub const DECK_SHUFFLE: u64 = 0x4445_434b_0000_0001;
    pub const DECK_DRAW: u64 = 0x4445_434b_0000_0002;
    pub const CARD_REROLL: u64 = 0x4341_5244_0000_0001;
    pub const DIFFICULTY_OFFER: u64 = 0x4449_4646_0000_0001;
    pub const EFFECT_PAYLOAD: u64 = 0x4546_4645_0000_0001;
    pub const ITEM_GENERATION: u64 = 0x4954_454d_0000_0001;
    pub const REWARD_UPGRADE: u64 = 0x5245_5741_0000_0001;
    pub const SHOP_CATEGORY_BAG: u64 = 0x5348_4f50_0000_0001;
    pub const SHOP_RARITY_ITEM: u64 = 0x5348_4f50_0000_0002;
    pub const SHOP_RARITY_CARD_SERVICE: u64 = 0x5348_4f50_0000_0003;
    pub const SHOP_RARITY_UPGRADE: u64 = 0x5348_4f50_0000_0004;
    pub const SHOP_CONTENT_ITEM: u64 = 0x5348_4f50_0000_0005;
    pub const SHOP_CONTENT_CARD_SERVICE: u64 = 0x5348_4f50_0000_0006;
    pub const SHOP_CONTENT_UPGRADE: u64 = 0x5348_4f50_0000_0007;
    pub const SHOP_ITEM_PAYLOAD: u64 = 0x5348_4f50_0000_0008;
    pub const SHOP_PRICE: u64 = 0x5348_4f50_0000_0009;
    pub const ML_TOWER_EXPERT: u64 = 0x4d4c_544f_0000_0001;
}

fn split_mix(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = value;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn derive_seed(master_seed: u64, domain: u64, coordinates: &[u64]) -> [u8; 32] {
    let mut state = split_mix(
        u64::from(RNG_ALGORITHM_VERSION)
            .wrapping_mul(0xD6E8_FEB8_6659_FD93)
            .wrapping_add(master_seed)
            .wrapping_add(domain.rotate_left(17))
            .wrapping_add(coordinates.len() as u64),
    );

    for &coordinate in coordinates {
        state = split_mix(state ^ coordinate.wrapping_mul(0xA24B_AED4_963E_E407));
    }

    let mut seed = [0; 32];
    for (index, chunk) in seed.chunks_exact_mut(8).enumerate() {
        state = split_mix(state ^ (index as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        chunk.copy_from_slice(&state.to_le_bytes());
    }
    seed
}

pub fn rng_for(master_seed: u64, domain: u64, coordinates: &[u64]) -> ChaCha8Rng {
    ChaCha8Rng::from_seed(derive_seed(master_seed, domain, coordinates))
}

pub fn uniform_index(rng: &mut impl RngCore, upper: usize) -> usize {
    assert!(upper > 0, "uniform_index requires a non-empty range");
    if upper == 1 {
        return 0;
    }

    let upper = upper as u64;
    let limit = u64::MAX - (u64::MAX % upper);
    loop {
        let value = rng.next_u64();
        if value < limit {
            return (value % upper) as usize;
        }
    }
}

pub fn shuffle<T>(values: &mut [T], rng: &mut impl RngCore) {
    for index in (1..values.len()).rev() {
        let swap_index = uniform_index(rng, index + 1);
        values.swap(index, swap_index);
    }
}

pub fn stable_key_hash(key: &str) -> u64 {
    let mut hash = 0xCBF2_9CE4_8422_2325u64;
    for byte in key.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}
