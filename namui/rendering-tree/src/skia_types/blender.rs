use crate::*;
use std::sync::Arc;

pub struct NativeBlender {
    pub skia_blender: skia_safe::Blender,
}

impl NativeBlender {
    pub fn get(blender: &Blender) -> Arc<NativeBlender> {
        static CACHE: LruCache<Blender, NativeBlender> = LruCache::new();

        CACHE.get_or_create(blender, |blender| blender.into())
    }

    pub fn skia(&self) -> &skia_safe::Blender {
        &self.skia_blender
    }
}
