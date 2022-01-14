use crate::namui;
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

pub fn send(event: impl Any + Send + Sync) {
    EVENT_SENDER.get().unwrap().send(Box::new(event)).unwrap();
}

pub enum NamuiEvent {
    AnimationFrame,
    MouseDown(namui::Xy<f32>),
    MouseUp(namui::Xy<f32>),
    MouseMove(namui::Xy<f32>),
    ScreenResize(namui::Wh<i16>),
    Wheel(namui::Xy<f32>),
}
