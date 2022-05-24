use crate::{types::*, RenderingTree};
use serde::{Deserialize, Serialize};
mod keyframe;
pub use self::keyframe::*;
mod animatable_image;
pub use self::animatable_image::AnimatableImage;

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
