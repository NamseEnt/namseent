mod raw;

use super::*;
use derivative::Derivative;
pub use raw::*;
use std::fmt::Debug;

#[derive(Derivative)]
#[derivative(Debug)]
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
        event: WheelEvent<'a>,
    },
    DragAndDrop {
        event: FileDropEvent<'a>,
    },
    KeyDown {
        event: KeyboardEvent<'a>,
    },
    KeyUp {
        event: KeyboardEvent<'a>,
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
        text: &'a str,
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
    },
    TextInputKeyDown {
        event: TextinputKeyDownEvent<'a>,
    },
}

pub trait EventExt {
    fn stop_propagation(&self) {
        crate::hooks::stop_event_propagation();
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct TextinputKeyDownEvent<'a> {
    pub code: Code,
    pub text: &'a str,
    pub selection_direction: SelectionDirection,
    pub selection_start: usize,
    pub selection_end: usize,
    pub is_composing: bool,
    #[derivative(Debug = "ignore")]
    pub(crate) prevent_default: &'a Box<dyn Fn()>,
}
impl EventExt for TextinputKeyDownEvent<'_> {}
impl TextinputKeyDownEvent<'_> {
    pub fn prevent_default(&self) {
        (self.prevent_default)();
    }
}

#[derive(Debug)]
pub struct DeepLinkOpenedEvent {
    pub url: String,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MouseEvent<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) local_xy: Box<dyn 'a + Fn() -> Xy<Px>>,
    #[derivative(Debug = "ignore")]
    pub(crate) is_local_xy_in: Box<dyn 'a + Fn() -> bool>,
    pub global_xy: Xy<Px>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
    pub event_type: MouseEventType,
    #[derivative(Debug = "ignore")]
    pub(crate) prevent_default: &'a Box<dyn Fn()>,
}
impl EventExt for MouseEvent<'_> {}
impl MouseEvent<'_> {
    pub fn local_xy(&self) -> Xy<Px> {
        (self.local_xy)()
    }
    pub fn is_local_xy_in(&self) -> bool {
        (self.is_local_xy_in)()
    }
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
    pub(crate) is_local_xy_in: Box<dyn 'a + Fn() -> bool>,
    #[derivative(Debug = "ignore")]
    pub(crate) local_xy: Box<dyn 'a + Fn() -> Xy<Px>>,
}
impl EventExt for WheelEvent<'_> {}
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
    #[derivative(Debug = "ignore")]
    pub(crate) prevent_default: &'a Box<dyn Fn()>,
}
impl EventExt for KeyboardEvent<'_> {}
impl KeyboardEvent<'_> {
    pub fn prevent_default(&self) {
        (self.prevent_default)();
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct FileDropEvent<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) is_local_xy_in: Box<dyn 'a + Fn() -> bool>,
    #[derivative(Debug = "ignore")]
    pub(crate) local_xy: Box<dyn 'a + Fn() -> Xy<Px>>,
    pub global_xy: Xy<Px>,
    pub files: Vec<File>,
}
impl EventExt for FileDropEvent<'_> {}
impl FileDropEvent<'_> {
    pub fn local_xy(&self) -> Xy<Px> {
        (self.local_xy)()
    }
    pub fn is_local_xy_in(&self) -> bool {
        (self.is_local_xy_in)()
    }
}
