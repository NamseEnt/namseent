mod components;
mod render;
mod update;

use crate::{components::sequence_player, sync::Syncer};
use components::*;
use namui::prelude::*;
use namui_prebuilt::*;
pub use render::Props;
use rpc::data::*;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

pub struct LoadedSequenceEditorPage {
    project_shared_data: ProjectSharedData,
    project_shared_data_syncer: Arc<Syncer<ProjectSharedData>>,
    sequence: Sequence,
    cut_list_view: list_view::ListView,
    line_text_inputs: HashMap<Uuid, text_input::TextInput>,
    sequence_syncer: Arc<Syncer<Sequence>>,
    character_edit_modal: Option<character_edit_modal::CharacterEditModal>,
    image_select_modal: Option<image_select_modal::ImageSelectModal>,
    recent_selected_image_ids: VecDeque<Uuid>,
    sequence_player: Option<sequence_player::SequencePlayer>,
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
        image_id: Option<Uuid>,
    },
    UpdateRecentSelectedImageIds {
        image_ids: VecDeque<Uuid>,
    },
    PreviewButtonClicked,
    ClosePlayer,
}

impl LoadedSequenceEditorPage {
    pub fn new(project_shared_data: ProjectSharedData, sequence: Sequence) -> Self {
        let sequence_syncer = new_sequence_syncer(sequence.clone());
        let project_shared_data_syncer =
            new_project_shared_data_syncer_syncer(project_shared_data.clone());

        let line_text_inputs = {
            let mut line_text_inputs = HashMap::new();
            sequence.cuts.iter().for_each(|cut| {
                line_text_inputs.insert(cut.id(), text_input::TextInput::new());
            });
            line_text_inputs
        };
        start_load_recent_selected_image_ids();
        Self {
            cut_list_view: list_view::ListView::new(),
            line_text_inputs,
            sequence_syncer,
            project_shared_data_syncer,
            character_edit_modal: None,
            image_select_modal: None,
            project_shared_data,
            sequence,
            recent_selected_image_ids: VecDeque::new(),
            sequence_player: None,
        }
    }
    fn project_id(&self) -> Uuid {
        self.project_shared_data.id()
    }
}

fn start_load_recent_selected_image_ids() {
    spawn_local(async move {
        let result = namui::cache::get_serde::<VecDeque<Uuid>>("recent_selected_image_ids").await;
        match result {
            Ok(image_ids) => {
                if let Some(image_ids) = image_ids {
                    namui::event::send(Event::UpdateRecentSelectedImageIds { image_ids });
                }
            }
            Err(error) => {
                namui::event::send(Event::Error(error.to_string()));
            }
        }
    })
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
