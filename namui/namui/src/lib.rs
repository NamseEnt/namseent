// #![deny(warnings)]

pub mod namui;
pub mod prelude;

pub use namui::*;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
