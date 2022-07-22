use crate::app::{
    editor::top_bar::TopBarEvent,
    storage::{GithubStorage, LockInfo},
};
use namui::prelude::*;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

pub struct SequenceLockExtender {
    storage: Arc<dyn GithubStorage>,
    sequence_title: String,
    lock_info: LockInfo,
    extend_state: ExtendState,
    retry_count: usize,
}
impl SequenceLockExtender {
    pub fn new(
        storage: Arc<dyn GithubStorage>,
        sequence_title: String,
        lock_info: LockInfo,
    ) -> Self {
        let sequence_lock_extender = Self {
            storage,
            sequence_title,
            lock_info,
            extend_state: ExtendState::Idle,
            retry_count: 0,
        };
        sequence_lock_extender.tick_after_delay();
        sequence_lock_extender
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        const EXTEND_START_REMAINING_TIME: Time = Time::Sec(15.0);
        const MAX_RETRY_COUNT: usize = 5;
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::CheckSequenceTick { sequence_title } => {
                    if sequence_title != &self.sequence_title {
                        return;
                    }
                    let should_extend = match self.extend_state {
                        ExtendState::Idle => {
                            self.lock_info.get_remaining_time() < EXTEND_START_REMAINING_TIME
                        }
                        ExtendState::Extending => false,
                    };
                    if should_extend {
                        self.extend();
                    }
                    self.tick_after_delay();
                }
                InternalEvent::LockFailed {
                    sequence_title,
                    message,
                } => {
                    if sequence_title != &self.sequence_title {
                        return;
                    }
                    self.retry_count += 1;
                    self.extend_state = ExtendState::Idle;
                    namui::log!("sequence lock extend failed: {message}");
                    if self.retry_count > MAX_RETRY_COUNT {
                        panic!("sequence lock extend failed: {message}");
                    }
                }
                InternalEvent::LockSuccess {
                    sequence_title,
                    lock_info,
                } => {
                    if sequence_title != &self.sequence_title {
                        return;
                    }
                    self.retry_count = 0;
                    self.extend_state = ExtendState::Idle;
                    self.lock_info = lock_info.clone();
                }
            }
        } else if let Some(event) = event.downcast_ref::<TopBarEvent>() {
            match event {
                TopBarEvent::GoBackButtonClicked => {
                    let storage = self.storage.clone();
                    let sequence_title = self.sequence_title.clone();
                    spawn_local(async move {
                        let _ = storage.unlock_sequence(sequence_title.as_str()).await;
                    })
                }
            }
        }
    }
    fn extend(&mut self) {
        self.extend_state = ExtendState::Extending;
        let storage = self.storage.clone();
        let sequence_title = self.sequence_title.clone();
        spawn_local(async move {
            match storage.lock_sequence(sequence_title.as_str()).await {
                Ok(lock_info) => {
                    namui::event::send(InternalEvent::LockSuccess {
                        sequence_title: sequence_title.clone(),
                        lock_info,
                    });
                }
                Err(error) => {
                    namui::event::send(InternalEvent::LockFailed {
                        sequence_title: sequence_title.clone(),
                        message: format!("{error:#?}"),
                    });
                }
            };
        });
    }
    fn tick_after_delay(&self) {
        const CHECK_INTERVAL: Time = Time::Sec(1.0);
        let sequence_title = self.sequence_title.clone();
        set_timeout(
            move || {
                namui::event::send(InternalEvent::CheckSequenceTick { sequence_title });
            },
            CHECK_INTERVAL,
        );
    }
}

enum InternalEvent {
    CheckSequenceTick {
        sequence_title: String,
    },
    LockFailed {
        sequence_title: String,
        message: String,
    },
    LockSuccess {
        sequence_title: String,
        lock_info: LockInfo,
    },
}

enum ExtendState {
    Idle,
    Extending,
}
