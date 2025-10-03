use super::*;
use crate::*;
use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Image {
    pub info: ImageInfo,
    // pub skia_image: std::sync::Arc<skia_safe::Image>,
}

impl Image {
    pub fn new(image_info: ImageInfo, // , image: skia_safe::Image
    ) -> Self {
        Self {
            info: image_info,
            // skia_image: std::sync::Arc::new(image),
        }
    }
    #[allow(dead_code)]
    pub(crate) fn get_default_shader(&self) -> Shader {
        Shader::Image {
            src: self.clone(),
            tile_mode: Xy::single(TileMode::Clamp),
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        // std::sync::Arc::ptr_eq(&self.skia_image, &other.skia_image) && self.info == other.info
        todo!()
    }
}
impl Eq for Image {}

impl Hash for Image {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.info.hash(state);
        // self.skia_image.unique_id().hash(state);
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, bincode::Encode, bincode::Decode)]
pub struct ImageInfo {
    pub alpha_type: AlphaType,
    pub color_type: ColorType,
    pub height: Px,
    pub width: Px,
}

impl ImageInfo {
    pub fn wh(&self) -> Wh<Px> {
        Wh {
            width: self.width,
            height: self.height,
        }
    }
}
