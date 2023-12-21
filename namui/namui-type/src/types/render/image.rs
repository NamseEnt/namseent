use crate::*;
use std::{fmt::Debug, sync::Arc};

#[type_derives(-serde::Serialize, -serde::Deserialize)]
pub struct Image {
    pub wh: Wh<Px>,
    pub src: ImageSource,
}

#[type_derives(Copy)]
pub struct ImageInfo {
    pub alpha_type: AlphaType,
    pub color_type: ColorType,
    pub height: Px,
    pub width: Px,
}

#[cfg(feature = "skia")]
impl Into<skia_safe::ImageInfo> for ImageInfo {
    fn into(self) -> skia_safe::ImageInfo {
        skia_safe::ImageInfo::new(
            skia_safe::ISize {
                width: self.width.as_f32() as i32,
                height: self.height.as_f32() as i32,
            },
            self.color_type.into(),
            self.alpha_type.into(),
            None,
        )
    }
}

#[type_derives(-serde::Deserialize, -PartialEq)]
pub struct ImageHandle {
    pub alpha_type: AlphaType,
    pub color_type: ColorType,
    pub height: Px,
    pub width: Px,
    id: crate::Uuid,
    #[cfg(feature = "skia")]
    #[serde(skip)]
    pub inner: Arc<skia_safe::Image>,
    #[cfg(not(feature = "skia"))]
    #[serde(skip)]
    pub inner: Arc<dyn Send + Sync + Debug>,
}

impl PartialEq for ImageHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ImageHandle {
    #[cfg(feature = "skia")]
    pub fn new(image_info: ImageInfo, id: crate::Uuid, inner: skia_safe::Image) -> Self {
        Self {
            alpha_type: image_info.alpha_type,
            color_type: image_info.color_type,
            height: image_info.height,
            width: image_info.width,
            id,
            inner: Arc::new(inner),
        }
    }
}
