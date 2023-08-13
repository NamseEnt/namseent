use super::*;

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
    HashChange {
        new_url: String,
        old_url: String,
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
    HashChange {
        new_url: String,
        old_url: String,
    },
    DragAndDrop {
        data_transfer: Option<web_sys::DataTransfer>,
        x: usize,
        y: usize,
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
pub struct KeyboardEvent {
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
}
pub struct FileDropEvent {
    pub local_xy: Xy<Px>,
    pub global_xy: Xy<Px>,
    pub files: Vec<File>,
    pub(crate) is_stop_propagation: Arc<AtomicBool>,
}
impl FileDropEvent {
    pub fn stop_propagation(&self) {
        self.is_stop_propagation
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
