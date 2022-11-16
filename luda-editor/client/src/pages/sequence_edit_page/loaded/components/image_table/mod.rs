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
}

enum SortOrderBy {
    Ascending { key: String },
    Descending { key: String },
}

impl ImageTable {
    pub fn new(project_id: Uuid) -> ImageTable {
        request_reload_images(project_id);
        ImageTable {
            project_id,
            list_view: list_view::ListView::new(),
            images: vec![],
            sort_order_by: None,
        }
    }
}
pub fn request_reload_images(project_id: Uuid) {
    spawn_local({
        async move {
            let result = crate::RPC
                .list_images(rpc::list_images::Request { project_id })
                .await;

            match result {
                Ok(response) => {
                    namui::event::send(InternalEvent::LoadImages(response.images));
                }
                Err(error) => {
                    namui::event::send(Event::Error(error.to_string()));
                }
            }
        }
    });
}
