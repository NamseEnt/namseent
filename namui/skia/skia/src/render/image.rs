use crate::*;
use std::{
    any::Any,
    fmt::Debug,
    sync::{Arc, OnceLock},
};

#[type_derives()]
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

#[type_derives()]
pub struct ImageHandle {
    pub alpha_type: AlphaType,
    pub color_type: ColorType,
    pub height: Px,
    pub width: Px,
    pub locker_key: Arc<Locker>,
}

impl ImageHandle {
    #[cfg(feature = "skia")]
    pub fn new(image_info: ImageInfo, image: skia_safe::Image) -> Self {
        Self {
            alpha_type: image_info.alpha_type,
            color_type: image_info.color_type,
            height: image_info.height,
            width: image_info.width,
            locker_key: Arc::new(Locker::new(image)),
        }
    }
}
use super::*;
use std::sync::atomic::AtomicUsize;

#[type_derives(Eq, Hash)]
pub struct Locker {
    key: usize,
}

impl Drop for Locker {
    fn drop(&mut self) {
        LOCKERS.get().unwrap().remove(self);
    }
}

static LOCKERS: OnceLock<dashmap::DashMap<Locker, Box<dyn Any + Send + Sync>>> = OnceLock::new();

impl Locker {
    pub fn new(value: impl Any + Send + Sync) -> Locker {
        static NEXT_LOCKER_KEY: AtomicUsize = AtomicUsize::new(0);
        let key = NEXT_LOCKER_KEY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        LOCKERS
            .get_or_init(dashmap::DashMap::new)
            .insert(Locker { key }, Box::new(value));

        Locker { key }
    }
    pub fn with<T: Any + Send + Sync>(&self, f: impl FnOnce(Option<&T>)) {
        if let Some(v) = LOCKERS.get().unwrap().get(self) {
            f(v.downcast_ref::<T>())
        }
    }
}
