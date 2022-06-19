use super::{canvas::ToolType, job::RectSelectionResizeDirection};
use namui::Xy;

pub enum CropperEvent {
    LeftMouseDownInCanvas {
        position: Xy<f32>,
        tool_type: ToolType,
    },
    RectSelectionResizeHandleClicked {
        selection_id: String,
        direction: RectSelectionResizeDirection,
    },
    MouseMoveInCanvas(Xy<f32>),
    SaveButtonClicked,
    SelectionRightClicked {
        target_id: String,
    },
}
