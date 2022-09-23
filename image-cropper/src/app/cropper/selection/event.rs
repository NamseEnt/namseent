use crate::app::cropper::job::RectSelectionResizeDirection;

pub enum SelectionEvent {
    RectSelectionResizeHandleClicked {
        selection_id: namui::Uuid,
        direction: RectSelectionResizeDirection,
    },
    SelectionRightClicked {
        target_id: namui::Uuid,
    },
    PolySelectionCreateButtonClicked,
}
