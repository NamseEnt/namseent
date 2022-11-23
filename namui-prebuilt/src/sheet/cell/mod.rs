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
use std::sync::Arc;
pub use text_cell::*;

pub trait Cell {
    fn render(&self, props: Props) -> RenderingTree;
    fn borders(&self) -> &Borders;
    fn copy(&self) -> ClipboardItem;
    fn on_paste(&self) -> Option<Arc<dyn Fn(ClipboardItem)>>;
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub is_editing: bool,
    pub is_selected: bool,
    pub text_input: &'a TextInput,
    pub color_palette: ColorPalette,
}
