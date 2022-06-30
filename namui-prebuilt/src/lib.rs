pub mod button;
pub mod list_view;
pub mod scroll_view;
mod simple_rect;
pub mod table;
pub mod typography;

use namui::prelude::*;
pub use simple_rect::simple_rect;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
