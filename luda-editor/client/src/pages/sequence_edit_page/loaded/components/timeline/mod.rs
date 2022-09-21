mod render;
mod resizable_clip;
mod update;

use namui::prelude::*;
use rpc::data::{Cut, EditorHistorySystem};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Timeline {
    selected_clip_ids: HashSet<String>,
    start_at: Time,
    time_per_px: TimePerPx,
    context_menu: Option<ContextMenu>,
    editor_history_system: EditorHistorySystem,
    selected_sequence_id: String,
    selected_cut_id: String,
    clip_sash_dragging: Option<ClipSashDragging>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub cut: &'a Cut,
}

pub enum Event {
    OpenContextMenu(ContextMenu),
    CloseContextMenu,
    NewImageClip,
    SelectImageClip {
        sequence_id: String,
        cut_id: String,
        image_clip_ids: HashSet<String>,
    },
    DeselectImageClip,
}

#[derive(Debug, Clone)]
pub enum ContextMenu {
    ImageClip { global_xy: Xy<Px> },
}

#[derive(Debug, Clone)]
struct ClipSashDragging {
    pub clip_id: String,
    pub start_global_mouse_x: Px,
    pub global_mouse_x: Px,
}

impl Timeline {
    pub fn new(
        editor_history_system: EditorHistorySystem,
        selected_sequence_id: String,
        selected_cut_id: String,
    ) -> Self {
        Self {
            selected_clip_ids: HashSet::new(),
            start_at: 0.sec(),
            time_per_px: 50.ms() / 1.px(),
            context_menu: None,
            editor_history_system,
            selected_sequence_id,
            selected_cut_id,
            clip_sash_dragging: None,
        }
    }
}
