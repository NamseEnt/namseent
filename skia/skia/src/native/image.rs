use super::*;
use crate::*;
use std::sync::Arc;

pub struct NativeImage {
    skia_image: skia_safe::Image,
    image_info: ImageInfo,
    src: ImageSource,
}

unsafe impl Send for NativeImage {}
unsafe impl Sync for NativeImage {}

static IMAGE_MAP: StaticHashMap<ImageSource, NativeImage> = StaticHashMap::new();

impl NativeImage {
    pub(crate) fn load(image_source: ImageSource, encoded_image: &[u8]) {
        IMAGE_MAP.get_or_create(image_source, |image_source| {
            let skia_image =
                skia_safe::Image::from_encoded(skia_safe::Data::new_copy(encoded_image)).unwrap();

            let image_info = get_image_info(&skia_image);

            NativeImage {
                skia_image,
                image_info,
                src: image_source.clone(),
            }
        });
    }

    pub(crate) fn get(image_source: &ImageSource) -> Option<Arc<NativeImage>> {
        IMAGE_MAP.get(image_source)
    }

    pub(crate) fn image(&self) -> Image {
        Image {
            wh: self.size(),
            src: self.src.clone(),
        }
    }

    pub fn size(&self) -> Wh<Px> {
        let skia_image_info = self.info();
        Wh {
            width: skia_image_info.width,
            height: skia_image_info.height,
        }
    }
    pub(crate) fn get_default_shader(&self) -> Shader {
        Shader::Image {
            src: self.src.clone(),
        }
    }

    pub(crate) fn skia(&self) -> &skia_safe::Image {
        &self.skia_image
    }
}

impl SkImage for NativeImage {
    fn info(&self) -> ImageInfo {
        self.image_info
    }
}

fn get_image_info(skia_image: &skia_safe::Image) -> ImageInfo {
    let skia_image_info = skia_image.image_info();
    ImageInfo {
        width: skia_image_info.width().px(),
        height: skia_image_info.height().px(),
        alpha_type: skia_image_info.alpha_type().into(),
        color_type: skia_image_info.color_type().into(),
    }
}
