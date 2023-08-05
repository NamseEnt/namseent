use super::*;
use std::sync::OnceLock;

static CHANNEL: OnceLock<Mutex<Vec<Item>>> = OnceLock::new();

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
}

pub(crate) fn send(item: Item) {
    crate::log!("send item: {:?}", item);
    CHANNEL.get().unwrap().lock().unwrap().push(item);
}

pub(crate) fn drain() -> Vec<Item> {
    let mut channel = CHANNEL.get().unwrap().lock().unwrap();
    channel.drain(..).collect()
}
