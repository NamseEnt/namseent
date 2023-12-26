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
    KeyDown {
        event: RawKeyboardEvent,
    },
    KeyUp {
        event: RawKeyboardEvent,
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
        event: RawTextinputKeyDownEvent,
    },
    #[cfg(target_family = "wasm")]
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
    ScreenRedraw,
}
unsafe impl Send for RawEvent {}
unsafe impl Sync for RawEvent {}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RawMouseEvent {
    pub xy: Xy<Px>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
    #[derivative(Debug = "ignore")]
    pub(crate) prevent_default: Box<dyn Fn()>,
}

#[derive(Debug)]
pub struct RawWheelEvent {
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
    pub delta_xy: Xy<f32>,
    pub mouse_xy: Xy<Px>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RawKeyboardEvent {
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
    #[derivative(Debug = "ignore")]
    pub(crate) prevent_default: Box<dyn Fn()>,
}

#[derive(Derivative)]
#[derivative(Debug)]

pub struct RawTextinputKeyDownEvent {
    pub code: Code,
    pub text: String,
    pub selection_direction: SelectionDirection,
    pub selection_start: usize,
    pub selection_end: usize,
    pub is_composing: bool,
    #[derivative(Debug = "ignore")]
    pub(crate) prevent_default: Box<dyn Fn()>,
}
