use crate::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) struct NativeColorFilter {
    skia_color_filter: skia_safe::ColorFilter,
}

impl NativeColorFilter {
    pub(crate) fn get(color_filter: ColorFilter) -> Arc<NativeColorFilter> {
        static CACHE: SerdeLruCache<ColorFilter, NativeColorFilter> = SerdeLruCache::new();

        CACHE.get_or_create(&color_filter, |color_filter| NativeColorFilter {
            skia_color_filter: {
                skia_safe::color_filters::blend(color_filter.color, color_filter.blend_mode.into())
                    .expect("Failed to create color filter")
            },
        })
    }

    pub(crate) fn skia(&self) -> &skia_safe::ColorFilter {
        &self.skia_color_filter
    }
}
