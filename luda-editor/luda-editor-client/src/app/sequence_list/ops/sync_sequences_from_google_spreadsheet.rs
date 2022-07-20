use super::get_sequences_with_title;
use crate::app::{
    editor::{JobExecute, SyncSubtitlesJob},
    sequence_list::{
        events::SequenceListEvent,
        types::{self, SequenceIndex, SequenceSyncState},
        SequenceList,
    },
    storage::GithubStorage,
    types::*,
};
use futures::future::join_all;
use linked_hash_map::LinkedHashMap;
use std::{fmt::Debug, sync::Arc};
use wasm_bindgen_futures::spawn_local;

impl SequenceList {
    pub fn sync_sequences_from_google_spreadsheet(&mut self) {
        let started_at = namui::now();
        namui::event::send(SequenceListEvent::SequencesSyncStateUpdateEvent {
            state: SequenceSyncState {
                started_at,
                detail: types::SequencesSyncStateDetail::Loading,
            },
        });

        spawn_local({
            let storage = self.storage.clone();
            async move {
                let result = sync_sequences_with_sheets(&storage).await;
                match result {
                    Ok(title_sequence_map) => {
                        namui::event::send(SequenceListEvent::SequencesSyncStateUpdateEvent {
                            state: SequenceSyncState {
                                started_at,
                                detail: types::SequencesSyncStateDetail::Loaded {
                                    title_sequence_map,
                                },
                            },
                        })
                    }
                    Err(error) => {
                        namui::log!("{:#?}", error);
                        namui::event::send(SequenceListEvent::SequencesSyncStateUpdateEvent {
                            state: SequenceSyncState {
                                started_at,
                                detail: types::SequencesSyncStateDetail::Failed { error },
                            },
                        });
                    }
                }
            }
        });
    }
}

async fn sync_sequences_with_sheets(
    storage: &Arc<dyn GithubStorage>,
) -> Result<LinkedHashMap<String, Arc<Sequence>>, String> {
    let sheets = google_spreadsheet::get_sheets().await?;

    save_order_of_spreadsheet_to_local(&storage, &sheets).await?;

    let mut title_sequence_map = get_sequences_with_title(&storage).await?;

    create_new_sequences_if_not_exist_in_sequences_but_in_sheets(&mut title_sequence_map, &sheets);

    delete_sequence_if_not_exist_in_sheets(&mut title_sequence_map, &sheets);

    sync_sequences_subtitles_from_sheets(&sheets, &mut title_sequence_map)?;

    save_sequences(&storage, &title_sequence_map).await?;

    Ok(title_sequence_map)
}

async fn save_sequences(
    storage: &Arc<dyn GithubStorage>,
    title_sequence_map: &LinkedHashMap<String, Arc<Sequence>>,
) -> Result<(), String> {
    let sequence_save_futures = title_sequence_map
        .iter()
        .map(|(title, sequence)| async move {
            (title, storage.put_sequence(title.as_str(), sequence).await)
        });
    let title_sequence_save_result_pair_list = join_all(sequence_save_futures).await;
    throw_error_if_sequence_download_failed(&title_sequence_save_result_pair_list)?;
    Ok(())
}

fn throw_error_if_sequence_download_failed<E>(
    title_sequence_save_result_pair_list: &Vec<(&String, Result<(), E>)>,
) -> Result<(), String>
where
    E: Debug + Sized,
{
    let sequence_save_error_list: Vec<_> = title_sequence_save_result_pair_list
        .iter()
        .filter_map(|(title, result)| {
            if let Err(error) = result {
                return Some((title, error));
            }
            None
        })
        .collect();

    if sequence_save_error_list.len() > 0 {
        return Err(format!(
            "failed to download some sequence: {:#?}",
            sequence_save_error_list
        ));
    }

    Ok(())
}

fn delete_sequence_if_not_exist_in_sheets(
    title_sequence_map: &mut LinkedHashMap<String, Arc<Sequence>>,
    sheets: &[Sheet],
) {
    let titles_in_sheets: Vec<&String> = sheets.iter().map(|sheet| &sheet.title).collect();
    let titles_not_exist_in_sheets: Vec<String> = title_sequence_map
        .keys()
        .filter_map(|title| match titles_in_sheets.contains(&title) {
            true => None,
            false => Some(title.clone()),
        })
        .collect();
    for title in titles_not_exist_in_sheets {
        title_sequence_map.remove(&title);
    }
}

fn sync_sequences_subtitles_from_sheets(
    sheets: &[Sheet],
    title_sequence_map: &mut LinkedHashMap<String, Arc<Sequence>>,
) -> Result<(), String> {
    for sheet in sheets {
        let sequence = title_sequence_map.get_mut(&sheet.title).unwrap();

        let job = SyncSubtitlesJob {
            subtitles: sheet.subtitles.clone(),
        };
        let sequence = job.execute(sequence)?;
        title_sequence_map.insert(sheet.title.clone(), Arc::new(sequence));
    }
    Ok(())
}

fn create_new_sequences_if_not_exist_in_sequences_but_in_sheets(
    title_sequence_map: &mut LinkedHashMap<String, Arc<Sequence>>,
    sheets: &[Sheet],
) {
    for sheet in sheets {
        if title_sequence_map.get(&sheet.title).is_none() {
            let new_sequence = Arc::new(Sequence::default());
            title_sequence_map.insert(sheet.title.clone(), new_sequence);
        }
    }
}

async fn save_order_of_spreadsheet_to_local(
    storage: &Arc<dyn GithubStorage>,
    sheets: &Vec<Sheet>,
) -> Result<(), String> {
    let sheet_titles: Vec<String> = sheets.iter().map(|sheet| sheet.title.clone()).collect();
    SequenceIndex::new(sheet_titles).save(storage).await
}
