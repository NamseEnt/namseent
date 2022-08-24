use crate::storage::HistorySystem;
use namui::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum SyncStatus {
    Idle,
    Syncing(namui::Time),
    Synced(namui::Time),
    Error(String),
}
/// Syncer: Yrs Doc sync system
/// Please let Syncer know what changed.
/// Syncer will update server automatically. If server updated, sync_status will be set to Synced.
/// If Syncer get new update from server, it will send event to you. Check `sequence_id` to clarify which Syncer is updated.
pub struct Syncer {
    sequence_id: String,
    update_sync_status: Arc<Mutex<SyncStatus>>,
    client_state_vector: Arc<Mutex<Box<[u8]>>>,
    server_state_vector: Arc<Mutex<Box<[u8]>>>,
    update_for_server: Arc<Mutex<Option<Box<[u8]>>>>,
    is_dropped: Arc<Mutex<bool>>,
}

pub enum Event {
    UpdateReceived {
        sequence_id: String,
        /// Please check that update_encoded_v2 could be empty update.
        update_encoded_v2: Box<[u8]>,
    },
}

impl Syncer {
    pub fn new(
        sequence_id: String,
        client_state_vector: Box<[u8]>,
        server_state_vector: Box<[u8]>,
        e_tag: String,
    ) -> Self {
        let syncer = Self {
            sequence_id,
            update_sync_status: Arc::new(Mutex::new(SyncStatus::Idle)),
            client_state_vector: Arc::new(Mutex::new(client_state_vector)),
            server_state_vector: Arc::new(Mutex::new(server_state_vector)),
            update_for_server: Arc::new(Mutex::new(None)),
            is_dropped: Arc::new(Mutex::new(false)),
        };

        syncer.run_sync_loop(e_tag);

        syncer
    }
    pub fn get_sync_status(&self) -> SyncStatus {
        self.update_sync_status.lock().unwrap().clone()
    }
    fn run_sync_loop(&self, mut e_tag: String) {
        namui::spawn_local({
            let sequence_id = self.sequence_id.clone();
            let client_state_vector = self.client_state_vector.clone();
            let server_state_vector = self.server_state_vector.clone();
            let update_for_server = self.update_for_server.clone();
            let update_sync_status = self.update_sync_status.clone();
            let is_dropped = self.is_dropped.clone();
            async move {
                const DEAFULT_WAIT_DELAY: Time = Time::Ms(1000.0);
                const MAX_WAIT_DELAY: Time = Time::Ms(10000.0);
                let mut wait_delay = DEAFULT_WAIT_DELAY;

                struct OnUpdateForClientParam {
                    server_state_vector_base64: String,
                    yrs_update_v2_for_client_base64: String,
                    e_tag: String,
                }

                let on_update_for_client = |param: OnUpdateForClientParam, e_tag: &mut String| {
                    namui::event::send(Event::UpdateReceived {
                        sequence_id: sequence_id.clone(),
                        update_encoded_v2: rpc::base64::decode(
                            param.yrs_update_v2_for_client_base64,
                        )
                        .unwrap()
                        .into_boxed_slice(),
                    });
                    *server_state_vector.lock().unwrap() =
                        rpc::base64::decode(param.server_state_vector_base64)
                            .unwrap()
                            .into_boxed_slice();
                    *e_tag = param.e_tag;
                };

                while {
                    let is_dropped = is_dropped.lock().unwrap().clone();
                    !is_dropped
                } {
                    // Update Client
                    {
                        let client_state_vector_base64 =
                            { rpc::base64::encode(client_state_vector.lock().unwrap().as_ref()) };

                        let result = crate::RPC
                            .update_client_sequence(rpc::update_client_sequence::Request {
                                sequence_id: sequence_id.clone(),
                                client_state_vector_base64,
                                e_tag: Some(e_tag.clone()),
                            })
                            .await;

                        match result {
                            Err(error) => {
                                match error {
                                    rpc::update_client_sequence::Error::ServerSequenceNotExists => {
                                        // ignores
                                    }
                                    rpc::update_client_sequence::Error::Unknown(error) => {
                                        {
                                            *update_sync_status.lock().unwrap() =
                                                SyncStatus::Error(format!("error: {:?}", error));
                                        }
                                        namui::log!("error: update_server_sequence{}", error);
                                        wait_delay = (wait_delay * 2_i32).min(MAX_WAIT_DELAY);
                                        namui::time::delay(wait_delay).await;
                                        continue;
                                    }
                                }
                            }
                            Ok(response) => {
                                match response {
                                    rpc::update_client_sequence::Response::NotModified => {
                                        // ignores
                                    }
                                    rpc::update_client_sequence::Response::Modified {
                                        e_tag: received_e_tag,
                                        yrs_update_v2_for_client_base64,
                                        server_state_vector_base64,
                                    } => (on_update_for_client)(
                                        OnUpdateForClientParam {
                                            server_state_vector_base64,
                                            yrs_update_v2_for_client_base64,
                                            e_tag: received_e_tag,
                                        },
                                        &mut e_tag,
                                    ),
                                }
                            }
                        }
                    }

                    // Update Server
                    {
                        let yrs_update_v2_for_server = {
                            let update_for_server = { update_for_server.lock().unwrap().clone() };
                            match update_for_server {
                                Some(update_for_server) => update_for_server,
                                None => {
                                    wait_delay = DEAFULT_WAIT_DELAY;
                                    namui::time::delay(wait_delay).await;
                                    continue;
                                }
                            }
                        };

                        let client_state_vector_base64 =
                            { rpc::base64::encode(client_state_vector.lock().unwrap().as_ref()) };
                        let result = crate::RPC
                            .update_server_sequence(rpc::update_server_sequence::Request {
                                sequence_id: sequence_id.clone(),
                                client_state_vector_base64,
                                yrs_update_v2_for_server_base64: rpc::base64::encode(
                                    yrs_update_v2_for_server.as_ref(),
                                ),
                            })
                            .await;

                        match result {
                            Ok(response) => {
                                {
                                    let mut mutex_guard_update_for_server =
                                        update_for_server.lock().unwrap();
                                    if let Some(inner) = mutex_guard_update_for_server.as_ref() {
                                        if yrs_update_v2_for_server.eq(inner) {
                                            *mutex_guard_update_for_server = None;
                                        }
                                    }

                                    (on_update_for_client)(
                                        OnUpdateForClientParam {
                                            server_state_vector_base64: response
                                                .server_state_vector_base64,
                                            yrs_update_v2_for_client_base64: response
                                                .yrs_update_v2_for_client_base64,
                                            e_tag: response.e_tag,
                                        },
                                        &mut e_tag,
                                    );
                                }

                                update_sync_status
                                    .lock()
                                    .unwrap()
                                    .clone_from(&SyncStatus::Synced(namui::time::now()));
                            }
                            Err(error) => {
                                {
                                    *update_sync_status.lock().unwrap() =
                                        SyncStatus::Error(format!("error: {:?}", error));
                                }

                                namui::log!("error: update_server_sequence {}", error);
                                wait_delay = (wait_delay * 2_i32).min(MAX_WAIT_DELAY);
                                namui::time::delay(wait_delay).await;
                                continue;
                            }
                        }
                    }
                }
            }
        })
    }
    pub fn on_client_updated(&self, history_system: &HistorySystem) {
        {
            *self.client_state_vector.lock().unwrap() = history_system.state_vector().into();

            let server_state_vector = self.server_state_vector.lock().unwrap();

            let update_for_server =
                history_system.encode_against_state_vector(server_state_vector.as_ref());

            self.update_for_server
                .lock()
                .unwrap()
                .replace(update_for_server.as_ref().into());
        }

        *self.update_sync_status.lock().unwrap() = SyncStatus::Syncing(namui::time::now());
    }
}

impl Drop for Syncer {
    fn drop(&mut self) {
        *self.is_dropped.lock().unwrap() = true;
    }
}
