use super::*;
mod f32_based;

pub(super) trait RenderGraph {
    fn render(&self, wh: Wh<Px>) -> RenderingTree;
    fn render_x_axis_guide_lines(&self, wh: Wh<Px>) -> RenderingTree;
    fn render_mouse_guide(&self, wh: Wh<Px>) -> RenderingTree;
    fn render_point_and_lines(&self, wh: Wh<Px>) -> RenderingTree;
}
