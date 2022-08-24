mod components;
mod render;
mod update;

use crate::{
    storage::{EditorHistorySystem, SendableHistorySystem},
    sync::Syncer,
};
use namui::prelude::*;
use namui_prebuilt::*;
pub use render::Props;
use std::{collections::HashMap, sync::Arc};

pub struct LoadedSequenceEditorPage {
    project_id: String,
    sequence_id: String,
    cut_list_view: list_view::ListView,
    line_text_inputs: HashMap<String, text_input::TextInput>,
    syncer: Arc<Syncer>,
    editor_history_system: EditorHistorySystem,
}

enum Event {
    AddCutClicked,
    EditorHistorySystemUpdated,
    Error(String),
}

impl LoadedSequenceEditorPage {
    pub fn new(
        project_id: String,
        sequence_id: String,
        history_system: SendableHistorySystem,
        server_state_vector: Box<[u8]>,
        e_tag: String,
    ) -> Self {
        let syncer = Arc::new(Syncer::new(
            sequence_id.clone(),
            history_system.lock().unwrap().state_vector().into(),
            server_state_vector,
            e_tag,
        ));
        let editor_history_system = EditorHistorySystem::new(
            history_system.clone(),
            Box::new({
                let syncer = syncer.clone();
                move |history_system| {
                    syncer.on_client_updated(history_system);
                    namui::event::send(Event::EditorHistorySystemUpdated);
                }
            }),
        );

        let line_text_inputs = {
            let mut line_text_inputs = HashMap::new();
            let state = editor_history_system.get_state();
            state.sequence.cuts.iter().for_each(|cut| {
                line_text_inputs.insert(cut.id().to_string(), text_input::TextInput::new());
            });
            line_text_inputs
        };
        Self {
            project_id,
            sequence_id,
            cut_list_view: list_view::ListView::new(),
            line_text_inputs,
            syncer,
            editor_history_system,
        }
    }
}
