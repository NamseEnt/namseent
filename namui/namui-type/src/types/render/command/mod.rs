mod image;
mod path;
mod text;

use crate::*;
pub use image::*;
pub use path::*;
pub use text::*;

#[type_derives(-serde::Deserialize)]
pub enum DrawCommand {
    Path { command: PathDrawCommand },
    Text { command: TextDrawCommand },
    Image { command: ImageDrawCommand },
}
