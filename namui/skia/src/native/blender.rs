use crate::*;
use std::sync::Arc;

pub(crate) struct NativeBlender {
    pub(crate) skia_blender: skia_safe::Blender,
}

impl NativeBlender {
    pub(crate) fn get(blender: &Blender) -> Arc<NativeBlender> {
        static CACHE: LruCache<Blender, NativeBlender> = LruCache::new();

        CACHE.get_or_create(blender, |blender| blender.into())
    }

    pub(crate) fn skia(&self) -> &skia_safe::Blender {
        &self.skia_blender
    }
}

impl From<&Blender> for NativeBlender {
    fn from(blender: &Blender) -> Self {
        let skia_blender = match blender {
            Blender::BlendMode(blend_mode) => skia_safe::BlendMode::from(*blend_mode).into(),
            Blender::Sksl(sksl) => skia_safe::RuntimeEffect::make_for_blender(sksl, None)
                .unwrap()
                .make_blender(skia_safe::Data::new_empty(), None)
                .unwrap(),
            Blender::Arithmetic { k1, k2, k3, k4 } => skia_safe::Blender::arithmetic(
                (*k1).into(),
                (*k2).into(),
                (*k3).into(),
                (*k4).into(),
                false,
            )
            .unwrap(),
        };
        NativeBlender { skia_blender }
    }
}

impl From<&Blender> for skia_safe::Blender {
    fn from(blender: &Blender) -> Self {
        let native_blender = NativeBlender::get(blender);
        native_blender.skia().clone()
    }
}
