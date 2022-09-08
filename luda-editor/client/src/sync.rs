use namui::prelude::*;
use rpc::json_patch::Patch;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub enum SyncStatus {
    Idle,
    Syncing(namui::Time),
    Synced(namui::Time),
    Error(String),
}
pub struct Syncer<State: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned + Clone> {
    id: String,
    is_dropped: Arc<Mutex<bool>>,
    update_sync_status: Arc<Mutex<SyncStatus>>,
    patch_state: Arc<Mutex<Option<PatchState<Patch, State>>>>,
}

#[derive(Clone)]
struct PatchState<Patch: Clone, State: Clone> {
    pub patch: Patch,
    pub state: State,
}

pub enum Event {
    UpdateReceived { patch: Patch, id: String },
}

impl<State: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned + 'static + Clone>
    Syncer<State>
{
    pub fn new<UpdateServer, UpdateServerReturn, UpdateClient, UpdateClientReturn>(
        state: State,
        update_server: UpdateServer,
        update_client: UpdateClient,
    ) -> Self
    where
        UpdateServer: 'static + Fn(Patch) -> UpdateServerReturn,
        UpdateServerReturn: Future<Output = Result<(), Box<dyn std::error::Error>>>,
        UpdateClient: 'static + Fn(serde_json::Value) -> UpdateClientReturn,
        UpdateClientReturn: Future<Output = Result<Patch, Box<dyn std::error::Error>>>,
    {
        let syncer = Self {
            id: nanoid(),
            is_dropped: Arc::new(Mutex::new(false)),
            update_sync_status: Arc::new(Mutex::new(SyncStatus::Idle)),
            patch_state: Arc::new(Mutex::new(None)),
        };

        syncer.run_sync_loop(state, update_server, update_client);

        syncer
    }
    pub fn get_sync_status(&self) -> SyncStatus {
        self.update_sync_status.lock().unwrap().clone()
    }
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    fn run_sync_loop<UpdateServer, UpdateServerReturn, UpdateClient, UpdateClientReturn>(
        &self,
        state: State,
        update_server: UpdateServer,
        update_client: UpdateClient,
    ) where
        UpdateServer: 'static + Fn(Patch) -> UpdateServerReturn,
        UpdateServerReturn: Future<Output = Result<(), Box<dyn std::error::Error>>>,
        UpdateClient: 'static + Fn(serde_json::Value) -> UpdateClientReturn,
        UpdateClientReturn: Future<Output = Result<Patch, Box<dyn std::error::Error>>>,
    {
        namui::spawn_local({
            let self_update_sync_status = self.update_sync_status.clone();
            let self_is_dropped = self.is_dropped.clone();
            let self_patch_state = self.patch_state.clone();
            let id = self.id.clone();

            async move {
                const DEAFULT_WAIT_DELAY: Time = Time::Ms(1000.0);
                const MAX_WAIT_DELAY: Time = Time::Ms(10000.0);
                let mut wait_delay = DEAFULT_WAIT_DELAY;
                let mut patch = rpc::json_patch::Patch(vec![]);
                let mut state = state;

                while {
                    let is_dropped = self_is_dropped.lock().unwrap().clone();
                    !is_dropped
                } {
                    // Merge ing_patch_state
                    {
                        if let Some(mut self_patch_state) = self_patch_state.lock().unwrap().take()
                        {
                            patch.0.append(self_patch_state.patch.0.as_mut());
                            state = self_patch_state.state;
                        }
                    }

                    // Upload patch
                    {
                        // TODO: Make patch idempotent
                        // TODO: Remove clone
                        if !patch.0.is_empty() {
                            match update_server(patch.clone()).await {
                                Ok(_) => {
                                    patch.0.clear();

                                    if self_patch_state.lock().unwrap().is_some() {
                                        continue;
                                    }
                                }
                                Err(error) => {
                                    {
                                        *self_update_sync_status.lock().unwrap() =
                                            SyncStatus::Error(error.to_string());
                                    }
                                    namui::log!("error: Fail to update_server. {}", error);
                                    wait_delay = (wait_delay * 2_i32).min(MAX_WAIT_DELAY);
                                    namui::time::delay(wait_delay).await;
                                    continue;
                                }
                            }
                        }
                    }

                    // Download patch
                    loop {
                        let mut json = serde_json::to_value::<State>(state.clone())
                            .expect(&format!("Fail to serialize state {:?}", state));

                        match update_client(json.clone()).await {
                            Ok(patch) => {
                                if patch.0.len() > 0 {
                                    state = {
                                        rpc::json_patch::patch(&mut json, &patch).unwrap();
                                        serde_json::from_value::<State>(json).unwrap()
                                    };

                                    namui::event::send(Event::UpdateReceived {
                                        patch,
                                        id: id.clone(),
                                    });
                                }
                                {
                                    if self_patch_state.lock().unwrap().is_none() {
                                        *self_update_sync_status.lock().unwrap() =
                                            SyncStatus::Synced(namui::time::now());
                                    }
                                }
                                wait_delay = DEAFULT_WAIT_DELAY;
                                namui::time::delay(wait_delay).await;
                                break;
                            }
                            Err(error) => {
                                {
                                    *self_update_sync_status.lock().unwrap() =
                                        SyncStatus::Error(error.to_string());
                                }

                                {
                                    if self_patch_state.lock().unwrap().is_some() {
                                        break;
                                    }
                                }

                                namui::log!("error: Fail to update_client. {}", error);
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
    pub fn push_patch(&self, mut patch: Patch, state: State) {
        let mut self_patch_state = self.patch_state.lock().unwrap();
        match self_patch_state.as_mut() {
            Some(patch_state) => {
                patch_state.patch.0.append(&mut patch.0);
                patch_state.state = state;
            }
            None => *self_patch_state = Some(PatchState { patch, state }),
        }
        *self.update_sync_status.lock().unwrap() = SyncStatus::Syncing(namui::time::now());
    }
}

impl<State: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned + Clone> Drop
    for Syncer<State>
{
    fn drop(&mut self) {
        *self.is_dropped.lock().unwrap() = true;
    }
}
