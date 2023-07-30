use super::*;
use std::sync::OnceLock;

static CHANNEL: OnceLock<Mutex<Vec<Item>>> = OnceLock::new();

#[derive(Debug)]
pub(crate) enum Item {
    SetStateItem(SetStateItem),
    EventCallback(EventCallback),
}

pub(crate) fn init() {
    CHANNEL.set(Default::default()).unwrap();
}

pub(crate) fn send(item: Item) {
    CHANNEL.get().unwrap().lock().unwrap().push(item);
}

pub(crate) fn drain() -> Vec<Item> {
    let mut channel = CHANNEL.get().unwrap().lock().unwrap();
    channel.drain(..).collect()
}
