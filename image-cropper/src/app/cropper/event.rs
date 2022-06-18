use super::{job::RectSelectionResizeDirection, selection::Selection};
use namui::Xy;

pub enum CropperEvent {
    SelectionCreate(Selection),
    RectSelectionResizeHandleClicked {
        selection_id: String,
        direction: RectSelectionResizeDirection,
    },
    MouseMoveInCanvas(Xy<f32>),
    SaveButtonClicked,
}
