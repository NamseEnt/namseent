use super::*;
use std::fmt::Debug;

#[derive(Debug)]
pub enum RawEvent {
    MouseDown {
        event: RawMouseEvent,
    },
    MouseMove {
        event: RawMouseEvent,
    },
    MouseUp {
        event: RawMouseEvent,
    },
    Wheel {
        event: RawWheelEvent,
    },
    FileDrop {
        data_transfer: Option<web_sys::DataTransfer>,
        xy: Xy<Px>,
    },
    SelectionChange {
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
        text: String,
    },
    KeyDown {
        code: Code,
        pressing_code_set: HashSet<Code>,
    },
    KeyUp {
        code: Code,
        pressing_code_set: HashSet<Code>,
    },
    Blur,
    VisibilityChange,
    ScreenResize {
        wh: Wh<IntPx>,
    },
    TextInputTextUpdated {
        text: String,
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
    },
    TextInputKeyDown {
        code: Code,
        text: String,
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
        is_composing: bool,
    },
}

#[derive(Debug)]
pub enum Event<'a> {
    MouseDown {
        event: MouseEvent<'a>,
    },
    MouseMove {
        event: MouseEvent<'a>,
    },
    MouseUp {
        event: MouseEvent<'a>,
    },
    Wheel {
        event: WheelEvent,
    },
    DragAndDrop {
        event: FileDropEvent<'a>,
    },
    KeyDown {
        event: KeyboardEvent,
    },
    KeyUp {
        event: KeyboardEvent,
    },
    Blur,
    VisibilityChange,
    ScreenResize {
        wh: Wh<IntPx>,
    },
    SelectionChange {
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
        text: String,
    },
    TextInputTextUpdated {
        text: String,
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
    },
    TextInputKeyDown {
        code: Code,
        text: String,
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
        is_composing: bool,
    },
}

#[derive(Debug)]
pub struct RawMouseEvent {
    pub xy: Xy<Px>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
}

#[derive(Debug)]
pub struct RawWheelEvent {
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
    pub delta_xy: Xy<f32>,
    pub mouse_xy: Xy<Px>,
}

#[derive(Debug)]
pub struct RawKeyboardEvent {
    pub id: crate::Uuid,
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
}

#[derive(Debug)]
pub struct DeepLinkOpenedEvent {
    pub url: String,
}

pub struct MouseEvent<'a> {
    pub(crate) local_xy: Box<dyn 'a + Fn() -> Xy<Px>>,
    pub(crate) is_local_xy_in: Box<dyn 'a + Fn() -> bool>,
    pub global_xy: Xy<Px>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
    pub event_type: MouseEventType,
    pub(crate) is_stop_propagation: Arc<AtomicBool>,
}
impl Debug for MouseEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MouseEvent")
            .field("global_xy", &self.global_xy)
            .field("pressing_buttons", &self.pressing_buttons)
            .field("button", &self.button)
            .field("event_type", &self.event_type)
            .finish()
    }
}
impl MouseEvent<'_> {
    pub fn stop_propagation(&self) {
        self.is_stop_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
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
#[derive(Debug)]
pub struct WheelEvent {
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
    pub delta_xy: Xy<f32>,
    pub mouse_local_xy: Xy<Px>,
    pub(crate) is_stop_propagation: Arc<AtomicBool>,
}
impl WheelEvent {
    pub fn stop_propagation(&self) {
        self.is_stop_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
#[derive(Debug)]
pub struct KeyboardEvent {
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
}

pub struct FileDropEvent<'a> {
    pub(crate) is_local_xy_in: Box<dyn 'a + Fn() -> bool>,
    pub local_xy: Box<dyn 'a + Fn() -> Xy<Px>>,
    pub global_xy: Xy<Px>,
    pub files: Vec<File>,
    pub(crate) is_stop_propagation: Arc<AtomicBool>,
}
impl Debug for FileDropEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileDropEvent")
            // .field("is_local_xy_in", &self.is_local_xy_in)
            // .field("local_xy", &self.local_xy)
            .field("global_xy", &self.global_xy)
            .field("files", &self.files)
            .field("is_stop_propagation", &self.is_stop_propagation)
            .finish()
    }
}
impl FileDropEvent<'_> {
    pub fn local_xy(&self) -> Xy<Px> {
        (self.local_xy)()
    }
    pub fn is_local_xy_in(&self) -> bool {
        (self.is_local_xy_in)()
    }
    pub fn stop_propagation(&self) {
        self.is_stop_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
