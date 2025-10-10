mod canvas;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
mod traits;
#[cfg(target_os = "wasi")]
mod wasi;
#[cfg(target_os = "windows")]
mod windows;

use anyhow::Result;
pub use namui_rendering_tree::*;
use namui_type::*;
pub use traits::*;

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "wasi")]
pub use wasi::*;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub fn init_skia(_screen_id: usize, _window_wh: Wh<IntPx>) -> Result<NativeSkia> {
    unimplemented!()
}
#[cfg(target_os = "macos")]
pub fn init_skia(_screen_id: usize, _window_wh: Wh<IntPx>) -> Result<NativeSkia> {
    unimplemented!()
}
#[cfg(target_os = "wasi")]
pub fn init_skia(_screen_id: usize, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
    NativeSkia::new(window_wh)
}
#[cfg(target_os = "windows")]
pub fn init_skia(screen_id: usize, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
    NativeSkia::new(screen_id, window_wh)
}
