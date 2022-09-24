pub mod uuid;

#[cfg(target_family = "wasm")]
pub fn random(length: usize) -> Vec<u8> {
    let mut array = vec![0u8; length];
    let window = web_sys::window().unwrap();
    let crypto = window.crypto().unwrap();
    crypto.get_random_values_with_u8_array(&mut array).unwrap();
    array
}
