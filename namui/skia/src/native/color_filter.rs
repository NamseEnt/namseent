use crate::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) struct NativeColorFilter {
    pub(crate) skia_color_filter: skia_safe::ColorFilter,
}

impl NativeColorFilter {
    pub(crate) fn get(color_filter: ColorFilter) -> Arc<NativeColorFilter> {
        static CACHE: LruCache<ColorFilter, NativeColorFilter> = LruCache::new();

        CACHE.get_or_create(&color_filter, |color_filter| color_filter.into())
    }

    pub(crate) fn skia(&self) -> &skia_safe::ColorFilter {
        &self.skia_color_filter
    }
}
