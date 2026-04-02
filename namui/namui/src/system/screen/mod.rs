use crate::*;
use std::sync::{OnceLock, atomic::AtomicU32};

static SIZE: OnceLock<AtomicU32> = OnceLock::new();

pub(crate) fn init() -> crate::Result<()> {
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
