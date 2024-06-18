use super::*;
use namui_skia::*;

pub(crate) fn init() -> Result<()> {
    tokio::task::spawn_blocking(on_skia_drawing_thread);
    Ok(())
}

pub(super) fn init_skia() -> Result<NativeSkia> {
    let skia = namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )?;

    Ok(skia)
}

pub(super) fn after_draw() {
    // Nothing
}
