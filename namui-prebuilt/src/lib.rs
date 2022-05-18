pub mod list_view;
pub mod scroll_view;
mod simple_rect;
pub use simple_rect::simple_rect;
mod center_text;
pub use center_text::center_text;
pub mod table;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
