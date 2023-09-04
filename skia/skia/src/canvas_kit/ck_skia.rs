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
            surface: canvas_element.map(CkSurface::new),
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

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        CkFont::get(font).map(|x| x.metrics)
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) {
        CkTypeface::load(typeface_name, bytes)
    }

    fn load_image(&self, image_source: &ImageSource, image_bitmap: &web_sys::ImageBitmap) {
        CkImage::load(image_source, image_bitmap)
    }

    fn image(&self, image_source: &ImageSource) -> Option<Image> {
        CkImage::get(image_source).map(|x| x.image())
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        CkPath::get(path).contains(paint, xy)
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        CkPath::get(path).bounding_box(paint)
    }

    fn encode_loaded_image_to_png(&self, image: &Image) -> Vec<u8> {
        CkImage::get(&image.src).unwrap().encode_to_png()
    }
}
