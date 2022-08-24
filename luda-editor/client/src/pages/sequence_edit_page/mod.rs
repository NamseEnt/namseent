mod components;
mod render;
mod update;

use crate::storage::{EditorHistorySystem, Storage};
use namui::prelude::*;
use namui_prebuilt::*;
pub use render::Props;
use std::collections::HashMap;

pub struct SequenceEditPage {
    editor_history_system: EditorHistorySystem,
    selected_sequence_id: String,
    cut_list_view: list_view::ListView,
    storage: Storage,
    line_text_inputs: HashMap<String, text_input::TextInput>,
}

enum Event {
    AddCutClicked,
}

impl SequenceEditPage {
    pub fn new(
        editor_history_system: EditorHistorySystem,
        selected_sequence_id: String,
        storage: Storage,
    ) -> Self {
        let mut line_text_inputs = HashMap::new();
        editor_history_system.with_sequence(&selected_sequence_id, |sequence| {
            for cut in sequence.cuts.iter() {
                line_text_inputs.insert(cut.id().to_string(), text_input::TextInput::new());
            }
        });
        Self {
            editor_history_system: editor_history_system.clone(),
            selected_sequence_id,
            cut_list_view: list_view::ListView::new(),
            storage,
            line_text_inputs,
        }
    }
}
