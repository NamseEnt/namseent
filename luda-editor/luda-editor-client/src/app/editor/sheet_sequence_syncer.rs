use crate::app::{editor::events::EditorEvent, types::Subtitle};
use namui::Language;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, PartialEq)]
pub enum SheetSequenceSyncerStatus {
    Idle,
    Syncing,
    Failed(String),
    Successful,
}

pub(super) struct SheetSequenceSyncer {
    status: SheetSequenceSyncerStatus,
    sequence_title: String,
}

pub enum SheetSequenceSyncerEvent {
    RequestSyncStart,
    SyncDone(Result<(), String>),
}

impl SheetSequenceSyncer {
    pub(super) fn new(sequence_title: &str) -> Self {
        Self {
            status: SheetSequenceSyncerStatus::Idle,
            sequence_title: sequence_title.to_string(),
        }
    }
    pub(super) fn get_status(&self) -> SheetSequenceSyncerStatus {
        self.status.clone()
    }
    pub(super) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SheetSequenceSyncerEvent>() {
            match event {
                SheetSequenceSyncerEvent::RequestSyncStart => {
                    self.start_sync();
                }
                SheetSequenceSyncerEvent::SyncDone(result) => match result {
                    Ok(_) => {
                        self.status = SheetSequenceSyncerStatus::Successful;
                    }
                    Err(error) => {
                        self.status = SheetSequenceSyncerStatus::Failed(error.to_string());
                    }
                },
                _ => {}
            }
        }
    }

    fn start_sync(&mut self) {
        if self.status == SheetSequenceSyncerStatus::Syncing {
            return;
        }

        self.status = SheetSequenceSyncerStatus::Syncing;

        const SPREADSHEET_ID: &str = "1TSSmaIuBjTLVSYcCL0olqWeu9MTaYP4LEq9M1xzn1OU";
        const API_KEY: &str = "AIzaSyBhMI9rz9l_f5NFsZlSh48K6Ee3Cbf4Oxw";

        let range = format!("{}!A1:Z", self.sequence_title);

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
            SPREADSHEET_ID, range, API_KEY
        );

        spawn_local(async move {
            let result = namui::fetch_get_json::<SpreadsheetValuesGet>(&url).await;

            if result.is_err() {
                let error = result.err().unwrap();
                namui::event::send(SheetSequenceSyncerEvent::SyncDone(Err(error.to_string())));
                return;
            }

            let spreadsheet_values_get = result.unwrap();

            let subtitles = spreadsheet_values_get.into_subtitles();
            namui::event::send(EditorEvent::SubtitleSyncRequestEvent { subtitles });
        });
    }
}

#[derive(Deserialize, Debug)]
struct SpreadsheetValuesGet {
    values: Box<[Box<[String]>]>,
}
impl SpreadsheetValuesGet {
    pub(crate) fn into_subtitles(&self) -> Vec<Subtitle> {
        self.values
            .iter()
            .skip(1)
            .filter_map(|row| {
                if row.len() < 7 {
                    return None;
                }

                let id = row[0].clone();
                let korean_text = row[6].clone();
                Some(Subtitle {
                    id,
                    language_text_map: vec![(Language::Ko, korean_text)].into_iter().collect(),
                })
            })
            .collect()
    }
}
