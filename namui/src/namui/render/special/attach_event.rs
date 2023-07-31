use super::SpecialRenderingNode;
use crate::*;
use std::{
    collections::HashSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Clone, serde::Serialize)]
pub struct AttachEventNode {
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
    #[serde(skip)]
    pub on_mouse_move_in: Option<MouseEventCallback>,
    #[serde(skip)]
    pub on_mouse_move_out: Option<MouseEventCallback>,
    #[serde(skip)]
    pub on_mouse_down_in: Option<MouseEventCallback>,
    #[serde(skip)]
    pub on_mouse_down_out: Option<MouseEventCallback>,
    #[serde(skip)]
    pub on_mouse_up_in: Option<MouseEventCallback>,
    #[serde(skip)]
    pub on_mouse_up_out: Option<MouseEventCallback>,
    #[serde(skip)]
    pub on_mouse: Option<MouseEventCallback>,
    #[serde(skip)]
    pub on_wheel: Option<WheelEventCallback>,
    #[serde(skip)]
    pub on_key_down: Option<KeyboardEventCallback>,
    #[serde(skip)]
    pub on_key_up: Option<KeyboardEventCallback>,
    #[serde(skip)]
    pub on_file_drop: Option<FileDropEventCallback>,
}

#[derive(Clone)]
pub struct MouseEvent {
    pub id: crate::Uuid,
    pub local_xy: Xy<Px>,
    pub global_xy: Xy<Px>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
    pub event_type: MouseEventType,
    pub(crate) is_stop_propagation: Arc<AtomicBool>,
}
impl MouseEvent {
    pub fn stop_propagation(&self) {
        self.is_stop_propagation.store(true, Ordering::Relaxed);
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
}
pub struct WheelEvent {
    pub id: crate::Uuid,
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
    pub id: crate::Uuid,
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

pub type MouseEventCallback = Arc<dyn Fn(MouseEvent)>;
pub type WheelEventCallback = Arc<dyn Fn(WheelEvent)>;
pub type KeyboardEventCallback = Arc<dyn Fn(KeyboardEvent)>;
pub type FileDropEventCallback = Arc<dyn Fn(FileDropEvent)>;

impl std::fmt::Debug for AttachEventNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttachEventNode")
            .field("rendering_tree", &self.rendering_tree)
            .field("on_mouse_move_in", &self.on_mouse_move_in.is_some())
            .field("on_mouse_move_out", &self.on_mouse_move_out.is_some())
            .field("on_mouse_down_in", &self.on_mouse_down_in.is_some())
            .field("on_mouse_down_out", &self.on_mouse_down_out.is_some())
            .field("on_mouse_up_in", &self.on_mouse_up_in.is_some())
            .field("on_mouse_up_out", &self.on_mouse_up_out.is_some())
            .field("on_mouse", &self.on_mouse.is_some())
            .field("on_wheel", &self.on_wheel.is_some())
            .field("on_key_down", &self.on_key_down.is_some())
            .field("on_key_up", &self.on_key_up.is_some())
            .finish()
    }
}

#[derive(Default, Clone)]
pub struct AttachEventBuilder {
    pub(crate) on_mouse_move_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_move_out: Option<MouseEventCallback>,
    pub(crate) on_mouse_down_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_down_out: Option<MouseEventCallback>,
    pub(crate) on_mouse_up_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_up_out: Option<MouseEventCallback>,
    pub(crate) on_mouse: Option<MouseEventCallback>,
    pub(crate) on_wheel: Option<WheelEventCallback>,
    pub(crate) on_key_down: Option<KeyboardEventCallback>,
    pub(crate) on_key_up: Option<KeyboardEventCallback>,
    pub(crate) on_file_drop: Option<FileDropEventCallback>,
}

