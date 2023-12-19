use super::*;
use std::sync::{atomic::AtomicBool, OnceLock};

static CHANNEL: OnceLock<Mutex<Vec<Item>>> = OnceLock::new();
static RERENDER_REQUESTED: OnceLock<AtomicBool> = OnceLock::new();

#[derive(Debug)]
pub(crate) enum Item {
    SetStateItem(SetStateItem),
}
impl Item {
    pub(crate) fn sig_id(&self) -> SigId {
        match self {
            Item::SetStateItem(set_state_item) => set_state_item.sig_id(),
        }
    }
}

pub(crate) fn init() {
    CHANNEL.set(Default::default()).unwrap();
    RERENDER_REQUESTED.set(Default::default()).unwrap();
}

pub(crate) fn send(item: Item) {
    CHANNEL.get().unwrap().lock().unwrap().push(item);

    if !RERENDER_REQUESTED
        .get()
        .unwrap()
        .swap(true, std::sync::atomic::Ordering::Relaxed)
    {
        crate::spawn(async move {
            RERENDER_REQUESTED
                .get()
                .unwrap()
                .store(false, std::sync::atomic::Ordering::Relaxed);

            if is_empty() {
                return;
            }
            crate::hooks::render_and_draw();
        });
    }
}

pub(crate) fn drain() -> Vec<Item> {
    let mut channel = CHANNEL.get().unwrap().lock().unwrap();
    channel.drain(..).collect()
}

pub(crate) fn is_empty() -> bool {
    let channel = CHANNEL.get().unwrap().lock().unwrap();
    channel.is_empty()
}
