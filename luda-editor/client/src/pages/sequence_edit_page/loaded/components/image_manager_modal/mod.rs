mod render;
mod update;

use super::*;
use namui::prelude::*;
use namui_prebuilt::*;

pub struct ImageManagerModal {
    image_table: image_table::ImageTable,
    project_id: Uuid,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Close,
    Error(String),
}

enum InternalEvent {
    RequestUploadImages,
    UploadImageFinished,
}

impl ImageManagerModal {
    pub fn new(project_id: Uuid) -> ImageManagerModal {
        ImageManagerModal {
            image_table: image_table::ImageTable::new(project_id),
            project_id,
        }
    }
}
