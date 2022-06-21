pub mod events;
pub use events::Event;
mod graph_editor;
mod read_only_lock;
pub(crate) use read_only_lock::ReadOnlyLock;
pub mod animation_editor;
pub use animation_editor::{AnimationEditor, Props};
mod time_point_editor;
pub(crate) mod time_ruler;

pub(crate) fn adjust_font_size(height: f32) -> i16 {
    // 0, 4, 8, 16, 20, ...
    let mut font_size = (height * 0.7) as i16;
    font_size -= font_size % 4;
    font_size
}
