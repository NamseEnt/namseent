mod render;
mod update;

use namui::prelude::*;
use rpc::data::{ScreenImage, ScreenImages};

pub struct WysiwygEditor {
    project_id: Uuid,
    dragging: Option<Dragging>,
    pub screen_images: ScreenImages,
    editing_image_index: Option<usize>,
}

pub struct Props {
    pub wh: Wh<Px>,
}

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
        index: usize,
    },
    ResizeImage {
        index: usize,
        circumscribed: rpc::data::Circumscribed<Percent>,
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
}

impl WysiwygEditor {
    pub fn new(project_id: Uuid, screen_images: ScreenImages) -> Self {
        Self {
            project_id,
            dragging: None,
            screen_images,
            editing_image_index: None,
        }
    }
}
