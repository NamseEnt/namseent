mod loaded;

use futures::try_join;
use loaded::LoadedSequenceEditorPage;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::{Memo, ProjectSharedData, Sequence};
use std::collections::HashMap;

pub enum SequenceEditPage {
    Loading { error: Option<String> },
    Loaded(LoadedSequenceEditorPage),
}

enum Event {
    ErrorOnLoading(String),
    DataLoaded {
        sequence: Sequence,
        project_shared_data: ProjectSharedData,
        cut_id_memo_map: HashMap<Uuid, Vec<Memo>>,
        user_id: Uuid,
    },
}
pub struct Props {
    pub wh: Wh<Px>,
}

impl SequenceEditPage {
    pub fn new(sequence_id: namui::Uuid) -> Self {
        load_data(sequence_id);
        Self::Loading { error: None }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::DataLoaded {
                project_shared_data,
                sequence,
                cut_id_memo_map,
                user_id,
            } => match self {
                SequenceEditPage::Loading { .. } => {
                    *self = SequenceEditPage::Loaded(LoadedSequenceEditorPage::new(
                        project_shared_data.clone(),
                        sequence.clone(),
                        cut_id_memo_map.clone(),
                        *user_id,
                    ));
                }
                SequenceEditPage::Loaded(_) => unreachable!(),
            },
            Event::ErrorOnLoading(error) => match self {
                SequenceEditPage::Loading {
                    error: page_error, ..
                } => {
                    *page_error = Some(error.clone());
                }
                SequenceEditPage::Loaded(_) => unreachable!(),
            },
        });
        match self {
            SequenceEditPage::Loading { error: _ } => {}
            SequenceEditPage::Loaded(loaded) => loaded.update(event),
        }
    }
    pub fn render(&self, props: Props) -> RenderingTree {
        match self {
            SequenceEditPage::Loading { error, .. } => match error {
                Some(error) => typography::body::center(props.wh, error, Color::RED),
                None => typography::body::center(props.wh, "loading...", Color::WHITE),
            },
            SequenceEditPage::Loaded(loaded_sequence_editor_page) => {
                loaded_sequence_editor_page.render(loaded::Props { wh: props.wh })
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SequenceLocalCache {
    update_v2: Vec<u8>,
    e_tag: String,
    server_state_vector: Vec<u8>,
}

fn load_data(sequence_id: namui::Uuid) {
    async fn load_sequence_and_project_shared_data(
        sequence_id: namui::Uuid,
    ) -> Result<(Sequence, ProjectSharedData), Box<dyn std::error::Error>> {
        let response = crate::RPC
            .get_sequence_and_project_shared_data(
                rpc::get_sequence_and_project_shared_data::Request { sequence_id },
            )
            .await?;
        let sequence = serde_json::from_str(&response.sequence_json)?;
        let project_shared_data = serde_json::from_str(&response.project_shared_data_json)?;
        Ok((sequence, project_shared_data))
    }
    async fn load_memos(
        sequence_id: namui::Uuid,
    ) -> Result<HashMap<Uuid, Vec<Memo>>, Box<dyn std::error::Error>> {
        let response = crate::RPC
            .list_sequence_memos(rpc::list_sequence_memos::Request { sequence_id })
            .await?;
        let cut_id_memo_map =
            response
                .memos
                .into_iter()
                .fold(HashMap::<Uuid, Vec<Memo>>::new(), |mut acc, memo| {
                    match acc.get_mut(&memo.cut_id) {
                        Some(memos) => memos.push(memo),
                        None => {
                            acc.insert(memo.cut_id, vec![memo]);
                        }
                    };
                    acc
                });
        Ok(cut_id_memo_map)
    }
    async fn get_user_id() -> Result<Uuid, Box<dyn std::error::Error>> {
        let response = crate::RPC.get_user_id(rpc::get_user_id::Request {}).await?;
        Ok(response.user_id)
    }
    spawn_local(async move {
        let result = try_join!(
            load_sequence_and_project_shared_data(sequence_id),
            load_memos(sequence_id),
            get_user_id(),
        );
        match result {
            Ok(((sequence, project_shared_data), cut_id_memo_map, user_id)) => {
                namui::event::send(Event::DataLoaded {
                    sequence,
                    project_shared_data,
                    cut_id_memo_map,
                    user_id,
                });
            }
            Err(error) => {
                namui::event::send(Event::ErrorOnLoading(error.to_string()));
            }
        }
    })
}
