use crate::app::{editor::events::EditorEvent, types::*};

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
            }
        }
    }

    fn start_sync(&mut self) {
        if self.status == SheetSequenceSyncerStatus::Syncing {
            return;
        }

        self.status = SheetSequenceSyncerStatus::Syncing;

        let sequence_title = self.sequence_title.clone();

        spawn_local(async move {
            let result = google_spreadsheet::get_subtitles_by_title(&sequence_title).await;

            match result {
                Ok(subtitles) => {
                    namui::event::send(EditorEvent::SubtitleSyncRequestEvent { subtitles })
                }
                Err(error) => namui::event::send(SheetSequenceSyncerEvent::SyncDone(Err(error))),
            }
        });
    }
}
