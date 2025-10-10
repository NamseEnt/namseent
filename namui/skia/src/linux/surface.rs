use crate::*;
use namui_type::*;

pub(crate) struct NativeSurface {}

impl NativeSurface {
    pub(crate) fn resize(&mut self, _window_wh: Wh<IntPx>) {
        unimplemented!()
    }

    /// Should be called before use surface
    pub(crate) fn move_to_next_frame(&mut self) {
        unimplemented!()
    }
}

impl SkSurface for NativeSurface {
    fn flush(&mut self) {
        unimplemented!()
    }

    fn canvas(&mut self) -> &dyn SkCanvas {
        unimplemented!()
    }
}
