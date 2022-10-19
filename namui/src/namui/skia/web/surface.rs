use super::*;

pub(crate) struct Surface {
    canvas_kit_surface: CanvasKitSurface,
    canvas: Canvas,
}
unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}
impl Surface {
    pub fn new(canvas_kit_surface: CanvasKitSurface) -> Surface {
        let canvas = canvas_kit_surface.getCanvas();
        Surface {
            canvas_kit_surface,
            canvas: Canvas(canvas),
        }
    }
    pub fn flush(&self) {
        self.canvas_kit_surface.flush();
    }
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }
    pub fn make_image_from_texture_source(
        &self,
        src: web_sys::HtmlImageElement, // NOTE: It can also be an HTMLVideoElement or an HTMLCanvasElement.
        info: Option<PartialImageInfo>,
        src_is_premul: Option<bool>,
    ) -> Image {
        let info = info.map(|info| info.into_js_object());
        let image = self
            .canvas_kit_surface
            .makeImageFromTextureSource(src, info, src_is_premul);
        let image = image.makeCopyWithDefaultMipmaps();
        Image::new(image)
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.canvas_kit_surface.delete();
    }
}
