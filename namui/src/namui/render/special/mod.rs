pub mod absolute;
pub mod attach_event;
pub mod clip;
pub mod custom;
pub mod mouse_cursor;
pub mod on_top;
pub mod react;
pub mod rotate;
pub mod scale;
pub mod transform;
pub mod translate;
pub mod with_id;

pub use absolute::*;
pub use attach_event::*;
pub use clip::*;
pub use custom::*;
pub use mouse_cursor::*;
pub use on_top::*;
pub use react::*;
pub use rotate::*;
pub use scale::*;
use serde::Serialize;
pub use transform::*;
pub use translate::*;
pub use with_id::*;

#[derive(Serialize, Clone, Debug)]
pub enum SpecialRenderingNode {
    Translate(TranslateNode),
    Clip(ClipNode),
    AttachEvent(AttachEventNode),
    MouseCursor(MouseCursorNode),
    WithId(WithIdNode),
    Absolute(AbsoluteNode),
    Rotate(RotateNode),
    Custom(CustomNode),
    Scale(ScaleNode),
    Transform(TransformNode),
    React(ReactNode),
    OnTop(OnTopNode),
}
