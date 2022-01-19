use self::{
    events::SequenceListEvent,
    types::{
        SequenceLoadState, SequenceLoadStateDetail, SequenceLoadStateMap, SequenceTitlesLoadState,
    },
};
use crate::app::types::Sequence;
use luda_editor_rpc::{DirentFileType, Socket};
use namui::{render, Entity, Namui, NamuiImpl, Wh};
use std::{collections::HashMap, time::Duration};
use wasm_bindgen_futures::spawn_local;
mod button_text;
mod events;
mod list;
mod list_item;
mod load_button;
mod open_button;
mod reload_titles_button;
mod rounded_rectangle;
mod types;

const LIST_WIDTH: f32 = 800.0;
const BUTTON_HEIGHT: f32 = 36.0;
const RECT_RADIUS: f32 = 4.0;
const SPACING: f32 = 4.0;
const MARGIN: f32 = 4.0;

pub struct SequenceListProps {
    pub wh: Wh<f32>,
}

pub struct SequenceList {
    sequence_load_state_map: SequenceLoadStateMap,
    sequence_titles_load_state: Option<SequenceTitlesLoadState>,
    socket: Socket,
}

impl SequenceList {
    pub fn new(socket: Socket) -> Self {
        Self {
            sequence_load_state_map: HashMap::new(),
            sequence_titles_load_state: None,
            socket,
        }
    }
}

impl Entity for SequenceList {
    type Props = SequenceListProps;

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequenceListEvent>() {
            match event {
                SequenceListEvent::SequenceLoadStateUpdateEvent { path, state } => match state {
                    Some(state) => {
                        if let Some(old_state) = self.sequence_load_state_map.get(path) {
                            if old_state.started_at > state.started_at {
                                return;
                            }
                        }

                        self.sequence_load_state_map
                            .insert(path.clone(), (*state).clone());
                    }
                    None => {
                        self.sequence_load_state_map.remove(path);
                    }
                },
                SequenceListEvent::SequenceLoadEvent { path } => {
                    let started_at = Namui::now();
                    namui::event::send(SequenceListEvent::SequenceLoadStateUpdateEvent {
                        path: path.clone(),
                        state: Some(SequenceLoadState {
                            started_at,
                            detail: SequenceLoadStateDetail::Loading,
                        }),
                    });
                    spawn_local({
                        let path = path.clone();
                        let socket = self.socket.clone();
                        async move {
                            fn handle_error(path: String, started_at: Duration, error: String) {
                                namui::log(format!("error on read_file: {:?}", error));
                                namui::event::send(
                                    SequenceListEvent::SequenceLoadStateUpdateEvent {
                                        path,
                                        state: Some(SequenceLoadState {
                                            started_at,
                                            detail: SequenceLoadStateDetail::Failed { error },
                                        }),
                                    },
                                );
                            }
                            let result = socket
                                .read_file(luda_editor_rpc::read_file::Request {
                                    dest_path: path.clone(),
                                })
                                .await;
                            match result {
                                Ok(response) => {
                                    let file = response.file;
                                    match Sequence::try_from(file) {
                                        Ok(sequence) => namui::event::send(
                                            SequenceListEvent::SequenceLoadStateUpdateEvent {
                                                path: path.clone(),
                                                state: Some(SequenceLoadState {
                                                    started_at,
                                                    detail: SequenceLoadStateDetail::Loaded {
                                                        sequence,
                                                    },
                                                }),
                                            },
                                        ),
                                        Err(error) => handle_error(path.clone(), started_at, error),
                                    }
                                }
                                Err(error) => handle_error(path.clone(), started_at, error),
                            }
                        }
                    });
                }
                SequenceListEvent::SequenceTitlesLoadStateUpdateEvent { state } => {
                    if let Some(old_state) = &self.sequence_titles_load_state {
                        if old_state.started_at > state.started_at {
                            return;
                        }
                    }

                    self.sequence_titles_load_state = Some(state.clone());
                }
                SequenceListEvent::SequenceTitlesLoadEvent => {
                    let started_at = Namui::now();
                    namui::event::send(SequenceListEvent::SequenceTitlesLoadStateUpdateEvent {
                        state: SequenceTitlesLoadState {
                            started_at,
                            detail: types::SequenceTitlesLoadStateDetail::Loading,
                        },
                    });
                    spawn_local({
                        let socket = self.socket.clone();
                        async move {
                            let result = socket
                                .read_dir(luda_editor_rpc::read_dir::Request {
                                    dest_path: "sequence".to_string(),
                                })
                                .await;
                            match result {
                                Ok(response) => {
                                    let titles: Vec<String> = response
                                        .directory_entries
                                        .iter()
                                        .filter_map(|dirent| match dirent.file_type {
                                            DirentFileType::File => {
                                                match dirent.name.ends_with(".json") {
                                                    true => Some(dirent.name.clone()),
                                                    false => None,
                                                }
                                            }
                                            _ => None,
                                        })
                                        .collect();

                                    namui::event::send(
                                        SequenceListEvent::SequenceTitlesLoadStateUpdateEvent {
                                            state: SequenceTitlesLoadState {
                                                started_at,
                                                detail:
                                                    types::SequenceTitlesLoadStateDetail::Loaded {
                                                        titles,
                                                    },
                                            },
                                        },
                                    )
                                }
                                Err(error) => {
                                    namui::log(format!("error on read_dir: {:?}", error));
                                    namui::event::send(
                                        SequenceListEvent::SequenceTitlesLoadStateUpdateEvent {
                                            state: SequenceTitlesLoadState {
                                                started_at,
                                                detail:
                                                    types::SequenceTitlesLoadStateDetail::Failed {
                                                        error,
                                                    },
                                            },
                                        },
                                    );
                                }
                            }
                        }
                    });
                }
            }
        }
    }

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let list_wh = Wh {
            width: LIST_WIDTH,
            height: props.wh.height - 2.0 * MARGIN - SPACING - BUTTON_HEIGHT,
        };
        render![
            namui::translate(
                MARGIN,
                MARGIN,
                self.render_reload_titles_button(Wh {
                    width: LIST_WIDTH,
                    height: BUTTON_HEIGHT
                })
            ),
            namui::translate(
                MARGIN,
                MARGIN + SPACING + BUTTON_HEIGHT,
                self.render_list(list_wh)
            ),
        ]
    }
}
