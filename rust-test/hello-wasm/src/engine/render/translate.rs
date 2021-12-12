use serde::Serialize;

use super::RenderingTree;

#[derive(Serialize)]
pub struct Translate {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rendering_tree: Vec<RenderingTree>,
}

pub fn translate(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(Translate {
        x,
        y,
        rendering_tree: vec![rendering_tree],
    })
}
