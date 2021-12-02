use std::{cell::RefCell, marker::PhantomData, rc::Rc, time::Duration};

use super::draw::RenderingTree;

pub trait Surface {}

pub struct FpsInfo {
    pub fps: u16,
    pub frame_count: u16,
    pub last_60_frame_time: Duration,
}

pub struct EngineContext<TState> {
    pub state: TState,
    pub surface: Box<dyn Surface>,
    pub fps_info: FpsInfo,
}

pub trait EngineImpl {
    fn init<TState>(state: TState) -> EngineContext<TState>;
    fn request_animation_frame(callback: Box<dyn FnOnce()>);
    fn log(format: String);
    fn now() -> Duration;
}

pub type Render<TState> = fn(&mut TState) -> Option<RenderingTree>;
