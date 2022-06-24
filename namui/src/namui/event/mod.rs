use crate::{namui, RawKeyboardEvent, RawMouseEvent, RawWheelEvent};
use once_cell::sync::OnceCell;
use std::any::Any;
use tokio::sync::mpsc::{self, unbounded_channel};

pub type Event = Box<dyn Any + Send + Sync>;
static EVENT_SENDER: OnceCell<mpsc::UnboundedSender<Event>> = OnceCell::new();
pub(crate) type EventReceiver = mpsc::UnboundedReceiver<Event>;

pub fn init() -> EventReceiver {
    let (sender, receiver) = unbounded_channel();
    EVENT_SENDER.set(sender).unwrap();
    receiver
}

pub fn send(event: impl Any + Send + Sync) {
    EVENT_SENDER.get().unwrap().send(Box::new(event)).unwrap();
}

#[derive(Debug)]
pub enum NamuiEvent {
    AnimationFrame,
    MouseDown(RawMouseEvent),
    MouseUp(RawMouseEvent),
    MouseMove(RawMouseEvent),
    KeyDown(RawKeyboardEvent),
    KeyUp(RawKeyboardEvent),
    ScreenResize(namui::Wh<i16>),
    Wheel(RawWheelEvent),
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
        let event = event_receiver.recv().await.unwrap();
        let downcasted = event.downcast_ref::<Event>().unwrap();
        assert_eq!(downcasted, &Event::Test);
    }
}
