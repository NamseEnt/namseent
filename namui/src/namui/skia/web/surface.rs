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
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.canvas_kit_surface.delete();
    }
}
