mod render;
mod update;
use namui::prelude::*;
use namui_prebuilt::scroll_view::ScrollView;
use rpc::data::{Cut, CutUpdateAction, ScreenGraphic};

pub struct CharacterEditor {
    tooltip: Option<Tooltip>,
    scroll_view: ScrollView,
    edit_target: EditTarget,
}

#[derive(Clone, Copy)]
pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
    pub cut: Option<&'a Cut>,
}

pub enum Event {
    MouseDownOutsideCharacterEditor,
    OpenCharacterEditor { target: EditTarget },
}

enum InternalEvent {
    OpenTooltip {
        global_xy: Xy<Px>,
        text: String,
    },
    CloseTooltip,
    CgChangeButtonClicked,
    CgThumbnailClicked {
        cg_id: Uuid,
    },
    FocusCg {
        cut_id: Uuid,
        cg_id: Uuid,
        graphic_index: Uuid,
    },
}

impl CharacterEditor {
    pub fn new(edit_target: EditTarget) -> Self {
        Self {
            tooltip: None,
            scroll_view: ScrollView::new(),
            edit_target,
        }
    }
}

struct Tooltip {
    global_xy: Xy<Px>,
    text: String,
}

#[derive(Clone, Copy)]
pub enum EditTarget {
    NewCharacter {
        cut_id: Uuid,
    },
    ExistingCharacter {
        cut_id: Uuid,
        graphic_index: Uuid,
    },
    ExistingCharacterPart {
        cut_id: Uuid,
        cg_id: Uuid,
        graphic_index: Uuid,
    },
}
