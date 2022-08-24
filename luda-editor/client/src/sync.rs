use crate::storage::Storage;
use crdt::yrs::updates::decoder::Decode;
use editor_core::storage::SyncStatus;
use namui::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Syncer {
    update_queue: Arc<Mutex<Vec<crdt::yrs::Update>>>,
    update_sync_status: Arc<Mutex<SyncStatus>>,
}

pub enum Event {
    NewHistorySystem { encoded: Box<[u8]> },
}

impl Syncer {
    pub fn new(storage: Storage, encoded_history_system: Box<[u8]>) -> Self {
        Self::start_sync_receive(storage.clone(), encoded_history_system.clone());
        let update_queue = Arc::new(Mutex::new(vec![crdt::yrs::Update::decode_v2(
            &encoded_history_system,
        )
        .unwrap()]));
        let update_sync_status = Arc::new(Mutex::new(SyncStatus::Idle));
        Self::start_update_sync(
            storage.clone(),
            update_queue.clone(),
            update_sync_status.clone(),
        );

        Self {
            update_queue,
            update_sync_status,
        }
    }
    pub fn get_sync_status(&self) -> SyncStatus {
        self.update_sync_status.lock().unwrap().clone()
    }
    fn start_sync_receive(storage: Storage, encoded_history_system: Box<[u8]>) {
        namui::spawn_local({
            let storage = storage.clone();
            async move {
                const DEAFULT_WAIT_DELAY: Time = Time::Ms(1000.0);
                const MAX_WAIT_DELAY: Time = Time::Ms(10000.0);
                let mut wait_delay = DEAFULT_WAIT_DELAY;

                let mut last_encoded_history_system = encoded_history_system;
                loop {
                    match storage.get().await {
                        Ok(data) => {
                            wait_delay = DEAFULT_WAIT_DELAY;

                            let new_encoded_history_system = data.encode();

                            if new_encoded_history_system != last_encoded_history_system {
                                namui::event::send(Event::NewHistorySystem {
                                    encoded: new_encoded_history_system.clone(),
                                });
                                last_encoded_history_system = new_encoded_history_system;
                            }
                            namui::time::delay(wait_delay).await;
                        }
                        Err(error) => {
                            namui::log!(
                                "error on get storage: {}. retry after {:?}",
                                error,
                                wait_delay
                            );
                            namui::time::delay(wait_delay).await;
                            wait_delay = (wait_delay * 2_i32).min(MAX_WAIT_DELAY);
                        }
                    }
                }
            }
        })
    }
    fn start_update_sync(
        storage: Storage,
        update_queue: Arc<Mutex<Vec<crdt::yrs::Update>>>,
        update_sync_status: Arc<Mutex<SyncStatus>>,
    ) {
        namui::spawn_local({
            let storage = storage.clone();
            async move { storage.start_sync(update_queue, update_sync_status).await }
        })
    }
    pub fn send(&self, encoded_update: impl AsRef<[u8]>) {
        self.update_queue
            .lock()
            .unwrap()
            .push(crdt::yrs::Update::decode_v2(encoded_update.as_ref()).unwrap());
    }
}
