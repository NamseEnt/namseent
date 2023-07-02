use crate::hooks::{
    component::{Component, ComponentProps, WireClosures},
    render::Render,
};
use namui::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Button {
    pub text: String,
    pub on_click: ClosurePtr<MouseEvent, ()>,
}

impl ComponentProps for Button {
    fn render(&self, render: Render) -> Render {
        render
    }
}

impl WireClosures for Button {
    fn wire_closures(&self, to: &dyn Component) {
        if let Some(to) = to.as_any().downcast_ref::<Self>() {
            self.on_click.wire_to(&to.on_click);
        };
    }
}

impl Component for Button {}
