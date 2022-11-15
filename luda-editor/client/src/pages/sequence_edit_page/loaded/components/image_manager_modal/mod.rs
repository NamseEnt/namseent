mod render;
mod update;

use super::*;
use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

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
