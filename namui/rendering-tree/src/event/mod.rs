mod raw;

use crate::*;
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
    TextInput { event: &'a RawTextInputEvent },
    TextInputKeyDown { event: &'a RawTextInputKeyDownEvent },
    TextInputSelectionChange { event: &'a RawTextInputEvent },
}

pub trait EventExt {
    fn stop_propagation(&self);
}

pub struct MouseEvent<'a> {
    pub local_xy: &'a dyn Fn() -> Xy<Px>,
    pub is_local_xy_in: &'a dyn Fn() -> bool,
    pub global_xy: Xy<Px>,
    pub pressing_buttons: &'a HashSet<MouseButton>,
    pub button: Option<MouseButton>,
    pub event_type: MouseEventType,
    pub is_stop_event_propagation: &'a AtomicBool,
}
impl Debug for MouseEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MouseEvent")
            .field("global_xy", &self.global_xy)
            .field("pressing_buttons", &self.pressing_buttons)
            .field("button", &self.button)
            .field("event_type", &self.event_type)
            .field("is_stop_event_propagation", &self.is_stop_event_propagation)
            .finish()
    }
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
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
}
pub struct WheelEvent<'a> {
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
    pub delta_xy: Xy<f32>,
    pub is_local_xy_in: &'a dyn Fn() -> bool,
    pub local_xy: &'a dyn Fn() -> Xy<Px>,
    pub is_stop_event_propagation: &'a AtomicBool,
}
impl Debug for WheelEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WheelEvent")
            .field("delta_xy", &self.delta_xy)
            .field("is_stop_event_propagation", &self.is_stop_event_propagation)
            .finish()
    }
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

#[derive(Debug)]
pub struct KeyboardEvent<'a> {
    pub code: Code,
    pub pressing_codes: &'a HashSet<Code>,
    pub is_stop_event_propagation: &'a AtomicBool,
}
impl EventExt for KeyboardEvent<'_> {
    fn stop_propagation(&self) {
        self.is_stop_event_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
