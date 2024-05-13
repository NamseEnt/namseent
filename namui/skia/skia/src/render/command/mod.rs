mod image;
mod path;
mod text;

use crate::*;
pub use image::*;
pub use path::*;
pub use text::*;

#[type_derives()]
pub enum DrawCommand {
    Path { command: Box<PathDrawCommand> },
    Text { command: Box<TextDrawCommand> },
    Image { command: Box<ImageDrawCommand> },
}
