use crate::app::sequence_list::{
    events::SequenceListEvent,
    types::{self, SequenceTitlesLoadState},
    SequenceList,
};
use luda_editor_rpc::DirentFileType;
use namui::{Namui, NamuiImpl};
use wasm_bindgen_futures::spawn_local;

impl SequenceList {
    pub fn load_sequence_titles(&mut self) {
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
                                DirentFileType::File => match dirent.name.ends_with(".json") {
                                    true => Some(dirent.name.clone()),
                                    false => None,
                                },
                                _ => None,
                            })
                            .collect();

                        namui::event::send(SequenceListEvent::SequenceTitlesLoadStateUpdateEvent {
                            state: SequenceTitlesLoadState {
                                started_at,
                                detail: types::SequenceTitlesLoadStateDetail::Loaded { titles },
                            },
                        })
                    }
                    Err(error) => {
                        namui::log(format!("error on read_dir: {:?}", error));
                        namui::event::send(SequenceListEvent::SequenceTitlesLoadStateUpdateEvent {
                            state: SequenceTitlesLoadState {
                                started_at,
                                detail: types::SequenceTitlesLoadStateDetail::Failed { error },
                            },
                        });
                    }
                }
            }
        });
    }
}
