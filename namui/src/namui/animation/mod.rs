mod animatable_image;
mod image_keyframe_graph;
mod keyframe;

use crate::{types::*, RenderingTree};
pub use animatable_image::AnimatableImage;
pub use image_keyframe_graph::*;
pub use keyframe::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub image: AnimatableImage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub id: String,
    pub layers: Vec<Layer>,
}

pub trait Animate {
    fn render(&self, time: Time) -> RenderingTree;
}

impl Animate for Animation {
    fn render(&self, time: Time) -> RenderingTree {
        crate::render(self.layers.iter().map(|layer| layer.image.render(time)))
    }
}
