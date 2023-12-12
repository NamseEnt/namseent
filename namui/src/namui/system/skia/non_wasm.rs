use anyhow::Result;
use namui_skia::*;

pub(super) async fn init_skia() -> Result<std::sync::Arc<dyn SkSkia + Send + Sync>> {
    namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )
}
