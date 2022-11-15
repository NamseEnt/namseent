mod render;
mod update;

use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct ImageTable {
    project_id: Uuid,
    list_view: list_view::ListView,
    images: Vec<ImageWithLabels>,
    sort_order_by: Option<SortOrderBy>,
    text_input: text_input::TextInput,
    editing_target: Option<EditingTarget>,
    pub saving_count: usize,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Error(String),
}

enum InternalEvent {
    LoadImages(Vec<ImageWithLabels>),
    LeftClickOnLabelHeader { key: String },
    LeftClickOnLabelCell { image_id: Uuid, label_key: String },
    PutImageMetaDataSuccess,
}

enum SortOrderBy {
    Ascending { key: String },
    Descending { key: String },
}

struct EditingTarget {
    image_id: Uuid,
    label_key: String,
}

impl ImageTable {
    pub fn new(project_id: Uuid) -> ImageTable {
        let table = ImageTable {
            project_id,
            list_view: list_view::ListView::new(),
            images: vec![],
            sort_order_by: None,
            text_input: text_input::TextInput::new(),
            editing_target: None,
            saving_count: 0,
        };

        table.request_reload_images();
        table
    }
    pub fn request_reload_images(&self) {
        crate::RPC
            .list_images(rpc::list_images::Request {
                project_id: self.project_id,
            })
            .callback(|result| match result {
                Ok(response) => {
                    namui::event::send(InternalEvent::LoadImages(response.images));
                }
                Err(error) => {
                    namui::event::send(Event::Error(error.to_string()));
                }
            })
    }
}
