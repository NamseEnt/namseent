mod raw;

use crate::*;
use derivative::Derivative;
pub use raw::*;
use std::{collections::HashSet, fmt::Debug, sync::atomic::AtomicBool};

#[derive(Debug)]
pub enum Event<'a> {
    MouseDown { event: MouseEvent<'a> },
    MouseMove { event: MouseEvent<'a> },
    MouseUp { event: MouseEvent<'a> },
    Wheel { event: WheelEvent<'a> },
    KeyDown { event: KeyboardEvent<'a> },
    KeyUp { event: KeyboardEvent<'a> },
    Blur,
    VisibilityChange,
    ScreenResize { wh: Wh<IntPx> },
    ScreenRedraw,
}

pub trait EventExt {
    fn stop_propagation(&self);
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MouseEvent<'a> {
    #[derivative(Debug = "ignore")]
    pub local_xy: &'a dyn Fn() -> Xy<Px>,
    #[derivative(Debug = "ignore")]
    pub is_local_xy_in: &'a dyn Fn() -> bool,
    pub global_xy: Xy<Px>,
    pub pressing_buttons: &'a HashSet<MouseButton>,
    pub button: Option<MouseButton>,
    pub event_type: MouseEventType,
    #[cfg(target_family = "wasm")]
    #[derivative(Debug = "ignore")]
    pub prevent_default: &'a dyn Fn(),
    pub is_stop_event_propagation: &'a AtomicBool,
}
impl EventExt for MouseEvent<'_> {
    fn stop_propagation(&self) {
        self.is_stop_event_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
impl MouseEvent<'_> {
    pub fn local_xy(&self) -> Xy<Px> {
        (self.local_xy)()
    }
    pub fn is_local_xy_in(&self) -> bool {
        (self.is_local_xy_in)()
    }
    #[cfg(target_family = "wasm")]
    pub fn prevent_default(&self) {
        (self.prevent_default)();
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct WheelEvent<'a> {
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
    pub delta_xy: Xy<f32>,
    #[derivative(Debug = "ignore")]
    pub is_local_xy_in: &'a dyn Fn() -> bool,
    #[derivative(Debug = "ignore")]
    pub local_xy: &'a dyn Fn() -> Xy<Px>,
    pub is_stop_event_propagation: &'a AtomicBool,
}
impl EventExt for WheelEvent<'_> {
    fn stop_propagation(&self) {
        self.is_stop_event_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
impl WheelEvent<'_> {
    pub fn local_xy(&self) -> Xy<Px> {
        (self.local_xy)()
    }
    pub fn is_local_xy_in(&self) -> bool {
        (self.is_local_xy_in)()
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct KeyboardEvent<'a> {
    pub code: Code,
    pub pressing_codes: &'a HashSet<Code>,
    #[cfg(target_family = "wasm")]
    #[derivative(Debug = "ignore")]
    pub prevent_default: &'a dyn Fn(),
    pub is_stop_event_propagation: &'a AtomicBool,
}
impl EventExt for KeyboardEvent<'_> {
    fn stop_propagation(&self) {
        self.is_stop_event_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
impl KeyboardEvent<'_> {
    #[cfg(target_family = "wasm")]
    pub fn prevent_default(&self) {
        (self.prevent_default)();
    }
}
