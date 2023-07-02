mod components;
mod render;
mod update;

use super::{
    cg_files_atom::{CgFilesAtom, CG_FILES_ATOM},
    sequence::SequenceWrapped,
    sequence_atom::SEQUENCE_ATOM,
};
use crate::components::*;
use components::*;
use namui::prelude::*;
pub use render::Props;
use rpc::data::*;
use std::collections::HashMap;

pub struct LoadedSequenceEditorPage {
    project_shared_data: ProjectSharedData,
    cut_list_view: components::cut_list_view::CutListView,
    cut_editor: components::cut_editor::CutEditor,
    context_menu: Option<context_menu::ContextMenu>,
    focused_component: Option<FocusableComponent>,
    selected_cut_id: Option<Uuid>,
    character_editor: Option<components::character_editor::CharacterEditor>,
    memo_list_view: components::memo_list_view::MemoListView,
    memo_editor: Option<components::memo_editor::MemoEditor>,
    cut_id_memo_map: HashMap<Uuid, Vec<Memo>>,
    user_id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusableComponent {
    CutListView,
    CutEditor,
}

enum InternalEvent {
    Error(String),
}

impl LoadedSequenceEditorPage {
    pub fn new(
        project_shared_data: ProjectSharedData,
        sequence: Sequence,
        cut_id_memo_map: HashMap<Uuid, Vec<Memo>>,
        user_id: Uuid,
        cg_files: Vec<CgFile>,
    ) -> Self {
        SEQUENCE_ATOM.set(SequenceWrapped::new(sequence));
        CG_FILES_ATOM.set(CgFilesAtom::new(cg_files));
        Self {
            cut_list_view: components::cut_list_view::CutListView::new(),
            cut_editor: components::cut_editor::CutEditor::new(),
            project_shared_data,
            context_menu: None,
            focused_component: None,
            selected_cut_id: None,
            character_editor: None,
            memo_list_view: components::memo_list_view::MemoListView::new(),
            memo_editor: None,
            cut_id_memo_map,
            user_id,
        }
    }
    fn project_id(&self) -> Uuid {
        self.project_shared_data.id()
    }
}
