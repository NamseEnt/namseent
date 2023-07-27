use super::SpecialRenderingNode;
use crate::{closure::ClosurePtr, *};
use serde::Serialize;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Serialize, Clone, PartialEq)]
pub struct AttachEventNode {
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
    #[serde(skip_serializing)]
    pub on_mouse_move_in: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_move_out: Option<MouseEventCallback>,
    // onMouseIn?: () => void;
    #[serde(skip_serializing)]
    pub on_mouse_down_in: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_down_out: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_up_in: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_up_out: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_wheel: Option<WheelEventCallback>,
    #[serde(skip_serializing)]
    pub on_key_down: Option<KeyboardEventCallback>,
    #[serde(skip_serializing)]
    pub on_key_up: Option<KeyboardEventCallback>,
    #[serde(skip_serializing)]
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

pub type MouseEventCallback = ClosurePtr<MouseEvent, ()>;
pub type WheelEventCallback = ClosurePtr<WheelEvent, ()>;
pub type KeyboardEventCallback = ClosurePtr<KeyboardEvent, ()>;
pub type FileDropEventCallback = ClosurePtr<FileDropEvent, ()>;

impl std::fmt::Debug for AttachEventNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttachEventNode")
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

#[derive(Default, Clone, Debug)]
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
    pub fn on_mouse_move_in(
        &mut self,
        on_mouse_move_in: impl Into<ClosurePtr<MouseEvent, ()>>,
    ) -> &mut Self {
        self.on_mouse_move_in = Some(on_mouse_move_in.into());
        self
    }

    pub fn on_mouse_move_out(
        &mut self,
        on_mouse_move_out: impl Into<ClosurePtr<MouseEvent, ()>>,
    ) -> &mut Self {
        self.on_mouse_move_out = Some(on_mouse_move_out.into());
        self
    }

    pub fn on_mouse_down_in(
        &mut self,
        on_mouse_down_in: impl Into<ClosurePtr<MouseEvent, ()>> + 'static,
    ) -> &mut Self {
        self.on_mouse_down_in = Some(on_mouse_down_in.into());
        self
    }

    pub fn on_mouse_down_out(
        &mut self,
        on_mouse_down_out: impl Into<ClosurePtr<MouseEvent, ()>>,
    ) -> &mut Self {
        self.on_mouse_down_out = Some(on_mouse_down_out.into());
        self
    }

    pub fn on_mouse_up_in(
        &mut self,
        on_mouse_up_in: impl Into<ClosurePtr<MouseEvent, ()>>,
    ) -> &mut Self {
        self.on_mouse_up_in = Some(on_mouse_up_in.into());
        self
    }

    pub fn on_mouse_up_out(
        &mut self,
        on_mouse_up_out: impl Into<ClosurePtr<MouseEvent, ()>>,
    ) -> &mut Self {
        self.on_mouse_up_out = Some(on_mouse_up_out.into());
        self
    }

    pub fn on_mouse(&mut self, on_mouse: impl Into<ClosurePtr<MouseEvent, ()>>) -> &mut Self {
        self.on_mouse = Some(on_mouse.into());
        self
    }

    pub fn on_wheel(&mut self, on_wheel: impl Into<ClosurePtr<WheelEvent, ()>>) -> &mut Self {
        self.on_wheel = Some(on_wheel.into());
        self
    }

    pub fn on_key_down(
        &mut self,
        on_key_down: impl Into<ClosurePtr<KeyboardEvent, ()>>,
    ) -> &mut Self {
        self.on_key_down = Some(on_key_down.into());
        self
    }

    pub fn on_key_up(&mut self, on_key_up: impl Into<ClosurePtr<KeyboardEvent, ()>>) -> &mut Self {
        self.on_key_up = Some(on_key_up.into());
        self
    }

    pub fn on_file_drop(
        &mut self,
        on_file_drop: impl Into<ClosurePtr<FileDropEvent, ()>>,
    ) -> &mut Self {
        self.on_file_drop = Some(on_file_drop.into());
        self
    }
}
