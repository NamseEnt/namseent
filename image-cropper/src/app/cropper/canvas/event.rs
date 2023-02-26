use super::{CanvasDragState, ToolType};
use namui::prelude::*;

pub enum CanvasEvent {
    Scrolled {
        offset: Xy<Px>,
    },
    Zoomed {
        offset: Xy<Px>,
        scale: f32,
    },
    LeftMouseDownInCanvas {
        position: Xy<Px>,
        tool_type: ToolType,
    },
    MouseMoveInCanvas(Xy<Px>),
    DragStarted(CanvasDragState),
    DragEnded,
}
