use crate::system::InitResult;
use crate::*;
use std::sync::atomic::AtomicU32;

pub(crate) async fn init() -> InitResult {
    Ok(())
}

// width 16bits, height 16bits
static SIZE: AtomicU32 = AtomicU32::new(0);

pub extern "C" fn on_resize(width: u16, height: u16) {
    SIZE.store(
        (width as u32) << 16 | height as u32,
        std::sync::atomic::Ordering::Relaxed,
    );

    let wh = crate::Wh {
        width: (width as i32).int_px(),
        height: (height as i32).int_px(),
    };

    skia::on_window_resize(wh);

    crate::hooks::on_raw_event(RawEvent::ScreenResize { wh });
}

pub extern "C" fn on_animation_frame() {
    skia::redraw();
    crate::hooks::on_raw_event(RawEvent::ScreenRedraw {});
}

pub fn size() -> crate::Wh<IntPx> {
    let size = SIZE.load(std::sync::atomic::Ordering::Relaxed);
    crate::Wh {
        width: ((size >> 16) as i32).int_px(),
        height: ((size & 0xffff) as i32).int_px(),
    }
}

pub(crate) fn take_main_thread() {
    // Do nothing
}
