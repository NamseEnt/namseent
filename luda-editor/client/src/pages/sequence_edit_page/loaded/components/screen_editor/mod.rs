mod render;
mod update;

use super::wysiwyg_editor;
use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;
use std::sync::Arc;

pub struct ScreenEditor {
    wysiwyg_editor: wysiwyg_editor::WysiwygEditor,
    done: Arc<dyn Fn(ScreenImages)>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub project_shared_data: &'a ProjectSharedData,
    pub cut: &'a Cut,
}

impl ScreenEditor {
    pub fn new(
        project_id: Uuid,
        screen_images: ScreenImages,
        done: impl Fn(ScreenImages) + 'static,
    ) -> Self {
        Self {
            wysiwyg_editor: wysiwyg_editor::WysiwygEditor::new(project_id, screen_images),
            done: Arc::new(done),
        }
    }
}
