use anyhow::Result;
use namui_skia::*;
use namui_type::*;
use std::sync::{Arc, Mutex};

pub(super) async fn init_skia() -> Result<Arc<Mutex<dyn SkSkia + Send + Sync>>> {
    namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )
}

pub(crate) fn on_window_resize(wh: Wh<IntPx>) {
    let mut skia = super::SKIA.get().unwrap().lock().unwrap();
    skia.on_resize(wh);
}

pub(crate) fn render(draw_input: DrawInput) {
    let mut skia = super::SKIA.get().unwrap().lock().unwrap();
    // skia.(wh);
}
