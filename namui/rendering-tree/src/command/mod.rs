mod atlas;
mod image;
mod path;
mod text;

use crate::*;
pub use atlas::*;
pub use image::*;
pub use path::*;
pub use text::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, State)]
pub enum DrawCommand {
    Path { command: Box<PathDrawCommand> },
    Text { command: Box<TextDrawCommand> },
    Image { command: Box<ImageDrawCommand> },
    Atlas { command: Box<AtlasDrawCommand> },
}
