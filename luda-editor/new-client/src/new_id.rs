use rand::Rng;

// Duplicate of https://github.com/NamseEnt/namseent/blob/344fd633b31d3466c90412c58c3a6dc3b0deee0e/luda-editor/new-server/server/src/new_id.rs
pub fn new_id() -> u128 {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;
    u128::from_le_bytes(bytes)
}
