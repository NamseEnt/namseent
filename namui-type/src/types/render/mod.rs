mod command;
mod font;
mod paint;
mod paragraph;
mod path;
mod rendering_tree;
mod types;
mod image;

pub use image::*;
pub use command::*;
pub use font::*;
pub use paint::*;
pub use paragraph::*;
pub use path::*;
pub use rendering_tree::*;
pub use types::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DrawInput {
    pub rendering_tree: RenderingTree,
}

impl DrawInput {
    pub fn to_vec(&self) -> Vec<u8> {
        postcard::to_allocvec(self).unwrap()
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        postcard::from_bytes(bytes).unwrap()
    }
}
