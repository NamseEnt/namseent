mod render;
mod update;

use super::wysiwyg_editor;
use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct ScreenEditor {
    project_id: Uuid,
    wysiwyg_editor: wysiwyg_editor::WysiwygEditor,
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
        let screen_images = screen_images.into_iter().collect();
        Self {
            project_id,
            wysiwyg_editor: wysiwyg_editor::WysiwygEditor::new(project_id, screen_images),
        }
    }
}
