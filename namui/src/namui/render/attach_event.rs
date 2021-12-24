use std::sync::Arc;

use super::{
    BoxedMouseEventCallback, BoxedWheelEventCallback, RenderingTree, SpecialRenderingNode,
};
use crate::{MouseEventCallback, WheelEventCallback};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AttachEvent {
    pub(crate) rendering_tree: Vec<RenderingTree>,
    #[serde(skip_serializing)]
    pub on_mouse_move_in: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_move_out: Option<MouseEventCallback>,
    // #[serde(skip_serializing)]
    // onClickOut: Option<MouseEventCallback>,
    // onMouseIn?: () => void;
    #[serde(skip_serializing)]
    pub on_mouse_down: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_up: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_wheel: Option<WheelEventCallback>,
}

#[derive(Default)]
pub struct AttachEventBuilder {
    pub(crate) on_mouse_move_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_move_out: Option<MouseEventCallback>,
    // onClickOut: Option<MouseEventCallback>,
    // onMouseIn?: () => void;
    pub(crate) on_mouse_down: Option<MouseEventCallback>,
    pub(crate) on_mouse_up: Option<MouseEventCallback>,
    pub(crate) on_wheel: Option<WheelEventCallback>,
}

impl RenderingTree {
    pub fn attach_event(
        &self,
        attach_event_build: impl Fn(AttachEventBuilder) -> AttachEventBuilder,
    ) -> RenderingTree {
        let builder = AttachEventBuilder {
            ..Default::default()
        };
        let builder = attach_event_build(builder);
        RenderingTree::Special(SpecialRenderingNode::AttachEvent(AttachEvent {
            rendering_tree: vec![self.clone()],
            on_mouse_move_in: builder.on_mouse_move_in,
            on_mouse_move_out: builder.on_mouse_move_out,
            on_mouse_down: builder.on_mouse_down,
            on_mouse_up: builder.on_mouse_up,
            on_wheel: builder.on_wheel,
        }))
    }
}

impl AttachEventBuilder {
    pub fn on_mouse_move_in(mut self, on_mouse_move_in: BoxedMouseEventCallback) -> Self {
        self.on_mouse_move_in = Some(Arc::new(on_mouse_move_in));
        self
    }

    pub fn on_mouse_move_out(mut self, on_mouse_move_out: BoxedMouseEventCallback) -> Self {
        self.on_mouse_move_out = Some(Arc::new(on_mouse_move_out));
        self
    }

    pub fn on_mouse_down(mut self, on_mouse_down: BoxedMouseEventCallback) -> Self {
        self.on_mouse_down = Some(Arc::new(on_mouse_down));
        self
    }

    pub fn on_mouse_up(mut self, on_mouse_up: BoxedMouseEventCallback) -> Self {
        self.on_mouse_up = Some(Arc::new(on_mouse_up));
        self
    }

    pub fn on_wheel(mut self, on_wheel: BoxedWheelEventCallback) -> Self {
        self.on_wheel = Some(Arc::new(on_wheel));
        self
    }
}
