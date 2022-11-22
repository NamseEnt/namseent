mod image_cell;
mod text_cell;

use super::*;
pub use image_cell::*;
pub use text_cell::*;

pub trait Cell {
    fn render(&self, props: Props) -> RenderingTree;
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub is_editing: bool,
    pub is_selected: bool,
    pub text_input: &'a TextInput,
}

pub struct EmptyCell {
    on_edit: Option<Box<dyn Fn()>>,
}
pub fn empty() -> EmptyCell {
    EmptyCell { on_edit: None }
}
impl Cell for EmptyCell {
    fn render(&self, _props: Props) -> RenderingTree {
        RenderingTree::Empty
    }
}
impl Into<Box<dyn Cell>> for EmptyCell {
    fn into(self) -> Box<dyn Cell> {
        Box::new(self)
    }
}
