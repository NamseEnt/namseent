use rand::*;
use sha2::{Digest, Sha256};

/// v4
pub fn uuid() -> u128 {
    let mut rng = rand::thread_rng();
    let mut bits: u128 = rng.gen();
    bits |= 0b010;
    bits &= !(0b110 << 62);
    bits
}

pub fn to_hashed_id(data: impl AsRef<[u8]>) -> u128 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    u128::from_le_bytes(hash[..16].try_into().unwrap())
}
