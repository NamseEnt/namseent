use crate::*;
use namui_type::*;

pub struct NativeSkia {
    surface: NativeSurface,
}

impl NativeSkia {
    pub fn move_to_next_frame(&mut self) {
        self.surface.move_to_next_frame();
    }
    pub fn surface(&mut self) -> &mut NativeSurface {
        &mut self.surface
    }
    pub fn on_resize(&mut self, wh: Wh<IntPx>) {
        self.surface.resize(wh);
    }
}
