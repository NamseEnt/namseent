use super::*;
use std::sync::OnceLock;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub(crate) static TX: OnceLock<UnboundedSender<Item>> = OnceLock::new();
pub(crate) static mut RX: OnceLock<UnboundedReceiver<Item>> = OnceLock::new();

#[derive(Debug)]
pub(crate) enum Item {
    SetStateItem(SetStateItem),
    EventCallback(EventCallback),
}

pub(crate) fn init() {
    let (tx, rx) = unbounded_channel();

    TX.set(tx).unwrap();
    unsafe { RX.set(rx).unwrap() };
}

pub(crate) fn send(item: Item) {
    TX.get().unwrap().send(item).unwrap();
}
