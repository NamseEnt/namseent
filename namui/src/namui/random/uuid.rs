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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn try_from_base64_str() {
        use uuid::uuid;

        assert_eq!(
            "\"67e55044-10b1-426f-9247-bb680e5fe0c8\"",
            serde_json::to_string(&uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap()
        );
        assert_eq!(
            uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            serde_json::from_str("\"67e55044-10b1-426f-9247-bb680e5fe0c8\"").unwrap()
        );
    }
}
