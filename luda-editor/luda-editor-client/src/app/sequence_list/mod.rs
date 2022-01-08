use self::{
    events::SequenceListEvent,
    types::{SequenceLoadState, SequenceLoadStateDetail, SequenceLoadStateMap},
};
use crate::app::types::Sequence;
use luda_editor_rpc::Socket;
use namui::{render, Entity, Namui, NamuiEvent, NamuiImpl, Wh, XywhRect};
use std::{collections::HashMap, time::Duration};
use wasm_bindgen_futures::spawn_local;
mod button_background;
mod button_text;
mod events;
mod load_button;
mod open_button;
mod types;

const BUTTON_HEIGHT: f32 = 36.0;

pub struct SequenceList {
    sequence_load_state_map: SequenceLoadStateMap,
    socket: Socket,
    xywh: XywhRect<f32>,
}

impl SequenceList {
    pub fn new(socket: Socket, xywh: XywhRect<f32>) -> Self {
        Self {
            sequence_load_state_map: HashMap::new(),
            socket,
            xywh,
        }
    }
}

impl Entity for SequenceList {
    type Props = ();

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
                    namui::event::send(Box::new(SequenceListEvent::SequenceLoadStateUpdateEvent {
                        path: path.clone(),
                        state: Some(SequenceLoadState {
                            started_at,
                            detail: SequenceLoadStateDetail::Loading,
                        }),
                    }));
                    spawn_local({
                        let path = path.clone();
                        let socket = self.socket.clone();
                        async move {
                            fn handle_error(path: String, started_at: Duration, error: String) {
                                namui::log(format!("error on read_file: {:?}", error));
                                namui::event::send(Box::new(
                                    SequenceListEvent::SequenceLoadStateUpdateEvent {
                                        path,
                                        state: Some(SequenceLoadState {
                                            started_at,
                                            detail: SequenceLoadStateDetail::Failed { error },
                                        }),
                                    },
                                ));
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
                                        Ok(sequence) => namui::event::send(Box::new(
                                            SequenceListEvent::SequenceLoadStateUpdateEvent {
                                                path: path.clone(),
                                                state: Some(SequenceLoadState {
                                                    started_at,
                                                    detail: SequenceLoadStateDetail::Loaded {
                                                        sequence,
                                                    },
                                                }),
                                            },
                                        )),
                                        Err(error) => handle_error(path.clone(), started_at, error),
                                    }
                                }
                                Err(error) => handle_error(path.clone(), started_at, error),
                            }
                        }
                    });
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                namui::NamuiEvent::ScreenResize(wh) => {
                    self.xywh = namui::XywhRect {
                        x: 0.0,
                        y: 0.0,
                        width: wh.width as f32,
                        height: wh.height as f32,
                    };
                }
                _ => (),
            }
        }
    }

    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        let button_wh = Wh {
            width: self.xywh.width,
            height: BUTTON_HEIGHT,
        };
        let test_path: String = "sequence/testSequence.json".to_string();
        match self.sequence_load_state_map.get(&test_path) {
            Some(load_state) => match &load_state.detail {
                SequenceLoadStateDetail::Loading => render![
                    self.render_button_background(button_wh),
                    self.render_button_text(button_wh, "Loading...".to_string())
                ],
                SequenceLoadStateDetail::Loaded { sequence } => {
                    self.render_open_button(button_wh, &test_path, &sequence)
                }
                SequenceLoadStateDetail::Failed { error } => render![
                    self.render_button_background(button_wh),
                    self.render_button_text(button_wh, format!("Error: {}", error))
                ],
            },
            None => self.render_load_button(button_wh, &test_path),
        }
    }
}
