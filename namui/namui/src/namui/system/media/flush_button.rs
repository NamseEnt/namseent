use std::sync::{atomic::AtomicBool, Arc};

pub(crate) struct FlushButton {
    receivers: Vec<Arc<AtomicBool>>,
}

impl FlushButton {
    pub(crate) fn new() -> Self {
        Self {
            receivers: Vec::new(),
        }
    }
    pub(crate) fn new_receiver(&mut self) -> FlushButtonReceiver {
        let receiver = Arc::new(AtomicBool::new(false));

        self.receivers.push(receiver.clone());

        FlushButtonReceiver {
            inner: receiver.clone(),
        }
    }
    pub(crate) fn press(&self) {
        for receiver in &self.receivers {
            receiver.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

#[derive(Debug)]
pub(crate) struct FlushButtonReceiver {
    inner: Arc<AtomicBool>,
}

impl FlushButtonReceiver {
    pub(crate) fn take(&self) -> bool {
        self.inner.swap(false, std::sync::atomic::Ordering::SeqCst)
    }
}
