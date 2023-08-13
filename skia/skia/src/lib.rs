mod traits;

use canvas_kit::{CkImage, CkSkia};
use std::sync::Arc;
pub use traits::*;
use web_sys::HtmlCanvasElement;

#[cfg(target_family = "wasm")]
pub mod canvas_kit;

pub fn init_skia(canvas_element: Option<&HtmlCanvasElement>) -> Arc<dyn SkSkia + Send + Sync> {
    Arc::new(CkSkia::new(canvas_element))
}
