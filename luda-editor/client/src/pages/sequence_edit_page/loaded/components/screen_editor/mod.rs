mod render;
mod update;

use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct ScreenEditor {
    project_id: Uuid,
    screen_images: Vec<ScreenImage>,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Error(String),
}

enum InternalEvent {}

impl ScreenEditor {
    pub fn new(project_id: Uuid, screen_images: impl IntoIterator<Item = ScreenImage>) -> Self {
        Self {
            project_id,
            screen_images: screen_images.into_iter().collect(),
        }
    }
}
