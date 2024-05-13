// #![deny(warnings)]

mod namui;

pub use namui::*;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
