use super::SpecialRenderingNode;
use crate::{Code, MouseButton, NamuiContext, RenderingTree, Xy};
use serde::Serialize;
use std::{collections::HashSet, sync::Arc};

#[derive(Serialize, Clone)]
pub struct AttachEventNode {
    pub(crate) rendering_tree: Box<RenderingTree>,
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
    #[serde(skip_serializing)]
    pub on_key_down: Option<KeyboardEventCallback>,
    #[serde(skip_serializing)]
    pub on_key_up: Option<KeyboardEventCallback>,
}

#[derive(Clone)]
pub struct MouseEvent<'a> {
    pub id: String,
    pub namui_context: &'a NamuiContext,
    pub target: &'a RenderingTree,
    pub local_xy: Xy<f32>,
    pub global_xy: Xy<f32>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
}
pub enum MouseEventType {
    Down,
    Up,
    Move,
}
pub struct WheelEvent<'a> {
    pub id: String,
    pub namui_context: &'a NamuiContext,
    pub target: &'a RenderingTree,
    pub delta_xy: Xy<f32>,
}
pub struct KeyboardEvent<'a> {
    pub id: String,
    pub namui_context: &'a NamuiContext,
    pub target: &'a RenderingTree,
    pub code: Code,
    pub pressing_codes: HashSet<Code>,
}
pub type MouseEventCallback = Arc<dyn Fn(&MouseEvent)>;
pub type WheelEventCallback = Arc<dyn Fn(&WheelEvent)>;
pub type KeyboardEventCallback = Arc<dyn Fn(&KeyboardEvent)>;

impl std::fmt::Debug for AttachEventNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rendering_tree: {:?}, on_mouse_move_in: {:?}, on_mouse_move_out: {:?}, on_mouse_down: {:?}, on_mouse_up: {:?}, on_wheel: {:?}", self.rendering_tree, self.on_mouse_move_in.is_some(), self.on_mouse_move_out.is_some(), self.on_mouse_down.is_some(), self.on_mouse_up.is_some(), self.on_wheel.is_some())
    }
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
    pub(crate) on_key_down: Option<KeyboardEventCallback>,
    pub(crate) on_key_up: Option<KeyboardEventCallback>,
}

#[derive(Default)]
pub struct AttachEventBuilder2 {
    pub(crate) on_mouse_move_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_move_out: Option<MouseEventCallback>,
    // onClickOut: Option<MouseEventCallback>,
    // onMouseIn?: () => void;
    pub(crate) on_mouse_down: Option<MouseEventCallback>,
    pub(crate) on_mouse_up: Option<MouseEventCallback>,
    pub(crate) on_wheel: Option<WheelEventCallback>,
    pub(crate) on_key_down: Option<KeyboardEventCallback>,
    pub(crate) on_key_up: Option<KeyboardEventCallback>,
}

impl RenderingTree {
    pub fn attach_event(
        self,
        attach_event_build: impl Fn(AttachEventBuilder) -> AttachEventBuilder,
    ) -> RenderingTree {
        let builder = AttachEventBuilder {
            ..Default::default()
        };
        let builder = attach_event_build(builder);
        RenderingTree::Special(SpecialRenderingNode::AttachEvent(AttachEventNode {
            rendering_tree: Box::new(self),
            on_mouse_move_in: builder.on_mouse_move_in,
            on_mouse_move_out: builder.on_mouse_move_out,
            on_mouse_down: builder.on_mouse_down,
            on_mouse_up: builder.on_mouse_up,
            on_wheel: builder.on_wheel,
            on_key_down: builder.on_key_down,
            on_key_up: builder.on_key_up,
        }))
    }

    pub fn attach_event2(
        self,
        attach_event_build: impl Fn(&mut AttachEventBuilder2),
    ) -> RenderingTree {
        let mut builder = AttachEventBuilder2 {
            ..Default::default()
        };
        attach_event_build(&mut builder);
        RenderingTree::Special(SpecialRenderingNode::AttachEvent(AttachEventNode {
            rendering_tree: Box::new(self),
            on_mouse_move_in: builder.on_mouse_move_in,
            on_mouse_move_out: builder.on_mouse_move_out,
            on_mouse_down: builder.on_mouse_down,
            on_mouse_up: builder.on_mouse_up,
            on_wheel: builder.on_wheel,
            on_key_down: builder.on_key_down,
            on_key_up: builder.on_key_up,
        }))
    }
}

impl AttachEventBuilder {
    pub fn on_mouse_move_in(mut self, on_mouse_move_in: impl Fn(&MouseEvent) + 'static) -> Self {
        self.on_mouse_move_in = Some(Arc::new(on_mouse_move_in));
        self
    }

    pub fn on_mouse_move_out(mut self, on_mouse_move_out: impl Fn(&MouseEvent) + 'static) -> Self {
        self.on_mouse_move_out = Some(Arc::new(on_mouse_move_out));
        self
    }

    pub fn on_mouse_down(mut self, on_mouse_down: impl Fn(&MouseEvent) + 'static) -> Self {
        self.on_mouse_down = Some(Arc::new(on_mouse_down));
        self
    }

    pub fn on_mouse_up(mut self, on_mouse_up: impl Fn(&MouseEvent) + 'static) -> Self {
        self.on_mouse_up = Some(Arc::new(on_mouse_up));
        self
    }

    pub fn on_wheel(mut self, on_wheel: impl Fn(&WheelEvent) + 'static) -> Self {
        self.on_wheel = Some(Arc::new(on_wheel));
        self
    }

    pub fn on_key_down(mut self, on_key_down: impl Fn(&KeyboardEvent) + 'static) -> Self {
        self.on_key_down = Some(Arc::new(on_key_down));
        self
    }

    pub fn on_key_up(mut self, on_key_up: impl Fn(&KeyboardEvent) + 'static) -> Self {
        self.on_key_up = Some(Arc::new(on_key_up));
        self
    }
}

impl AttachEventBuilder2 {
    pub fn on_mouse_move_in(
        &mut self,
        on_mouse_move_in: impl Fn(&MouseEvent) + 'static,
    ) -> &mut Self {
        self.on_mouse_move_in = Some(Arc::new(on_mouse_move_in));
        self
    }

    pub fn on_mouse_move_out(
        &mut self,
        on_mouse_move_out: impl Fn(&MouseEvent) + 'static,
    ) -> &mut Self {
        self.on_mouse_move_out = Some(Arc::new(on_mouse_move_out));
        self
    }

    pub fn on_mouse_down(&mut self, on_mouse_down: impl Fn(&MouseEvent) + 'static) -> &mut Self {
        self.on_mouse_down = Some(Arc::new(on_mouse_down));
        self
    }

    pub fn on_mouse_up(&mut self, on_mouse_up: impl Fn(&MouseEvent) + 'static) -> &mut Self {
        self.on_mouse_up = Some(Arc::new(on_mouse_up));
        self
    }

    pub fn on_wheel(&mut self, on_wheel: impl Fn(&WheelEvent) + 'static) -> &mut Self {
        self.on_wheel = Some(Arc::new(on_wheel));
        self
    }

    pub fn on_key_down(&mut self, on_key_down: impl Fn(&KeyboardEvent) + 'static) -> &mut Self {
        self.on_key_down = Some(Arc::new(on_key_down));
        self
    }

    pub fn on_key_up(&mut self, on_key_up: impl Fn(&KeyboardEvent) + 'static) -> &mut Self {
        self.on_key_up = Some(Arc::new(on_key_up));
        self
    }
}
