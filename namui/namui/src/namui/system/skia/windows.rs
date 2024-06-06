use namui_skia::*;

static SKIA: OnceLock<Arc<RwLock<NativeSkia>>> = OnceLock::new();
static DRAW_COMMAND_TX: OnceLock<tokio::sync::mpsc::UnboundedSender<DrawingCommand>> =
    OnceLock::new();

pub(super) fn init_skia() -> Result<NativeSkia> {
    let skia = namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )?;

    Ok(skia)
}
