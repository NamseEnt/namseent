mod render;
mod update;

use namui::prelude::*;

pub struct CharacterPicker {
    project_id: Uuid,
    pose_files: Vec<PoseFile>,
}

#[derive(Clone, Copy)]
pub struct Props {
    pub wh: Wh<Px>,
}

enum InternalEvent {
    ImagesLoaded(Vec<PoseFile>),
}

pub enum Event {
    MouseDownOutsideCharacterPicker,
    OpenCharacterPicker,
}

impl CharacterPicker {
    pub fn new(project_id: Uuid) -> Self {
        let mut image_picker = Self {
            project_id,
            pose_files: Vec::new(),
        };
        image_picker.fetch_pose_files();
        image_picker
    }

    // TODO
    fn fetch_pose_files(&mut self) {
        let project_id = self.project_id;
        crate::RPC
            .list_images(rpc::list_images::Request { project_id })
            .callback(|result| match result {
                Ok(response) => {
                    namui::event::send(InternalEvent::ImagesLoaded(
                        response
                            .images
                            .iter()
                            .map(|image| PoseFile {
                                id: image.id,
                                name: image.id.to_string(),
                                thumbnail_url: Url::parse(&image.url).unwrap(),
                            })
                            .collect(),
                    ));
                }
                Err(error) => {
                    namui::log!("error: {error}");
                }
            })
    }
}

#[derive(Clone)]
struct PoseFile {
    id: Uuid,
    name: String,
    thumbnail_url: Url,
}
impl PoseFile {
    fn thumbnail_url(&self) -> Url {
        self.thumbnail_url.clone()
    }
}
