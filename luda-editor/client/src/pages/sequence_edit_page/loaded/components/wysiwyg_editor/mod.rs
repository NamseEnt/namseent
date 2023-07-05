mod render;
mod update;

use crate::components::context_menu;
use namui::prelude::*;
use rpc::data::{CgFile, CutUpdateAction, ScreenGraphic};

pub struct WysiwygEditor {
    dragging: Option<Dragging>,
    editing_image_index: Option<Uuid>,
    context_menu: Option<context_menu::ContextMenu>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub cut_id: Uuid,
    pub screen_graphics: &'a Vec<(Uuid, ScreenGraphic)>,
    pub project_id: Uuid,
    pub cg_files: &'a Vec<CgFile>,
}

#[derive(Debug, Clone)]
enum Dragging {
    Resizer {
        context: render::resizer::ResizerDraggingContext,
    },
    // Cropper,
    Mover {
        context: render::mover::MoverDraggingContext,
    },
}

enum InternalEvent {
    SelectImage {
        index: Uuid,
    },
    ImageMoveStart {
        start_global_xy: Xy<Px>,
        end_global_xy: Xy<Px>,
        container_wh: Wh<Px>,
    },
    MouseMoveContainer {
        global_xy: Xy<Px>,
    },
    MouseDownContainer,
    MouseUp,
    OpenContextMenu {
        global_xy: Xy<Px>,
        cut_id: Uuid,
        graphic_index: Uuid,
        graphic_wh: Wh<Px>,
        graphic: ScreenGraphic,
    },
}

impl WysiwygEditor {
    pub fn new() -> Self {
        Self {
            dragging: None,
            editing_image_index: None,
            context_menu: None,
        }
    }
}
