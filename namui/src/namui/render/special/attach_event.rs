use super::SpecialRenderingNode;
use crate::*;
use serde::Serialize;
use std::{collections::HashSet, ops::ControlFlow, sync::Arc};

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
    pub on_mouse_down_in: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_down_out: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_up_in: Option<MouseEventCallback>,
    #[serde(skip_serializing)]
    pub on_mouse_up_out: Option<MouseEventCallback>,
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
    pub local_xy: Xy<Px>,
    pub global_xy: Xy<Px>,
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
    /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
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
        write!(f, "rendering_tree: {:?}, on_mouse_move_in: {:?}, on_mouse_move_out: {:?}, on_mouse_down_in: {:?}, on_mouse_up_in: {:?}, on_wheel: {:?}", self.rendering_tree, self.on_mouse_move_in.is_some(), self.on_mouse_move_out.is_some(), self.on_mouse_down_in.is_some(), self.on_mouse_up_in.is_some(), self.on_wheel.is_some())
    }
}

#[derive(Default)]
pub struct AttachEventBuilder {
    pub(crate) on_mouse_move_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_move_out: Option<MouseEventCallback>,
    // onClickOut: Option<MouseEventCallback>,
    // onMouseIn?: () => void;
    pub(crate) on_mouse_down_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_down_out: Option<MouseEventCallback>,
    pub(crate) on_mouse_up_in: Option<MouseEventCallback>,
    pub(crate) on_mouse_up_out: Option<MouseEventCallback>,
    pub(crate) on_wheel: Option<WheelEventCallback>,
    pub(crate) on_key_down: Option<KeyboardEventCallback>,
    pub(crate) on_key_up: Option<KeyboardEventCallback>,
}

impl RenderingTree {
    pub fn attach_event(
        self,
        attach_event_build: impl Fn(&mut AttachEventBuilder),
    ) -> RenderingTree {
        let mut builder = AttachEventBuilder {
            ..Default::default()
        };
        attach_event_build(&mut builder);
        RenderingTree::Special(SpecialRenderingNode::AttachEvent(AttachEventNode {
            rendering_tree: Box::new(self),
            on_mouse_move_in: builder.on_mouse_move_in,
            on_mouse_move_out: builder.on_mouse_move_out,
            on_mouse_down_in: builder.on_mouse_down_in,
            on_mouse_down_out: builder.on_mouse_down_out,
            on_mouse_up_in: builder.on_mouse_up_in,
            on_mouse_up_out: builder.on_mouse_up_out,
            on_wheel: builder.on_wheel,
            on_key_down: builder.on_key_down,
            on_key_up: builder.on_key_up,
        }))
    }
}

impl AttachEventBuilder {
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

    pub fn on_mouse_down_in(
        &mut self,
        on_mouse_down_in: impl Fn(&MouseEvent) + 'static,
    ) -> &mut Self {
        self.on_mouse_down_in = Some(Arc::new(on_mouse_down_in));
        self
    }
    pub fn on_mouse_down_out(
        &mut self,
        on_mouse_down_out: impl Fn(&MouseEvent) + 'static,
    ) -> &mut Self {
        self.on_mouse_down_out = Some(Arc::new(on_mouse_down_out));
        self
    }

    pub fn on_mouse_up_in(&mut self, on_mouse_up_in: impl Fn(&MouseEvent) + 'static) -> &mut Self {
        self.on_mouse_up_in = Some(Arc::new(on_mouse_up_in));
        self
    }
    pub fn on_mouse_up_out(
        &mut self,
        on_mouse_up_out: impl Fn(&MouseEvent) + 'static,
    ) -> &mut Self {
        self.on_mouse_up_out = Some(Arc::new(on_mouse_up_out));
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

impl WheelEvent<'_> {
    pub fn is_mouse_in(&self) -> bool {
        let mut result = false;
        self.namui_context.rendering_tree.visit_rln(|node, utils| {
            if std::ptr::eq(node, self.target) {
                result = utils.is_xy_in(system::mouse::position());
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });

        result
    }
}
