pub mod events;
pub use events::Event;
pub mod animation_editor;
pub use animation_editor::{AnimationEditor, Props};
pub(crate) mod image_select_window;
pub(crate) mod layer_list_window;
mod time_point_editor;
pub(crate) mod time_ruler;
mod types;
pub(crate) use types::*;
pub(crate) mod zoom;

pub(crate) fn adjust_font_size(height: f32) -> i16 {
    // 0, 4, 8, 16, 20, ...
    let mut font_size = (height * 0.7) as i16;
    font_size -= font_size % 4;
    font_size
}

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
