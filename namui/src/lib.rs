pub mod namui;
pub use namui::*;
pub mod prelude;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
