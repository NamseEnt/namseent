use super::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) struct CkColorFilter {
    canvas_kit_color_filter: CanvasKitColorFilter,
}

impl CkColorFilter {
    pub(crate) fn get(color_filter: ColorFilter) -> Arc<CkColorFilter> {
        static CACHE: SerdeLruCache<ColorFilter, CkColorFilter> = SerdeLruCache::new();

        CACHE.get_or_create(&color_filter, |color_filter| CkColorFilter {
            canvas_kit_color_filter: {
                let color_array = color_filter.color.into_float32_array();
                canvas_kit()
                    .ColorFilter()
                    .MakeBlend(&color_array, color_filter.blend_mode.into())
            },
        })
    }

    pub(crate) fn canvas_kit(&self) -> &CanvasKitColorFilter {
        &self.canvas_kit_color_filter
    }
}

impl Drop for CkColorFilter {
    fn drop(&mut self) {
        self.canvas_kit_color_filter.delete();
    }
}
