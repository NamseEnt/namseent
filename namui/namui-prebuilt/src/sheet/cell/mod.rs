mod border;
mod clipboard_item;
mod empty_cell;
mod image_cell;
mod text_cell;

use super::*;
pub use border::*;
pub use clipboard_item::*;
pub use empty_cell::*;
pub use image_cell::*;
pub use text_cell::*;

pub trait CellTrait {
    fn render(&self, props: Props) -> RenderingTree;
    fn borders(&self) -> &Borders;
    fn copy(&self) -> ClipboardItem;
    fn on_paste(&self) -> Option<Closure<dyn Fn(ClipboardItem)>>;
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub is_editing: bool,
    pub is_selected: bool,
    pub text_input: &'a TextInput,
    pub color_palette: ColorPalette,
}

pub struct Cell {
    pub(crate) inner: Box<dyn CellTrait>,
    pub(crate) on_mouse_down: Option<Closure<dyn Fn(MouseEvent)>>,
}

impl Cell {
    fn new(inner: Box<dyn CellTrait>) -> Self {
        Cell {
            inner,
            on_mouse_down: None,
        }
    }
    pub fn on_mouse_down(mut self, on_mouse_down: Closure<dyn Fn(MouseEvent)>) -> Self {
        self.on_mouse_down = Some(on_mouse_down);
        self
    }
}
