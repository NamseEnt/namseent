mod log;
mod traits;

use std::sync::Arc;
pub use traits::*;

#[cfg(target_family = "wasm")]
pub mod canvas_kit;

#[cfg(target_family = "wasm")]
use canvas_kit::CkSkia;
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
pub fn init_skia(
    context: skia_safe::gpu::DirectContext,
    surfaces: Vec<skia_safe::Surface>,
) -> Arc<dyn SkSkia + Send + Sync> {
    Arc::new(NativeSkia::new(context, surfaces))
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
