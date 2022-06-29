use namui::prelude::*;
pub mod list_view;
pub mod scroll_view;
mod simple_rect;
pub use simple_rect::simple_rect;
pub mod button;
pub mod table;
pub mod typography;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
