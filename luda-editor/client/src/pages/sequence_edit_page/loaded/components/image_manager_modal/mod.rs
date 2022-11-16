mod render;
mod update;

use super::*;
use namui::prelude::*;
use namui_prebuilt::*;

pub struct ImageManagerModal {
    image_table: image_table::ImageTable,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Close,
    Error(String),
}

impl ImageManagerModal {
    pub fn new(project_id: Uuid) -> ImageManagerModal {
        ImageManagerModal {
            image_table: image_table::ImageTable::new(project_id),
        }
    }
}
