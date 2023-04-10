mod render;
mod update;

use namui::prelude::*;
use rpc::data::ImageWithLabels;

pub struct CharacterPicker {
    project_id: Uuid,
    images: Vec<ImageWithLabels>,
}

#[derive(Clone, Copy)]
pub struct Props {
    pub wh: Wh<Px>,
}

pub enum InternalEvent {
    ImagesLoaded(Vec<ImageWithLabels>),
}

impl CharacterPicker {
    pub fn new(project_id: Uuid) -> Self {
        let mut image_picker = Self {
            project_id,
            images: Vec::new(),
        };
        image_picker.fetch_images();
        image_picker
    }

    fn fetch_images(&mut self) {
        let project_id = self.project_id;
        crate::RPC
            .list_images(rpc::list_images::Request { project_id })
            .callback(|result| match result {
                Ok(response) => {
                    namui::event::send(InternalEvent::ImagesLoaded(response.images));
                }
                Err(error) => {
                    namui::log!("error: {error}");
                }
            })
    }
}
