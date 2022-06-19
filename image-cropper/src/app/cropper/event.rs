use super::{canvas::Tool, job::RectSelectionResizeDirection};
use namui::Xy;

pub enum CropperEvent {
    MouseDownInCanvas {
        position: Xy<f32>,
        tool: Tool,
    },
    RectSelectionResizeHandleClicked {
        selection_id: String,
        direction: RectSelectionResizeDirection,
    },
    MouseMoveInCanvas(Xy<f32>),
    SaveButtonClicked,
}
