use super::*;
use crate::*;
use web_sys::HtmlCanvasElement;

pub(crate) struct CkSkia {
    surface: Option<CkSurface>,
}
unsafe impl Send for CkSkia {}
unsafe impl Sync for CkSkia {}

impl CkSkia {
    pub(crate) fn new(canvas_element: Option<&HtmlCanvasElement>) -> CkSkia {
        Self {
            surface: canvas_element.map(|x| CkSurface::new(x)),
        }
    }
}

impl SkSkia for CkSkia {
    fn surface(&self) -> &dyn SkSurface {
        self.surface.as_ref().unwrap()
    }

    fn group_glyph(&self, font: &Font, paint: &Paint) -> std::sync::Arc<dyn GroupGlyph> {
        CkGroupGlyph::get(font, paint)
    }

    // fn image_info(&self, image_source: &ImageSource) -> Option<ImageInfo> {
    //     // CkImage::get(image_source).map(|x| x.info())
    // }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        CkFont::get(font).map(|x| x.metrics)
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) {
        CkTypeface::load(typeface_name, bytes)
    }

    fn load_image(&self, image_source: &ImageSource, image_bitmap: web_sys::ImageBitmap) {
        CkImage::load(image_source, image_bitmap)
    }

    fn image(&self, image_source: &ImageSource) -> Option<Image> {
        CkImage::get(image_source).map(|x| x.image())
    }
}
