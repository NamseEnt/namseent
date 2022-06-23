use crate::app::cropper::job::RectSelectionResizeDirection;

pub enum SelectionEvent {
    RectSelectionResizeHandleClicked {
        selection_id: String,
        direction: RectSelectionResizeDirection,
    },
    SelectionRightClicked {
        target_id: String,
    },
    PolySelectionCreateButtonClicked,
}
