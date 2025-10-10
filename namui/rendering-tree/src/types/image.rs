use super::*;
use crate::*;
use std::{fmt::Debug, hash::Hash, sync::OnceLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash, State)]
pub struct Image {
    id: usize,
}

impl Image {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
    #[allow(dead_code)]
    pub fn get_default_shader(&self) -> Shader {
        Shader::Image {
            src: self.clone(),
            tile_mode: Xy::single(TileMode::Clamp),
        }
    }
    pub fn info(&self) -> ImageInfo {
        IMAGE_INFOS.with(|image_infos| {
            image_infos
                .get_or_init(|| {
                    let image_count = unsafe { _get_image_count() };
                    let mut image_infos = Vec::with_capacity(image_count);
                    let mut buffer = vec![0u8; image_count * 10];
                    unsafe { _get_image_infos(buffer.as_mut_ptr()) };
                    for i in 0..image_count {
                        let alpha_type = AlphaType::from(buffer[i * 10]);
                        let color_type = ColorType::from(buffer[i * 10 + 1]);
                        let width =
                            f32::from_le_bytes(buffer[i * 10 + 2..i * 10 + 6].try_into().unwrap())
                                .px();
                        let height =
                            f32::from_le_bytes(buffer[i * 10 + 6..i * 10 + 10].try_into().unwrap())
                                .px();
                        image_infos.push(ImageInfo {
                            alpha_type,
                            color_type,
                            width,
                            height,
                        });
                    }
                    image_infos
                })
                .get(self.id)
                .cloned()
                .unwrap_or_else(|| panic!("Image {} not found", self.id))
        })
    }
}

thread_local! {
    static IMAGE_INFOS: OnceLock<Vec<ImageInfo>> = const { OnceLock::new() };
}

unsafe extern "C" {
    fn _get_image_count() -> usize;
    // buffer length = (1 + 1 + 4 + 4 = 10) * n
    fn _get_image_infos(buffer: *mut u8);
}

#[derive(Debug, Clone, Copy, Hash, namui_type::State)]
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
