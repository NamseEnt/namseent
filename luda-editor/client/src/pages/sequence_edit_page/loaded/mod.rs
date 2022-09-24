mod components;
mod render;
mod update;

use crate::sync::Syncer;
use components::*;
use namui::prelude::*;
use namui_prebuilt::*;
pub use render::Props;
use rpc::data::*;
use std::{collections::HashMap, sync::Arc};

pub struct LoadedSequenceEditorPage {
    project_id: namui::Uuid,
    #[allow(dead_code)]
    sequence_id: namui::Uuid,
    cut_list_view: list_view::ListView,
    line_text_inputs: HashMap<Uuid, text_input::TextInput>,
    sequence_syncer: Arc<Syncer<Sequence>>,
    project_shared_data_syncer: Arc<Syncer<ProjectSharedData>>,
    character_edit_modal: Option<character_edit_modal::CharacterEditModal>,
    image_select_modal: Option<image_select_modal::ImageSelectModal>,
    project_shared_data: ProjectSharedData,
    sequence: Sequence,
}

enum Event {
    AddCutClicked,
    #[allow(dead_code)]
    Error(String),
    CharacterCellClicked {
        cut_id: namui::Uuid,
    },
    ScreenEditorCellClicked {
        index: usize,
        cut_id: Uuid,
    },
    ScreenEditorConfirmClicked {
        index: usize,
        cut_id: Uuid,
        image_id: Uuid,
    },
}

impl LoadedSequenceEditorPage {
    pub fn new(
        project_id: namui::Uuid,
        sequence_id: namui::Uuid,
        project_shared_data: ProjectSharedData,
        sequence: Sequence,
    ) -> Self {
        let sequence_syncer = new_sequence_syncer(sequence.clone(), sequence_id.clone());
        let project_shared_data_syncer =
            new_project_shared_data_syncer_syncer(project_shared_data.clone(), project_id.clone());

        let line_text_inputs = {
            let mut line_text_inputs = HashMap::new();
            sequence.cuts.iter().for_each(|cut| {
                line_text_inputs.insert(cut.id(), text_input::TextInput::new());
            });
            line_text_inputs
        };
        Self {
            project_id: project_id.clone(),
            sequence_id,
            cut_list_view: list_view::ListView::new(),
            line_text_inputs,
            sequence_syncer,
            project_shared_data_syncer,
            character_edit_modal: None,
            image_select_modal: None,
            project_shared_data,
            sequence,
        }
    }
}

fn new_sequence_syncer(sequence: Sequence, sequence_id: namui::Uuid) -> Arc<Syncer<Sequence>> {
    Arc::new(Syncer::new(
        sequence,
        {
            let sequence_id = sequence_id.clone();
            move |patch| {
                let sequence_id = sequence_id.clone();
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
            }
        },
        {
            let sequence_id = sequence_id.clone();
            move |sequence_json| {
                let sequence_id = sequence_id.clone();
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
            }
        },
    ))
}

fn new_project_shared_data_syncer_syncer(
    project_shared_data: ProjectSharedData,
    project_id: namui::Uuid,
) -> Arc<Syncer<ProjectSharedData>> {
    Arc::new(Syncer::new(
        project_shared_data,
        {
            let project_id = project_id.clone();
            move |patch| {
                let project_id = project_id.clone();
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
            }
        },
        {
            let project_id = project_id.clone();
            move |project_shared_data_json| {
                let project_id = project_id.clone();
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
            }
        },
    ))
}
