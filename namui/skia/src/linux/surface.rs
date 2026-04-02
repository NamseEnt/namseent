use crate::*;
use namui_type::*;

pub struct NativeSurface {}

impl NativeSurface {
    pub fn resize(&mut self, _window_wh: Wh<IntPx>) {
        unimplemented!()
    }

    pub fn move_to_next_frame(&mut self) {
        unimplemented!()
    }

    pub fn flush(&mut self) {
        unimplemented!()
    }

    pub fn canvas(&mut self) -> &dyn SkCanvas {
        unimplemented!()
    }
}
