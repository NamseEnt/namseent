use crate::app::{
    sequence_list::{events::SequenceListEvent, types::*, SequenceList},
    types::*,
};
use luda_editor_rpc::Socket;
use std::{collections::BTreeMap, sync::Arc};
use wasm_bindgen_futures::spawn_local;

impl SequenceList {
    pub fn load_local_sequences(&mut self) {
        let started_at = namui::now();
        namui::event::send(SequenceListEvent::SequencesSyncStateUpdateEvent {
            state: SequenceSyncState {
                started_at,
                detail: SequencesSyncStateDetail::Loading,
            },
        });

        spawn_local({
            let socket = self.socket.clone();
            async move {
                let result = get_sequences_with_title(&socket).await;
                match result {
                    Ok(title_sequence_map) => {
                        namui::event::send(SequenceListEvent::SequencesSyncStateUpdateEvent {
                            state: SequenceSyncState {
                                started_at,
                                detail: SequencesSyncStateDetail::Loaded { title_sequence_map },
                            },
                        })
                    }
                    Err(error) => {
                        namui::event::send(SequenceListEvent::SequencesSyncStateUpdateEvent {
                            state: SequenceSyncState {
                                started_at,
                                detail: SequencesSyncStateDetail::Failed { error },
                            },
                        });
                    }
                }
            }
        });
    }
}

pub async fn get_sequences_with_title(
    socket: &Socket,
) -> Result<BTreeMap<String, Arc<Sequence>>, String> {
    socket
        .get_sequences(luda_editor_rpc::get_sequences::Request {})
        .await
        .and_then(|response| {
            let mut sequences = vec![];

            for (title, sequence_json) in response.title_sequence_json_tuples {
                match Sequence::try_from(sequence_json.as_ref()) {
                    Ok(sequence) => {
                        sequences.push((title, Arc::new(sequence)));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            Ok(sequences.into_iter().collect())
        })
}
