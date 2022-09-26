mod render;
mod update;

use super::image_edit_modal;
use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;
use std::collections::BTreeSet;

pub struct ImageSelectModal {
    project_id: Uuid,
    context_menu: Option<context_menu::ContextMenu>,
    label_scroll_view: scroll_view::ScrollView,
    image_list_scroll_view: scroll_view::ScrollView,
    image_edit_modal: Option<image_edit_modal::ImageEditModal>,
    images: Vec<ImageWithLabels>,
    selected_labels: BTreeSet<Label>,
    selected_image: Option<ImageWithLabels>,
    on_done: Box<dyn Fn(Option<Uuid>)>,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Close,
    Error(String),
}

enum InternalEvent {
    LoadImages(Vec<ImageWithLabels>),
    AddImageButtonClicked,
    ToggleLabel(Label),
    ImageSelected(ImageWithLabels),
    Done { image_id: Option<Uuid> },
}

impl ImageSelectModal {
    pub fn new(project_id: Uuid, on_done: impl Fn(Option<Uuid>) + 'static) -> ImageSelectModal {
        let modal = ImageSelectModal {
            project_id,
            context_menu: None,
            label_scroll_view: scroll_view::ScrollView::new(),
            image_list_scroll_view: scroll_view::ScrollView::new(),
            image_edit_modal: None,
            images: vec![],
            selected_labels: BTreeSet::new(),
            selected_image: None,
            on_done: Box::new(on_done),
        };
        modal.request_reload_images();
        modal
    }
    pub fn request_reload_images(&self) {
        let project_id = self.project_id;
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
}
