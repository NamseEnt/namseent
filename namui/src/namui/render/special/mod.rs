pub mod translate;
pub use translate::*;
pub mod clip;
pub use clip::*;
pub mod attach_event;
pub use attach_event::*;
pub mod mouse_cursor;
pub use mouse_cursor::*;
pub mod with_id;
pub use with_id::*;
pub mod absolute;
pub use absolute::*;
pub mod rotate;
pub use rotate::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum SpecialRenderingNode {
    Translate(TranslateNode),
    Clip(ClipNode),
    AttachEvent(AttachEventNode),
    MouseCursor(MouseCursorNode),
    WithId(WithIdNode),
    Absolute(AbsoluteNode),
    Rotate(RotateNode),
}
