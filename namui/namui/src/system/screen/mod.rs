use crate::*;
use std::sync::{OnceLock, atomic::AtomicU32};

static SIZE: OnceLock<AtomicU32> = OnceLock::new();

#[cfg(target_os = "wasi")]
pub(crate) fn init() -> crate::Result<()> {
    unsafe extern "C" {
        fn _initial_window_wh() -> u32;
    }
    let window_wh = unsafe { _initial_window_wh() };
    let _ = SIZE.set(AtomicU32::new(window_wh));
    Ok(())
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn init() -> crate::Result<()> {
    // On native, the initial size is set by the runner via set_size()
    let _ = SIZE.set(AtomicU32::new(0));
    Ok(())
}

pub fn size() -> crate::Wh<IntPx> {
    let size = SIZE
        .get()
        .unwrap()
        .load(std::sync::atomic::Ordering::Relaxed);
    crate::Wh {
        width: ((size >> 16) as i32).int_px(),
        height: ((size & 0xffff) as i32).int_px(),
    }
}

pub(crate) fn set_size(width: u16, height: u16) {
    SIZE.get().unwrap().store(
        (width as u32) << 16 | height as u32,
        std::sync::atomic::Ordering::Relaxed,
    );
}

// --- WASI-only FFI exports ---

#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
pub extern "C" fn _on_screen_resize(width: u16, height: u16) -> u64 {
    set_size(width, height);

    crate::on_event(RawEvent::ScreenResize {
        wh: Wh::new(int_px(width as i32), int_px(height as i32)),
    })
}

#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
pub extern "C" fn _on_animation_frame() -> u64 {
    crate::on_event(RawEvent::ScreenRedraw)
}

#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
pub extern "C" fn _on_blur() -> u64 {
    crate::on_event(RawEvent::Blur)
}

#[cfg(target_os = "wasi")]
#[unsafe(no_mangle)]
pub extern "C" fn _on_visibility_change() -> u64 {
    crate::on_event(RawEvent::VisibilityChange)
}
