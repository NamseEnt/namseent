use crate::*;
use std::{
    collections::HashSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

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
        self.is_stop_propagation.store(true, Ordering::Relaxed);
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
        self.is_stop_propagation.store(true, Ordering::Relaxed);
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
        self.is_stop_propagation.store(true, Ordering::Relaxed);
    }
}

pub type MouseEventCallback<'a> = Box<dyn 'a + FnOnce(MouseEvent)>;
pub type WheelEventCallback<'a> = Box<dyn 'a + FnOnce(WheelEvent)>;
pub type KeyboardEventCallback<'a> = Box<dyn 'a + FnOnce(KeyboardEvent)>;
pub type FileDropEventCallback<'a> = Box<dyn 'a + FnOnce(FileDropEvent)>;

#[derive(Default)]
pub struct AttachEventBuilder<'a> {
    pub(crate) on_mouse_move_in: Option<MouseEventCallback<'a>>,
    pub(crate) on_mouse_move_out: Option<MouseEventCallback<'a>>,
    pub(crate) on_mouse_down_in: Option<MouseEventCallback<'a>>,
    pub(crate) on_mouse_down_out: Option<MouseEventCallback<'a>>,
    pub(crate) on_mouse_up_in: Option<MouseEventCallback<'a>>,
    pub(crate) on_mouse_up_out: Option<MouseEventCallback<'a>>,
    pub(crate) on_mouse: Option<MouseEventCallback<'a>>,
    pub(crate) on_wheel: Option<WheelEventCallback<'a>>,
    pub(crate) on_key_down: Option<KeyboardEventCallback<'a>>,
    pub(crate) on_key_up: Option<KeyboardEventCallback<'a>>,
    pub(crate) on_file_drop: Option<FileDropEventCallback<'a>>,
}

// impl RenderingTree {
//     pub fn attach_event<'a>(
//         self,
//         attach_event: impl FnOnce(&'a mut AttachEventBuilder<'a>),
//     ) -> RenderingTree {
//         // {
//         //     let mut builder = AttachEventBuilder {
//         //         rendering_tree: &self,
//         //     };
//         //     attach_event(&mut builder);
//         //     drop(builder)
//         // }

//         self
//     }
// }

impl<'a> AttachEventBuilder<'a> {
    pub fn on_mouse_move_in(
        &'a mut self,
        on_mouse_move_in: impl 'a + FnOnce(MouseEvent),
    ) -> &mut Self {
        self.on_mouse_move_in = Some(Box::new(on_mouse_move_in));
        self
    }

    pub fn on_mouse_move_out(
        &'a mut self,
        on_mouse_move_out: impl 'a + FnOnce(MouseEvent),
    ) -> &mut Self {
        self.on_mouse_move_out = Some(Box::new(on_mouse_move_out));
        self
    }

    pub fn on_mouse_down_in(
        &'a mut self,
        on_mouse_down_in: impl 'a + FnOnce(MouseEvent),
    ) -> &mut Self {
        self.on_mouse_down_in = Some(Box::new(on_mouse_down_in));
        self
    }

    pub fn on_mouse_down_out(
        &'a mut self,
        on_mouse_down_out: impl 'a + FnOnce(MouseEvent),
    ) -> &mut Self {
        self.on_mouse_down_out = Some(Box::new(on_mouse_down_out));
        self
    }

    pub fn on_mouse_up_in(&'a mut self, on_mouse_up_in: impl 'a + FnOnce(MouseEvent)) -> &mut Self {
        self.on_mouse_up_in = Some(Box::new(on_mouse_up_in));
        self
    }

    pub fn on_mouse_up_out(
        &'a mut self,
        on_mouse_up_out: impl 'a + FnOnce(MouseEvent),
    ) -> &mut Self {
        self.on_mouse_up_out = Some(Box::new(on_mouse_up_out));
        self
    }

    pub fn on_mouse(&'a mut self, on_mouse: impl 'a + FnOnce(MouseEvent)) -> &mut Self {
        self.on_mouse = Some(Box::new(on_mouse));
        self
    }

    pub fn on_wheel(&'a mut self, on_wheel: impl 'a + FnOnce(WheelEvent)) -> &mut Self {
        self.on_wheel = Some(Box::new(on_wheel));
        self
    }

    pub fn on_key_down(&'a mut self, on_key_down: impl 'a + FnOnce(KeyboardEvent)) -> &mut Self {
        self.on_key_down = Some(Box::new(on_key_down));
        self
    }

    pub fn on_key_up(&'a mut self, on_key_up: impl 'a + FnOnce(KeyboardEvent)) -> &mut Self {
        self.on_key_up = Some(Box::new(on_key_up));
        self
    }

    pub fn on_file_drop(&'a mut self, on_file_drop: impl 'a + FnOnce(FileDropEvent)) -> &mut Self {
        self.on_file_drop = Some(Box::new(on_file_drop));
        self
    }
}
