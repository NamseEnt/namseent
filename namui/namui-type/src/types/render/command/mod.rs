mod image;
mod path;
mod text;

pub use image::*;
pub use path::*;
pub use text::*;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Hash, Eq)]
pub enum DrawCommand {
    Path { command: Box<PathDrawCommand> },
    Text { command: Box<TextDrawCommand> },
    Image { command: Box<ImageDrawCommand> },
}
