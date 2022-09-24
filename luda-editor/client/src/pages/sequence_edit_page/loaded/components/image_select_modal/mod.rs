mod render;
mod update;

use super::image_edit_modal;
use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct ImageSelectModal {
    project_id: Uuid,
    context_menu: Option<context_menu::ContextMenu>,
    label_scroll_view: scroll_view::ScrollView,
    image_list_scroll_view: scroll_view::ScrollView,
    image_edit_modal: Option<image_edit_modal::ImageEditModal>,
    images: Vec<UrlWithLabels>,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Close,
    Error(String),
}

enum InternalEvent {
    LoadImages(Vec<UrlWithLabels>),
    AddImageButtonClicked,
}

impl ImageSelectModal {
    pub fn new(project_id: Uuid) -> ImageSelectModal {
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

        ImageSelectModal {
            project_id,
            context_menu: None,
            label_scroll_view: scroll_view::ScrollView::new(),
            image_list_scroll_view: scroll_view::ScrollView::new(),
            image_edit_modal: None,
            images: vec![],
        }
    }
}
