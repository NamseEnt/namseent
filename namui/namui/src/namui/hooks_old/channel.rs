use super::*;
use std::sync::OnceLock;

static CHANNEL_TX: OnceLock<std::sync::mpsc::Sender<Item>> = OnceLock::new();

#[derive(Debug)]
pub(crate) enum Item {
    SetStateItem(SetStateItem),
    MutStateCalled { sig_id: SigId },
}
impl Item {
    pub(crate) fn sig_id(&self) -> SigId {
        match self {
            Item::SetStateItem(set_state_item) => set_state_item.sig_id(),
            Item::MutStateCalled { sig_id } => *sig_id,
        }
    }
}

pub(crate) fn init(channel_tx: std::sync::mpsc::Sender<Item>) {
    CHANNEL_TX.set(channel_tx).unwrap();
}

pub(crate) fn send(item: Item) {
    CHANNEL_TX.get().unwrap().send(item).unwrap();
}
