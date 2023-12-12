#[cfg(target_family = "wasm")]
pub mod canvas_kit;
mod log;
mod traits;

use anyhow::Result;
#[cfg(target_family = "wasm")]
use canvas_kit::CkSkia;
use namui_type::{IntPx, Wh};
use std::sync::Arc;
pub use traits::*;
#[cfg(target_family = "wasm")]
use web_sys::HtmlCanvasElement;

#[cfg(target_family = "wasm")]
pub fn init_skia(canvas_element: Option<&HtmlCanvasElement>) -> Arc<dyn SkSkia + Send + Sync> {
    Arc::new(CkSkia::new(canvas_element))
}

#[cfg(not(target_family = "wasm"))]
pub mod native;

#[cfg(not(target_family = "wasm"))]
use native::NativeSkia;

#[cfg(not(target_family = "wasm"))]
pub fn init_skia(screen_id: usize, window_wh: Wh<IntPx>) -> Result<Arc<dyn SkSkia + Send + Sync>> {
    Ok(Arc::new(NativeSkia::new(screen_id, window_wh)?))
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
