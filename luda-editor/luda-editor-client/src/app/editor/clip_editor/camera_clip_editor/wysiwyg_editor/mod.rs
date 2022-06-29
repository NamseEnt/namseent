pub mod background_wysiwyg_editor;
pub use background_wysiwyg_editor::*;
pub mod character_wysiwyg_editor;
pub use character_wysiwyg_editor::*;
pub mod resizer;
pub use resizer::*;

pub enum WysiwygEvent {
    ResizerHandleMouseDownEvent {
        target_id: String,
        mouse_xy: namui::Xy<f32>,
        handle: ResizerHandle,
        center_xy: namui::Xy<f32>,
        container_size: namui::Wh<f32>,
        image_size_ratio: namui::Wh<f32>,
    },
    InnerImageMouseDownEvent {
        target_id: String,
        mouse_xy: namui::Xy<f32>,
        container_size: namui::Wh<f32>,
    },
}
