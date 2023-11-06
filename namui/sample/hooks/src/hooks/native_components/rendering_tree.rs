use crate::hooks::component::{Component, ComponentProps, WireClosures};
use namui::RenderingTree;

impl ComponentProps for RenderingTree {
    fn render(&self, render: crate::hooks::render::Render) -> crate::hooks::render::Render {
        render
    }
}

impl WireClosures for RenderingTree {
    fn wire_closures(&self, _to: &dyn Component) {}
}

impl Component for RenderingTree {}
