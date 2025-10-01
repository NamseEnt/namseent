use super::*;
use crate::*;
use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone)]
pub struct Image {
    pub info: ImageInfo,
    pub skia_image: std::sync::Arc<skia_safe::Image>,
}

impl Image {
    pub fn new(image_info: ImageInfo, image: skia_safe::Image) -> Self {
        Self {
            info: image_info,
            skia_image: std::sync::Arc::new(image),
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
        std::sync::Arc::ptr_eq(&self.skia_image, &other.skia_image) && self.info == other.info
    }
}
impl Eq for Image {}

impl Hash for Image {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.info.hash(state);
        self.skia_image.unique_id().hash(state);
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
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

impl From<ImageInfo> for skia_safe::ImageInfo {
    fn from(val: ImageInfo) -> Self {
        skia_safe::ImageInfo::new(
            skia_safe::ISize {
                width: val.width.as_f32() as i32,
                height: val.height.as_f32() as i32,
            },
            val.color_type.into(),
            val.alpha_type.into(),
            None,
        )
    }
}

impl From<&skia_safe::ImageInfo> for ImageInfo {
    fn from(val: &skia_safe::ImageInfo) -> Self {
        Self {
            alpha_type: val.alpha_type().into(),
            color_type: val.color_type().into(),
            height: val.height().into(),
            width: val.width().into(),
        }
    }
}
