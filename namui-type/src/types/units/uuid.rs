use siphasher::sip128::{Hasher128, SipHasher};
pub use uuid::{uuid, Uuid};

pub fn uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn uuid_from_hash(hash: impl std::hash::Hash) -> Uuid {
    let mut hasher = SipHasher::new();
    hash.hash(&mut hasher);
    Uuid::from_u128(hasher.finish128().into())
}
