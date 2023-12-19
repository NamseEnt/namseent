use super::*;
use crate::*;
use web_sys::HtmlCanvasElement;

pub(crate) struct CkSurface {
    canvas_kit_surface: CanvasKitSurface,
    canvas: CkCanvas,
}
unsafe impl Send for CkSurface {}
unsafe impl Sync for CkSurface {}

impl CkSurface {
    pub(crate) fn new(canvas_element: &HtmlCanvasElement) -> CkSurface {
        let canvas_kit_surface = canvas_kit().make_web_glcanvas_surface(canvas_element, None, None);

        let canvas = canvas_kit_surface.getCanvas();
        CkSurface {
            canvas_kit_surface,
            canvas: CkCanvas::new(canvas),
        }
    }
}

impl SkSurface for CkSurface {
    fn flush(&mut self) {
        self.canvas_kit_surface.flush();
    }
    fn canvas(&self) -> &dyn SkCanvas {
        &self.canvas
    }
}

impl Drop for CkSurface {
    fn drop(&mut self) {
        self.canvas_kit_surface.delete();
    }
}