impl Debug for AttachEventBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttachEventBuilder")
            .field("on_mouse_move_in", &self.on_mouse_move_in.is_some())
            .field("on_mouse_move_out", &self.on_mouse_move_out.is_some())
            .field("on_mouse_down_in", &self.on_mouse_down_in.is_some())
            .field("on_mouse_down_out", &self.on_mouse_down_out.is_some())
            .field("on_mouse_up_in", &self.on_mouse_up_in.is_some())
            .field("on_mouse_up_out", &self.on_mouse_up_out.is_some())
            .field("on_mouse", &self.on_mouse.is_some())
            .field("on_wheel", &self.on_wheel.is_some())
            .field("on_key_down", &self.on_key_down.is_some())
            .field("on_key_up", &self.on_key_up.is_some())
            .field("on_file_drop", &self.on_file_drop.is_some())
            .finish()
    }
}

impl RenderingTree {
    pub fn attach_event(self, attach_event: impl FnOnce(&mut AttachEventBuilder)) -> RenderingTree {
        let mut builder = AttachEventBuilder {
            ..Default::default()
        };
        attach_event(&mut builder);
        RenderingTree::Special(SpecialRenderingNode::AttachEvent(AttachEventNode {
            rendering_tree: std::sync::Arc::new(self),
            on_mouse_move_in: builder.on_mouse_move_in,
            on_mouse_move_out: builder.on_mouse_move_out,
            on_mouse_down_in: builder.on_mouse_down_in,
            on_mouse_down_out: builder.on_mouse_down_out,
            on_mouse_up_in: builder.on_mouse_up_in,
            on_mouse_up_out: builder.on_mouse_up_out,
            on_mouse: builder.on_mouse,
            on_wheel: builder.on_wheel,
            on_key_down: builder.on_key_down,
            on_key_up: builder.on_key_up,
            on_file_drop: builder.on_file_drop,
        }))
    }
}

impl AttachEventBuilder {
    pub fn on_mouse_move_in(&mut self, on_mouse_move_in: impl Fn(MouseEvent)) -> &mut Self {
        let func = on_mouse_move_in;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(MouseEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_mouse_move_in = Some(func);
        self
    }

    pub fn on_mouse_move_out(&mut self, on_mouse_move_out: impl Fn(MouseEvent)) -> &mut Self {
        let func = on_mouse_move_out;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(MouseEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_mouse_move_out = Some(func);
        self
    }

    pub fn on_mouse_down_in(&mut self, on_mouse_down_in: impl Fn(MouseEvent)) -> &mut Self {
        let func = on_mouse_down_in;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(MouseEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_mouse_down_in = Some(func);
        self
    }

    pub fn on_mouse_down_out(&mut self, on_mouse_down_out: impl Fn(MouseEvent)) -> &mut Self {
        let func = on_mouse_down_out;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(MouseEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_mouse_down_out = Some(func);
        self
    }

    pub fn on_mouse_up_in(&mut self, on_mouse_up_in: impl Fn(MouseEvent)) -> &mut Self {
        let func = on_mouse_up_in;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(MouseEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_mouse_up_in = Some(func);
        self
    }

    pub fn on_mouse_up_out(&mut self, on_mouse_up_out: impl Fn(MouseEvent)) -> &mut Self {
        let func = on_mouse_up_out;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(MouseEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_mouse_up_out = Some(func);
        self
    }

    pub fn on_mouse(&mut self, on_mouse: impl Fn(MouseEvent)) -> &mut Self {
        let func = on_mouse;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(MouseEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_mouse = Some(func);
        self
    }

    pub fn on_wheel(&mut self, on_wheel: impl Fn(WheelEvent)) -> &mut Self {
        let func = on_wheel;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(WheelEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_wheel = Some(func);
        self
    }

    pub fn on_key_down(&mut self, on_key_down: impl Fn(KeyboardEvent)) -> &mut Self {
        let func = on_key_down;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(KeyboardEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_key_down = Some(func);
        self
    }

    pub fn on_key_up(&mut self, on_key_up: impl Fn(KeyboardEvent)) -> &mut Self {
        let func = on_key_up;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(KeyboardEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_key_up = Some(func);
        self
    }

    pub fn on_file_drop(&mut self, on_file_drop: impl Fn(FileDropEvent)) -> &mut Self {
        let func = on_file_drop;
        let func =
            unsafe { std::mem::transmute::<Arc<dyn Fn(FileDropEvent)>, Arc<_>>(Arc::new(func)) };
        self.on_file_drop = Some(func);
        self
    }
}
