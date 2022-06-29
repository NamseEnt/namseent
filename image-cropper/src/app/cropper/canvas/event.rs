use super::{CanvasDragState, ToolType};
use namui::Xy;

pub enum CanvasEvent {
    Scrolled {
        offset: Xy<f32>,
    },
    Zoomed {
        offset: Xy<f32>,
        scale: f32,
    },
    LeftMouseDownInCanvas {
        position: Xy<f32>,
        tool_type: ToolType,
    },
    MouseMoveInCanvas(Xy<f32>),
    DragStarted(CanvasDragState),
}
