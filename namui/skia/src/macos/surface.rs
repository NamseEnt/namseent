use crate::*;
use namui_type::*;

pub struct NativeSurface {
    surface: skia_safe::surface::Surface,
}

impl NativeSurface {
    pub fn resize(&mut self, _window_wh: Wh<IntPx>) {
        unimplemented!()
    }

    /// Should be called before use surface
    pub fn move_to_next_frame(&mut self) {
        unimplemented!()
    }

    pub fn flush(&mut self) {
        unimplemented!()
    }

    pub fn canvas(&mut self) -> &dyn SkCanvas {
        self.surface.canvas()
    }
}
