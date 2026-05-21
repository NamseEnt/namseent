mod image;
mod path;
mod text;

pub use image::*;
pub use path::*;
pub use text::*;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, bincode::Encode)]
pub enum DrawCommand {
    Path { command: &'static PathDrawCommand },
    Text { command: &'static TextDrawCommand },
    Image { command: &'static ImageDrawCommand },
}
