mod render;
mod update;

use crate::storage::*;
use namui::prelude::*;

pub struct WysiwygEditor {
    editor_history_system: EditorHistorySystem,
    dragging: Option<Dragging>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub image_clip: &'a ImageClip,
    pub selected_layer_index: Option<usize>,
    pub image_clip_address: &'a ImageClipAddress,
}

enum Event {
    Resize {
        circumscribed: Circumscribed,
        image_clip_address: ImageClipAddress,
        layer_index: usize,
    },
}

enum Dragging {
    Resizer {
        context: render::resizer::ResizerDraggingContext,
    },
    Cropper,
}

impl WysiwygEditor {
    pub fn new(editor_history_system: EditorHistorySystem) -> Self {
        Self {
            editor_history_system,
            dragging: None,
        }
    }
}
