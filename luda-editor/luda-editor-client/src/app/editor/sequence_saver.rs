use crate::app::types::*;
use luda_editor_rpc::{write_file, Socket};
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

pub(super) struct SequenceSaver {
    file_path: String,
    last_changed_sequence: Arc<Sequence>,
    last_saved_sequence: Arc<Sequence>,
    socket: Socket,
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
    pub(super) fn new(file_path: &str, sequence: Arc<Sequence>, socket: Socket) -> Self {
        Self {
            file_path: file_path.to_string(),
            last_changed_sequence: sequence.clone(),
            last_saved_sequence: sequence.clone(),
            socket,
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
        let serialize_result = serde_json::to_vec_pretty(sequence.as_ref());
        if serialize_result.is_err() {
            let error = serialize_result.err().unwrap();
            namui::event::send(SequenceSaverEvent::SavingDone(Err(error.to_string())));
            return;
        }

        let buffer = serialize_result.unwrap();
        spawn_local({
            let socket = self.socket.clone();
            let file_path = self.file_path.clone();
            async move {
                match socket
                    .write_file(write_file::Request {
                        dest_path: file_path,
                        file: buffer,
                    })
                    .await
                {
                    Ok(_) => {
                        namui::event::send(SequenceSaverEvent::SavingDone(Ok(sequence)));
                    }
                    Err(error) => {
                        namui::event::send(SequenceSaverEvent::SavingDone(Err(error.to_string())));
                    }
                }
            }
        });
    }
    pub(super) fn get_status(&self) -> SequenceSaverStatus {
        self.status.clone()
    }
}
