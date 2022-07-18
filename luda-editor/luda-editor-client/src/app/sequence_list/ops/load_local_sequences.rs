use crate::app::{
    sequence_list::{events::SequenceListEvent, types::*, SequenceList},
    storage::GithubStorage,
    types::*,
};
use futures::{future::join_all, TryFutureExt};
use linked_hash_map::LinkedHashMap;
use std::{fmt::Debug, sync::Arc};
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
            let storage = self.storage.clone();
            async move {
                let result = get_sequences_with_title(&storage).await;
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
    storage: &Arc<dyn GithubStorage>,
) -> Result<LinkedHashMap<String, Arc<Sequence>>, String> {
    let sequence_index = SequenceIndex::load(storage).await?;
    let sequence_download_futures = storage
        .get_sequence_list()
        .map_err(|error| format!("failed to load sequence list: {:#?}", error))
        .await?
        .into_iter()
        .map(|sequence_title| async move {
            (
                sequence_title.clone(),
                storage.get_sequence(sequence_title.as_str()).await,
            )
        });
    let title_sequence_download_result_pair_list = join_all(sequence_download_futures).await;
    throw_error_if_sequence_download_failed(&title_sequence_download_result_pair_list)?;
    let unsorted_title_sequence_map: LinkedHashMap<String, Arc<Sequence>> =
        title_sequence_download_result_pair_list
            .into_iter()
            .map(|(title, sequence_download_result)| {
                (title, Arc::new(sequence_download_result.unwrap()))
            })
            .collect();

    Ok(sequence_index.sort_title_sequence_map(&unsorted_title_sequence_map))
}

fn throw_error_if_sequence_download_failed<E>(
    title_sequence_download_result_pair_list: &Vec<(String, Result<Sequence, E>)>,
) -> Result<(), String>
where
    E: Debug + Sized,
{
    let sequence_download_error_list: Vec<_> = title_sequence_download_result_pair_list
        .iter()
        .filter_map(|(title, result)| {
            if let Err(error) = result {
                return Some((title, error));
            }
            None
        })
        .collect();

    if sequence_download_error_list.len() > 0 {
        return Err(format!(
            "failed to download some sequence: {:#?}",
            sequence_download_error_list
        ));
    }

    Ok(())
}
