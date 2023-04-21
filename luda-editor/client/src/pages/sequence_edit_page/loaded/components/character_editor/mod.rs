mod render;
mod update;
use namui::prelude::*;
use namui_prebuilt::scroll_view::ScrollView;

pub struct CharacterEditor {
    tooltip: Option<Tooltip>,
    scroll_view: ScrollView,
    edit_target: EditTarget,
}

#[derive(Clone, Copy)]
pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    MouseDownOutsideCharacterEditor,
    OpenCharacterEditor { target: EditTarget },
}

enum InternalEvent {
    OpenTooltip { global_xy: Xy<Px>, text: String },
    CloseTooltip,
}

impl CharacterEditor {
    pub fn new(edit_target: EditTarget) -> Self {
        let image_picker = Self {
            tooltip: None,
            scroll_view: ScrollView::new(),
            edit_target,
        };
        image_picker
    }
}

struct Tooltip {
    global_xy: Xy<Px>,
    text: String,
}

#[derive(Clone, Copy)]
pub enum EditTarget {
    NewCharacterPose,
    // ExistingCharacterPose,
    ExistingCharacterPart,
}
