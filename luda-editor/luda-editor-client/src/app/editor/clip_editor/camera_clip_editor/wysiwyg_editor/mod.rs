pub mod background_wysiwyg_editor;
pub mod character_wysiwyg_editor;
pub mod resizer;

pub use background_wysiwyg_editor::*;
pub use character_wysiwyg_editor::*;
use namui::prelude::*;
pub use resizer::*;

pub enum WysiwygEvent {
    ResizerHandleMouseDownEvent {
        target_id: String,
        mouse_xy: namui::Xy<Px>,
        handle: ResizerHandle,
        center_xy: namui::Xy<Px>,
        container_size: namui::Wh<Px>,
        image_size_ratio: namui::Wh<Px>,
    },
    InnerImageMouseDownEvent {
        target_id: String,
        mouse_xy: namui::Xy<Px>,
        container_size: namui::Wh<Px>,
    },
}
