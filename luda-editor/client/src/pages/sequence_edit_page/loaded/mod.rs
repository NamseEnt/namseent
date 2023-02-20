mod components;
mod render;
mod update;

use crate::components::{sync::Syncer, *};
use components::*;
use namui::prelude::*;
use namui_prebuilt::*;
pub use render::Props;
use rpc::data::*;
use std::sync::Arc;

pub struct LoadedSequenceEditorPage {
    project_shared_data: ProjectSharedData,
    project_shared_data_syncer: Arc<Syncer<ProjectSharedData>>,
    sequence: Sequence,
    cut_list_view: components::cut_list_view::CutListView,
    cut_editor: components::cut_editor::CutEditor,
    sequence_syncer: Arc<Syncer<Sequence>>,
    context_menu: Option<context_menu::ContextMenu>,
    patch_stack: Vec<rpc::json_patch::RevertablePatch>,
    undo_stack: Vec<rpc::json_patch::RevertablePatch>,
    cut_clipboard: Option<Cut>,
    focused_component: Option<FocusableComponent>,
    selected_cut_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusableComponent {
    CutListView,
    CutEditor,
}

enum InternalEvent {
    Error(String),
    ListViewContextMenuAddCutClicked,
}

#[derive(Clone, Copy)]
enum AddCutPosition {
    Before { cut_id: Uuid },
    After { cut_id: Uuid },
}

impl LoadedSequenceEditorPage {
    pub fn new(project_shared_data: ProjectSharedData, sequence: Sequence) -> Self {
        let sequence_syncer = new_sequence_syncer(sequence.clone());
        let project_shared_data_syncer =
            new_project_shared_data_syncer_syncer(project_shared_data.clone());

        Self {
            cut_list_view: components::cut_list_view::CutListView::new(),
            cut_editor: components::cut_editor::CutEditor::new(),
            sequence_syncer,
            project_shared_data_syncer,
            project_shared_data,
            sequence,
            context_menu: None,
            patch_stack: Vec::new(),
            undo_stack: Vec::new(),
            cut_clipboard: None,
            focused_component: None,
            selected_cut_id: None,
        }
    }
    fn project_id(&self) -> Uuid {
        self.project_shared_data.id()
    }

    fn cut(&self, cut_id: Uuid) -> Option<&Cut> {
        self.sequence.cuts.iter().find(|cut| cut.id() == cut_id)
    }
}

fn new_sequence_syncer(sequence: Sequence) -> Arc<Syncer<Sequence>> {
    let sequence_id = sequence.id();
    Arc::new(Syncer::new(
        sequence,
        move |patch| {
            Box::pin(async move {
                let response = crate::RPC
                    .update_server_sequence(rpc::update_server_sequence::Request {
                        sequence_id,
                        patch,
                    })
                    .await;
                match response {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error.into()),
                }
            })
        },
        move |sequence_json| {
            Box::pin(async move {
                let response = crate::RPC
                    .update_client_sequence(rpc::update_client_sequence::Request {
                        sequence_id,
                        sequence_json,
                    })
                    .await;
                match response {
                    Ok(response) => Ok(response.patch),
                    Err(error) => Err(error.into()),
                }
            })
        },
    ))
}

fn new_project_shared_data_syncer_syncer(
    project_shared_data: ProjectSharedData,
) -> Arc<Syncer<ProjectSharedData>> {
    let project_id = project_shared_data.id();
    Arc::new(Syncer::new(
        project_shared_data,
        move |patch| {
            Box::pin(async move {
                let response = crate::RPC
                    .update_server_project_shared_data(
                        rpc::update_server_project_shared_data::Request { project_id, patch },
                    )
                    .await;
                match response {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error.into()),
                }
            })
        },
        move |project_shared_data_json| {
            Box::pin(async move {
                let response = crate::RPC
                    .update_client_project_shared_data(
                        rpc::update_client_project_shared_data::Request {
                            project_id,
                            project_shared_data_json,
                        },
                    )
                    .await;
                match response {
                    Ok(response) => Ok(response.patch),
                    Err(error) => Err(error.into()),
                }
            })
        },
    ))
}
