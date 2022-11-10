mod render;
mod update;

use super::{image_edit_modal, screen_editor};
use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;
use std::{
    collections::{BTreeSet, VecDeque},
    sync::Arc,
};

pub struct ImageSelectModal {
    project_id: Uuid,
    pub cut_id: Uuid,
    context_menu: Option<context_menu::ContextMenu>,
    label_scroll_view: scroll_view::ScrollView,
    image_list_view: list_view::ListView,
    image_edit_modal: Option<image_edit_modal::ImageEditModal>,
    images: Vec<ImageWithLabels>,
    selected_labels: BTreeSet<Label>,
    selected_image: Option<ImageWithLabels>,
    on_update_image: Arc<dyn Fn(Update)>,
    selected_screen_image_index: Option<usize>,
    screen_editor: Option<screen_editor::ScreenEditor>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub recent_selected_image_ids: &'a VecDeque<Uuid>,
    pub cut: &'a Cut,
    pub project_shared_data: &'a ProjectSharedData,
}

pub enum Event {
    Close,
    Error(String),
}

enum InternalEvent {
    LoadImages(Vec<ImageWithLabels>),
    AddImageButtonClicked,
    ToggleLabel(Label),
    ImageSelected {
        image: ImageWithLabels,
        update_labels: bool,
    },
    EditScreenPressed {
        screen_images: ScreenImages,
    },
    SelectScreenImageIndex {
        index: usize,
    },
    ScreenEditDone {
        screen_images: ScreenImages,
    },
    RequestUploadBulkImages,
}

pub struct Update {
    pub cut_id: Uuid,
    pub screen_images: ScreenImages,
}

impl ImageSelectModal {
    pub fn new(
        project_id: Uuid,
        cut_id: Uuid,
        selected_screen_image_index: usize,
        on_update_image: impl Fn(Update) + 'static,
    ) -> ImageSelectModal {
        request_reload_images(project_id);
        ImageSelectModal {
            project_id,
            cut_id,
            context_menu: None,
            label_scroll_view: scroll_view::ScrollView::new(),
            image_list_view: list_view::ListView::new(),
            image_edit_modal: None,
            images: vec![],
            selected_labels: BTreeSet::new(),
            selected_image: None,
            on_update_image: Arc::new(on_update_image),
            selected_screen_image_index: Some(selected_screen_image_index),
            screen_editor: None,
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
