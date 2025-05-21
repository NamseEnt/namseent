pub use namui_type::uuid;

pub fn random(length: usize) -> Vec<u8> {
    use rand::RngCore;

    let mut array = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut array);
    array
}
