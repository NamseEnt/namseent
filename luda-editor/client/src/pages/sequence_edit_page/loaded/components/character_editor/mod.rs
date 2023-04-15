mod render;
mod update;
use namui::prelude::*;
use namui_prebuilt::scroll_view::ScrollView;

pub struct CharacterEditor {
    variant_name_tooltip: Option<VariantNameTooltip>,
    scroll_view: ScrollView,
}

#[derive(Clone, Copy)]
pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    MouseDownOutsideCharacterEditor,
    OpenCharacterEditor,
}

enum InternalEvent {
    OpenVariantNameTooltip {
        global_xy: Xy<Px>,
        pose_name: String,
    },
    CloseVariantNameTooltip,
}

impl CharacterEditor {
    pub fn new() -> Self {
        let image_picker = Self {
            variant_name_tooltip: None,
            scroll_view: ScrollView::new(),
        };
        image_picker
    }
}

struct VariantNameTooltip {
    global_xy: Xy<Px>,
    pose_name: String,
}
