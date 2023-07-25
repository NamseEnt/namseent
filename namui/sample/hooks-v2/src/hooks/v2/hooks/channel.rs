use super::*;
use std::sync::OnceLock;
use tokio::sync::mpsc::UnboundedReceiver;

pub(crate) static TX: OnceLock<tokio::sync::mpsc::UnboundedSender<Item>> = OnceLock::new();

#[derive(Debug)]
pub(crate) enum Item {
    SetStateItem(SetStateItem),
    EventCallback(EventCallback),
}

pub(crate) fn init() -> UnboundedReceiver<Item> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    TX.set(tx).unwrap();

    rx
}

pub(crate) fn send(item: Item) {
    namui::log!("Channel Send: {:#?}", item);
    TX.get().unwrap().send(item).unwrap();
}
