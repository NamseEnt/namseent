use super::*;
mod pixel_size;

pub(super) trait RenderGraph {
    fn render(&self, wh: Wh<f32>) -> RenderingTree;
    fn render_x_axis_guide_lines(&self, wh: Wh<f32>) -> RenderingTree;
    fn render_mouse_guide(&self, wh: Wh<f32>) -> RenderingTree;
}
