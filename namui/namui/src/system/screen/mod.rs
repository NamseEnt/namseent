use crate::*;
use std::sync::{OnceLock, atomic::AtomicU32};

unsafe extern "C" {
    fn _initial_window_wh() -> u32;
}

static SIZE: OnceLock<AtomicU32> = OnceLock::new();

pub(crate) fn init() -> crate::Result<()> {
    let window_wh = unsafe { _initial_window_wh() };
    let _ = SIZE.set(AtomicU32::new(window_wh));

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

#[unsafe(no_mangle)]
pub extern "C" fn _on_screen_resize(width: u16, height: u16) -> u64 {
    set_size(width, height);

    crate::on_event(RawEvent::ScreenResize {
        wh: Wh::new(int_px(width as i32), int_px(height as i32)),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_animation_frame() -> u64 {
    crate::on_event(RawEvent::ScreenRedraw)
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_blur() -> u64 {
    crate::on_event(RawEvent::Blur)
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_visibility_change() -> u64 {
    crate::on_event(RawEvent::VisibilityChange)
}

fn set_size(width: u16, height: u16) {
    SIZE.get().unwrap().store(
        (width as u32) << 16 | height as u32,
        std::sync::atomic::Ordering::Relaxed,
    );
}
