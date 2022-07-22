use crate::app::{storage::GithubStorage, types::*};
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

pub(super) struct SequenceSaver {
    sequence_title: String,
    last_changed_sequence: Arc<Sequence>,
    last_saved_sequence: Arc<Sequence>,
    storage: Arc<dyn GithubStorage>,
    status: SequenceSaverStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceSaverStatus {
    Idle,
    Saving,
    Saved,
    Failed(String),
}

pub(super) enum SequenceSaverEvent {
    SavingDone(Result<Arc<Sequence>, String>),
}

impl SequenceSaver {
    pub(super) fn new(
        sequence_title: &str,
        sequence: Arc<Sequence>,
        storage: Arc<dyn GithubStorage>,
    ) -> Self {
        Self {
            sequence_title: sequence_title.to_string(),
            last_changed_sequence: sequence.clone(),
            last_saved_sequence: sequence.clone(),
            storage,
            status: SequenceSaverStatus::Idle,
        }
    }
    pub(super) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequenceSaverEvent>() {
            match event {
                SequenceSaverEvent::SavingDone(result) => {
                    self.status = match result {
                        Ok(sequence) => {
                            self.last_saved_sequence = sequence.clone();
                            SequenceSaverStatus::Saved
                        }
                        Err(error) => SequenceSaverStatus::Failed(error.to_string()),
                    };

                    if result.is_ok() {
                        self.try_save();
                    }
                }
            }
        }
    }
    pub(super) fn on_change_sequence(&mut self, sequence: Arc<Sequence>) {
        self.last_changed_sequence = sequence;
        self.try_save();
    }
    fn try_save(&mut self) {
        if self.status == SequenceSaverStatus::Saving
            || Arc::ptr_eq(&self.last_changed_sequence, &self.last_saved_sequence)
        {
            return;
        }

        self.status = SequenceSaverStatus::Saving;

        let sequence = self.last_changed_sequence.clone();
        let storage = self.storage.clone();
        let sequence_title = self.sequence_title.clone();
        spawn_local(async move {
            let save_result = storage
                .put_sequence(sequence_title.as_str(), &sequence)
                .await
                .map(|_| sequence)
                .map_err(|error| format!("Failed to save sequence: {:#?}", error));
            namui::event::send(SequenceSaverEvent::SavingDone(save_result));
        });
    }
    pub(super) fn get_status(&self) -> SequenceSaverStatus {
        self.status.clone()
    }
}
