mod render;
mod update;

use super::*;
use crate::storage::{Cut, EditorHistorySystem, ImageClipAddress, Storage};
use namui::prelude::*;
use namui_prebuilt::list_view;

#[derive(Debug, Clone)]
pub struct PropertyEditor {
    editor_history_system: EditorHistorySystem,
    content: PropertyContent,
    storage: Storage,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub cut: &'a Cut,
    pub selected_layer_index: Option<usize>,
    pub selected_sequence_id: &'a str,
    pub selected_cut_id: &'a str,
}

enum Event {
    LayerListPlusButtonClicked {
        image_clip_address: ImageClipAddress,
    },
    ChangeImage {
        image_clip_address: ImageClipAddress,
        image: String,
        layer_index: usize,
    },
}

#[derive(Debug, Clone)]
enum PropertyContent {
    Nothing,
    ImageClip {
        image_clip_address: ImageClipAddress,
        layer_list_view: list_view::ListView,
    },
    ImageLayer {
        image_clip_address: ImageClipAddress,
        image_browser: image_browser::ImageBrowser,
        layer_list_view: list_view::ListView,
        layer_index: usize,
    },
}

impl std::default::Default for PropertyContent {
    fn default() -> Self {
        PropertyContent::Nothing
    }
}

impl PropertyEditor {
    pub fn new(editor_history_system: EditorHistorySystem, storage: Storage) -> Self {
        Self {
            editor_history_system,
            content: PropertyContent::Nothing,
            storage,
        }
    }
}
