use namui_type::Uuid;

pub fn get_image_hash(image: &[u8]) -> Uuid {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(image);
    let hash = hasher.finalize().to_le_bytes();
    let bytes: [u8; 16] = [
        hash[0], hash[1], hash[2], hash[3], hash[0], hash[1], hash[2], hash[3], hash[0], hash[1],
        hash[2], hash[3], hash[0], hash[1], hash[2], hash[3],
    ];
    Uuid::from_bytes(bytes)
}
