mod render;
mod update;

use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::Storage;
use std::sync::Arc;

#[derive(Clone)]
pub struct ImageBrowser {
    list_view: list_view::ListView,
    storage: Storage,
    resources: Box<[String]>,
    pub on_item_click: Arc<dyn Fn(&str)>,
}

impl std::fmt::Debug for ImageBrowser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImageBrowser: {:?}", self.resources)
    }
}

pub struct Props {
    pub wh: Wh<Px>,
    pub selected_resource: Option<String>,
}

enum Event {
    PlusButtonClicked,
    RequestListRefresh,
    ListRefreshed { resources: Box<[String]> },
}

impl ImageBrowser {
    pub fn new(storage: Storage, on_item_click: impl Fn(&str) + 'static) -> Self {
        namui::event::send(Event::RequestListRefresh);
        Self {
            list_view: list_view::ListView::new(),
            storage,
            resources: Box::new([]),
            on_item_click: Arc::new(on_item_click),
        }
    }
}
