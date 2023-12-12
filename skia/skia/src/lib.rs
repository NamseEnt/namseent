#[cfg(target_family = "wasm")]
pub mod canvas_kit;
mod log;
mod traits;

use anyhow::Result;
#[cfg(target_family = "wasm")]
use canvas_kit::CkSkia;
use namui_type::{IntPx, Wh};
use std::sync::Arc;
use std::sync::Mutex;
pub use traits::*;
#[cfg(target_family = "wasm")]
use web_sys::HtmlCanvasElement;

#[cfg(target_family = "wasm")]
pub fn init_skia(
    canvas_element: Option<&HtmlCanvasElement>,
) -> Result<Arc<Mutex<dyn SkSkia + Send + Sync>>> {
    Ok(Arc::new(Mutex::new(CkSkia::new(canvas_element))))
}

#[cfg(feature = "window")]
#[cfg(not(target_family = "wasm"))]
pub mod native;

#[cfg(feature = "window")]
#[cfg(not(target_family = "wasm"))]
use native::NativeSkia;

#[cfg(feature = "window")]
#[cfg(not(target_family = "wasm"))]
pub fn init_skia(
    screen_id: usize,
    window_wh: Wh<IntPx>,
) -> Result<Arc<Mutex<dyn SkSkia + Send + Sync>>> {
    Ok(Arc::new(Mutex::new(NativeSkia::new(screen_id, window_wh)?)))
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
