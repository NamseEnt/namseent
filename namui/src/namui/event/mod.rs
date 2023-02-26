use crate::{
    drag_and_drop::RawFileDropEvent, namui, DeepLinkOpenedEvent, RawKeyboardEvent, RawWheelEvent,
};
use once_cell::sync::OnceCell;
use std::any::Any;
use tokio::sync::mpsc::{self, unbounded_channel};

static EVENT_SENDER: OnceCell<mpsc::UnboundedSender<Event>> = OnceCell::new();
pub(crate) type EventReceiver = mpsc::UnboundedReceiver<Event>;

pub fn init() -> EventReceiver {
    let (sender, receiver) = unbounded_channel();
    EVENT_SENDER.set(sender).unwrap();
    receiver
}

pub fn send(event: impl Any + Send + Sync) {
    EVENT_SENDER
        .get()
        .unwrap()
        .send(Event {
            inner: Box::new(event),
        })
        .unwrap();
}

#[derive(Debug)]
pub struct Event {
    inner: Box<dyn Any + Send + Sync>,
}
impl Event {
    pub fn is<T: 'static>(&self, callback: impl FnOnce(&T)) -> &Self {
        if let Some(event) = self.inner.downcast_ref::<T>() {
            callback(event);
        }
        self
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }
}

#[derive(Debug)]
pub enum NamuiEvent {
    AnimationFrame,
    KeyDown(RawKeyboardEvent),
    KeyUp(RawKeyboardEvent),
    ScreenResize(namui::Wh<i16>),
    Wheel(RawWheelEvent),
    DeepLinkOpened(DeepLinkOpenedEvent),
    FileDrop(RawFileDropEvent),
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn received_event_should_be_able_to_downcast() {
        let mut event_receiver = super::init();
        #[derive(Debug, PartialEq)]
        enum Event {
            Test,
        }
        super::send(Event::Test);
        let mut is_called = false;
        let event = event_receiver.recv().await.unwrap();
        event.is::<Event>(|event| {
            is_called = true;
            assert_eq!(event, &Event::Test);
        });
        assert!(is_called);
    }
}
