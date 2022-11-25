use super::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
pub fn read_write_delete() {
    let key = "__test__";
    let content = "test content";

    assert!(read(key).is_err());

    assert!(write(key, content).is_ok());
    assert_eq!(read(key).unwrap(), content.to_string());

    assert!(delete(key).is_ok());
    assert!(read(key).is_err());
}
