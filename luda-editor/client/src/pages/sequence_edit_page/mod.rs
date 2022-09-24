mod loaded;

use loaded::LoadedSequenceEditorPage;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::{ProjectSharedData, Sequence};

pub enum SequenceEditPage {
    Loading {
        project_id: namui::Uuid,
        sequence_id: namui::Uuid,
        error: Option<String>,
    },
    Loaded(LoadedSequenceEditorPage),
}

enum Event {
    ErrorOnLoading(String),
    DataLoaded {
        sequence: Sequence,
        project_shared_data: ProjectSharedData,
    },
}
pub struct Props {
    pub wh: Wh<Px>,
}

impl SequenceEditPage {
    pub fn new(project_id: namui::Uuid, sequence_id: namui::Uuid) -> Self {
        load_data(sequence_id.clone());
        Self::Loading {
            project_id,
            sequence_id,
            error: None,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::DataLoaded {
                    project_shared_data,
                    sequence,
                } => match self {
                    SequenceEditPage::Loading {
                        project_id,
                        sequence_id,
                        ..
                    } => {
                        *self = SequenceEditPage::Loaded(LoadedSequenceEditorPage::new(
                            project_id.clone(),
                            sequence_id.clone(),
                            project_shared_data.clone(),
                            sequence.clone(),
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
            }
        }
        match self {
            SequenceEditPage::Loading {
                project_id: _,
                sequence_id: _,
                error: _,
            } => {}
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
    async fn inner(
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
    spawn_local(async move {
        let result = inner(sequence_id).await;
        match result {
            Ok(tuple) => {
                namui::event::send(Event::DataLoaded {
                    sequence: tuple.0,
                    project_shared_data: tuple.1,
                });
            }
            Err(error) => {
                namui::event::send(Event::ErrorOnLoading(error.to_string()));
            }
        }
    })
}
