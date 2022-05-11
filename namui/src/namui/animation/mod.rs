use crate::{types::*, RenderingTree};
mod keyframe;
pub use self::keyframe::*;
mod animatable_image;
pub use self::animatable_image::AnimatableImage;

pub struct Layer {
    pub name: String,
    pub image: AnimatableImage,
}

pub trait Animate {
    fn render(&self, time: &Time) -> RenderingTree;
}
