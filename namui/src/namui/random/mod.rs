pub use namui_type::uuid;

#[cfg(target_family = "wasm")]
pub fn random(length: usize) -> Vec<u8> {
    let mut array = vec![0u8; length];
    let window = web_sys::window().unwrap();
    let crypto = window.crypto().unwrap();
    crypto.get_random_values_with_u8_array(&mut array).unwrap();
    array
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn serde_should_work_with_uuid() {
        use super::uuid;

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
