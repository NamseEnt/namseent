mod render;
mod update;
use namui::prelude::*;
use namui_prebuilt::scroll_view::ScrollView;
use rpc::data::CgFile;

pub struct CharacterEditor {
    tooltip: Option<Tooltip>,
    scroll_view: ScrollView,
    edit_target: EditTarget,
    cg_file_load_state: CgFileLoadState,
    selected_cg_name: Option<String>,
}

#[derive(Clone, Copy)]
pub struct Props {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

pub enum Event {
    MouseDownOutsideCharacterEditor,
    OpenCharacterEditor { target: EditTarget },
}

enum InternalEvent {
    OpenTooltip { global_xy: Xy<Px>, text: String },
    CloseTooltip,
    CgChangeButtonClicked,
    CgFileLoadStateChanged(CgFileLoadState),
}

impl CharacterEditor {
    pub fn new(project_id: Uuid, edit_target: EditTarget) -> Self {
        let mut character_editor = Self {
            tooltip: None,
            scroll_view: ScrollView::new(),
            edit_target,
            cg_file_load_state: CgFileLoadState::Loading,
            selected_cg_name: None,
        };
        character_editor.start_load_cg_files(project_id);
        character_editor
    }

    fn start_load_cg_files(&mut self, project_id: Uuid) {
        self.cg_file_load_state = CgFileLoadState::Loading;
        spawn_local(async move {
            let response = crate::RPC
                .list_cg_files(rpc::list_cg_files::Request { project_id })
                .await;

            match response {
                Ok(response) => {
                    namui::event::send(InternalEvent::CgFileLoadStateChanged(
                        CgFileLoadState::Loaded(response.cg_files),
                    ));
                }
                Err(error) => {
                    namui::event::send(InternalEvent::CgFileLoadStateChanged(
                        CgFileLoadState::Failed {
                            error: error.to_string(),
                        },
                    ));
                }
            }
        });
    }
}

struct Tooltip {
    global_xy: Xy<Px>,
    text: String,
}

#[derive(Clone, Copy)]
pub enum EditTarget {
    NewCharacter,
    ExistingCharacter,
    ExistingCharacterPart,
}

#[derive(Clone)]
enum CgFileLoadState {
    Loading,
    Loaded(Vec<CgFile>),
    Failed { error: String },
}
