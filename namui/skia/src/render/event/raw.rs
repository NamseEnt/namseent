use super::*;

#[derive(Debug)]
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
}

#[derive(Derivative)]
#[derivative(Debug)]
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

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RawKeyboardEvent {
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
}
