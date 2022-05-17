use super::*;

pub trait RectSlice<Props> {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree;
}
pub trait Row<Props> {
    fn get_height(&self, parent_wh: Wh<f32>) -> f32;
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree;
}
pub trait Column<Props> {
    fn get_width(&self, parent_wh: Wh<f32>) -> f32;
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree;
}
pub trait Fill<Props> {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree;
}
