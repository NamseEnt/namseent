use super::*;
use crate::*;
use std::fmt::Debug;

#[type_derives(-Debug, -PartialEq, -serde::Serialize, -serde::Deserialize)]
pub struct Image {
    pub info: ImageInfo,
    #[cfg(feature = "skia")]
    pub skia_image: std::sync::Arc<skia_safe::image>,
    #[cfg(feature = "wasm-runtime")]
    pub drop_box: std::sync::Arc<DropBox>,
    #[cfg(feature = "wasm-drawer")]
    pub(crate) ck_image: std::sync::Arc<CkImage>,
}

impl Image {
    #[cfg(feature = "skia")]
    pub fn new(image_info: ImageInfo, image: skia_safe::Image) -> Self {
        Self {
            info: image_info,
            skia_image: std::sync::Arc::new(image),
        }
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image").field("info", &self.info).finish()
    }
}

impl PartialEq for Image {
    #[cfg(feature = "skia")]
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.skia_image, &other.skia_image)
    }
    #[cfg(feature = "wasm-runtime")]
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.drop_box, &other.drop_box)
    }
    #[cfg(feature = "wasm-drawer")]
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.ck_image, &other.ck_image)
    }
    #[cfg(not(any(feature = "skia", feature = "wasm-runtime", feature = "wasm-drawer")))]
    fn eq(&self, _other: &Self) -> bool {
        unreachable!()
    }
}

impl serde::Serialize for Image {
    #[cfg(feature = "skia")]
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unreachable!("Do not serialize Image in skia feature")
    }
    #[cfg(feature = "wasm-runtime")]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.drop_box.id)
    }
    #[cfg(feature = "wasm-drawer")]
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unreachable!("Do not serialize Image in wasm-drawer feature")
    }
    #[cfg(not(any(feature = "skia", feature = "wasm-runtime", feature = "wasm-drawer")))]
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unreachable!()
    }
}
impl<'de> serde::Deserialize<'de> for Image {
    #[cfg(feature = "skia")]
    fn deserialize<D>(_deserializer: D) -> Result<Image, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unreachable!("Do not deserialize Image in skia feature")
    }
    #[cfg(feature = "wasm-runtime")]
    fn deserialize<D>(_deserializer: D) -> Result<Image, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unreachable!("Do not deserialize Image in wasm-runtime feature")
    }
    #[cfg(feature = "wasm-drawer")]
    fn deserialize<D>(deserializer: D) -> Result<Image, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = u32::deserialize(deserializer)?;
        Ok(crate::canvas_kit::CkImage::get(id))
    }
    #[cfg(not(any(feature = "skia", feature = "wasm-runtime", feature = "wasm-drawer")))]
    fn deserialize<D>(_deserializer: D) -> Result<Image, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unreachable!()
    }
}

#[cfg(feature = "wasm-runtime")]
mod drop_box {
    pub struct DropBox {
        pub id: u32,
        drop_fn: Option<Box<dyn FnOnce() + Send + Sync>>,
    }

    impl Drop for DropBox {
        fn drop(&mut self) {
            (self.drop_fn.take().unwrap())();
        }
    }

    impl DropBox {
        pub fn new(id: u32, drop_fn: impl FnOnce() + Send + Sync + 'static) -> Self {
            Self {
                id,
                drop_fn: Some(Box::new(drop_fn)),
            }
        }
    }
}
#[cfg(feature = "wasm-runtime")]
pub use drop_box::*;

#[type_derives(Copy)]
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

#[cfg(target_family = "wasm")]
#[type_derives()]
pub struct ImageLoaded {
    pub id: u32,
    pub image_info: ImageInfo,
}
