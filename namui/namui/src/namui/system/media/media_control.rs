use namui_type::*;
use std::sync::{Arc, RwLock};

pub(crate) struct MediaController {
    receivers: Vec<MediaControlReceiver>,
}

impl MediaController {
    pub(crate) fn new() -> Self {
        Self {
            receivers: Vec::new(),
        }
    }
    pub(crate) fn new_receiver(&mut self) -> MediaControlReceiver {
        // TODO: Clone it, not create it.
        let receiver = MediaControlReceiver {
            flush_requested: Arc::new(RwLock::new(None)),
            start_requested: Arc::new(RwLock::new(None)),
        };

        self.receivers.push(receiver.clone());

        receiver
    }
    pub(crate) fn flush(&self) {
        let now = crate::time::now();
        for receiver in &self.receivers {
            *receiver.flush_requested.write().unwrap() = Some(now);
        }
    }
    pub(crate) fn start(&self) {
        let now = crate::time::now();
        for receiver in &self.receivers {
            *receiver.start_requested.write().unwrap() = Some(now);
        }
    }
    pub(crate) fn stop(&self) {
        for receiver in &self.receivers {
            *receiver.start_requested.write().unwrap() = None;
        }
    }
}

/// If performance issue occurs, use rtrb.
#[derive(Debug, Clone)]
pub(crate) struct MediaControlReceiver {
    flush_requested: Arc<RwLock<Option<Instant>>>,
    start_requested: Arc<RwLock<Option<Instant>>>,
}

impl MediaControlReceiver {
    pub(crate) fn flush_requested(&self) -> Option<Instant> {
        *self.flush_requested.read().unwrap()
    }
    pub(crate) fn should_skip_this(&self, instant: Instant) -> bool {
        Some(instant) <= self.flush_requested()
    }
    pub(crate) fn start_requested(&self) -> Option<Instant> {
        *self.start_requested.read().unwrap()
    }
}
