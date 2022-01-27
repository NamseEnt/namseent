use crate::app::{
    sequence_list::{
        events::SequenceListEvent,
        types::{SequenceLoadState, SequenceLoadStateDetail},
        SequenceList,
    },
    types::Sequence,
};
use namui::{Namui, NamuiImpl};
use std::{sync::Arc, time::Duration};
use wasm_bindgen_futures::spawn_local;

impl SequenceList {
    pub fn load_sequence(&mut self, path: &String) {
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
                    namui::event::send(SequenceListEvent::SequenceLoadStateUpdateEvent {
                        path,
                        state: Some(SequenceLoadState {
                            started_at,
                            detail: SequenceLoadStateDetail::Failed { error },
                        }),
                    });
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
                                            sequence: Arc::new(sequence),
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
        })
    }
}
