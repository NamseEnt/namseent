mod render;
mod update;

use namui::prelude::*;
use rpc::data::ScreenImage;

pub struct WysiwygEditor {
    dragging: Option<Dragging>,
    editing_image_index: Option<usize>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub cut_id: Uuid,
    pub screen_images: &'a Vec<ScreenImage>,
    pub project_id: Uuid,
}

#[derive(Debug)]
enum Dragging {
    Resizer {
        context: render::resizer::ResizerDraggingContext,
    },
    // Cropper,
    Mover {
        context: render::mover::MoverDraggingContext,
    },
}

pub enum Event {
    UpdateImages {
        cut_id: Uuid,
        callback: Box<dyn Fn(&mut Vec<ScreenImage>) -> () + 'static + Send + Sync>,
    },
}

enum InternalEvent {
    SelectImage {
        index: usize,
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
    MouseUp {
        global_xy: Xy<Px>,
        cut_id: Uuid,
    },
}

impl WysiwygEditor {
    pub fn new() -> Self {
        Self {
            dragging: None,
            editing_image_index: None,
        }
    }
}
