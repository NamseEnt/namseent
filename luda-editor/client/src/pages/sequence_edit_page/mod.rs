mod loaded;

use crate::storage::{HistorySystem, SendableHistorySystem};
use loaded::LoadedSequenceEditorPage;
use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::{Arc, Mutex};

pub enum SequenceEditPage {
    Loading {
        project_id: String,
        sequence_id: String,
        error: Option<String>,
    },
    Loaded(LoadedSequenceEditorPage),
}

enum Event {
    HistorySystemLoaded {
        history_system: SendableHistorySystem,
        server_state_vector: Box<[u8]>,
        e_tag: String,
    },
    ErrorOnLoading(String),
}
pub struct Props {
    pub wh: Wh<Px>,
}

impl SequenceEditPage {
    pub fn new(project_id: String, sequence_id: String, sequence_name: String) -> Self {
        load_sequence(sequence_id.clone(), sequence_name);
        Self::Loading {
            project_id,
            sequence_id,
            error: None,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::HistorySystemLoaded {
                    history_system,
                    server_state_vector,
                    e_tag,
                } => match self {
                    SequenceEditPage::Loading {
                        project_id,
                        sequence_id,
                        ..
                    } => {
                        *self = SequenceEditPage::Loaded(LoadedSequenceEditorPage::new(
                            project_id.clone(),
                            sequence_id.clone(),
                            history_system.clone(),
                            server_state_vector.clone(),
                            e_tag.clone(),
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
            SequenceEditPage::Loading { .. } => {
                typography::body::center(props.wh, "loading...", Color::WHITE)
            }
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

fn load_sequence(sequence_id: String, sequence_name: String) {
    spawn_local(async move {
        let cache = namui::cache::get_serde::<SequenceLocalCache>("SequenceLocalCache").await;
        if let Err(error) = cache {
            return namui::event::send(Event::ErrorOnLoading(error.to_string()));
        }
        let cache = cache.unwrap();

        let (client_state_vector, history_system) = match &cache {
            Some(cache) => {
                let history_system = HistorySystem::decode(&cache.update_v2);
                (history_system.state_vector(), Some(history_system))
            }
            None => (HistorySystem::default_state_vector(), None),
        };

        let (history_system, server_state_vector, e_tag) = match crate::RPC
            .update_client_sequence(rpc::update_client_sequence::Request {
                sequence_id: sequence_id.clone(),
                client_state_vector_base64: rpc::base64::encode(client_state_vector),
                e_tag: cache.as_ref().map(|cache| cache.e_tag.clone()),
            })
            .await
        {
            Ok(response) => match response {
                rpc::update_client_sequence::Response::Modified {
                    server_state_vector_base64,
                    yrs_update_v2_for_client_base64,
                    e_tag,
                } => {
                    let update_for_client =
                        rpc::base64::decode(yrs_update_v2_for_client_base64).unwrap();

                    let history_system = match history_system {
                        Some(mut history_system) => {
                            history_system.merge(update_for_client);
                            history_system
                        }
                        None => {
                            let history_system = HistorySystem::decode(update_for_client);
                            history_system
                        }
                    };

                    (
                        history_system,
                        rpc::base64::decode(server_state_vector_base64).unwrap(),
                        e_tag,
                    )
                }
                rpc::update_client_sequence::Response::NotModified => {
                    let cached = cache.unwrap();
                    (
                        history_system.expect("history_system must be Some"),
                        cached.server_state_vector,
                        cached.e_tag,
                    )
                }
            },
            Err(error) => {
                if let rpc::update_client_sequence::Error::ServerSequenceNotExists = error {
                    let mut history_system = HistorySystem::new(crate::storage::SystemTree::new(
                        sequence_id.clone(),
                        sequence_name.clone(),
                    ));
                    match crate::RPC
                        .update_server_sequence(rpc::update_server_sequence::Request {
                            sequence_id: sequence_id.clone(),
                            client_state_vector_base64: rpc::base64::encode(
                                history_system.state_vector(),
                            ),
                            yrs_update_v2_for_server_base64: rpc::base64::encode(
                                history_system.encode(),
                            ),
                        })
                        .await
                    {
                        Ok(response) => {
                            history_system.merge(
                                rpc::base64::decode(response.yrs_update_v2_for_client_base64)
                                    .unwrap(),
                            );

                            (
                                history_system,
                                rpc::base64::decode(response.server_state_vector_base64).unwrap(),
                                response.e_tag,
                            )
                        }
                        Err(error) => {
                            return namui::event::send(Event::ErrorOnLoading(error.to_string()));
                        }
                    }
                } else {
                    return namui::event::send(Event::ErrorOnLoading(error.to_string()));
                }
            }
        };

        namui::event::send(Event::HistorySystemLoaded {
            history_system: Arc::new(Mutex::new(history_system)),
            server_state_vector: server_state_vector.into_boxed_slice(),
            e_tag,
        });
    })
}
