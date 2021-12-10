use once_cell::sync::OnceCell;
use std::any::Any;
use tokio::sync::mpsc::{self, unbounded_channel};

type Event = Box<dyn Any + Send + Sync>;
static EVENT_SENDER: OnceCell<mpsc::UnboundedSender<Event>> = OnceCell::new();

pub fn init() -> mpsc::UnboundedReceiver<Event> {
    let (sender, receiver) = unbounded_channel();
    EVENT_SENDER.set(sender).unwrap();
    receiver
}

pub fn send(event: Event) {
    EVENT_SENDER.get().unwrap().send(event).unwrap();
}
