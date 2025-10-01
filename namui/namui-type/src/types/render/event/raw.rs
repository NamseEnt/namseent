use super::*;
use derive_more::Debug;

#[derive(Debug, bincode::Decode, bincode::Encode, Clone)]
pub enum RawEvent {
    MouseDown { event: RawMouseEvent },
    MouseMove { event: RawMouseEvent },
    MouseUp { event: RawMouseEvent },
    Wheel { event: RawWheelEvent },
    KeyDown { event: RawKeyboardEvent },
    KeyUp { event: RawKeyboardEvent },
    Blur,
    VisibilityChange,
    ScreenResize { wh: Wh<IntPx> },
    ScreenRedraw,
    TextInput { event: RawTextInputEvent },
    TextInputKeyDown { event: RawTextInputKeyDownEvent },
    TextInputSelectionChange { event: RawTextInputEvent },
}

#[derive(Debug, bincode::Decode, bincode::Encode, Clone)]
pub struct RawMouseEvent {
    pub xy: Xy<Px>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
}

#[derive(Debug, bincode::Decode, bincode::Encode, Clone)]
pub struct RawWheelEvent {
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
    pub delta_xy: Xy<f32>,
    pub mouse_xy: Xy<Px>,
}

#[derive(Debug, bincode::Decode, bincode::Encode, Clone)]
pub struct RawKeyboardEvent {
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
}

#[derive(Debug, bincode::Decode, bincode::Encode, Clone)]
pub struct RawTextInputEvent {
    pub text: String,
    pub selection_direction: SelectionDirection,
    pub selection_start: usize,
    pub selection_end: usize,
}

#[derive(Debug, bincode::Decode, bincode::Encode, Clone)]
pub struct RawTextInputKeyDownEvent {
    pub text: String,
    pub selection_direction: SelectionDirection,
    pub selection_start: usize,
    pub selection_end: usize,
    pub code: Code,
}

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Copy)]
pub enum SelectionDirection {
    None = 0,
    Forward,
    Backward,
}
