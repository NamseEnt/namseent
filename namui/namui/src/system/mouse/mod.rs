use super::InitResult;
use crate::*;
use std::sync::atomic::AtomicU32;

pub(crate) fn init() -> InitResult {
    Ok(())
}

pub fn set_mouse_cursor(_cursor: &MouseCursor) {
    // TODO: implement native cursor support
}

#[cfg(target_os = "wasi")]
mod wasi_ffi;
#[cfg(not(target_os = "wasi"))]
pub(crate) mod non_wasm;

/// Convert a u8 button code to a MouseButton (for FFI use)
#[cfg(not(target_os = "wasi"))]
pub(crate) fn button_from_u8(button: u8) -> Option<MouseButton> {
    match button {
        0 => Some(MouseButton::Left),
        1 => Some(MouseButton::Right),
        2 => Some(MouseButton::Middle),
        _ => None,
    }
}

// --- Shared position tracking (used by both wasi and native) ---

/// 16 bit x, 16 bit y
static MOUSE_POSITION: AtomicU32 = AtomicU32::new(0);

pub fn position() -> Xy<Px> {
    let mouse_position = MOUSE_POSITION.load(std::sync::atomic::Ordering::SeqCst);
    Xy::new(
        px((mouse_position >> 16) as f32),
        px((mouse_position & 0xffff) as f32),
    )
}

pub(crate) fn mouse_position_u32() -> u32 {
    MOUSE_POSITION.load(std::sync::atomic::Ordering::SeqCst)
}

fn update_mouse_position_atomic(x: u16, y: u16) {
    MOUSE_POSITION.store(
        (x as u32) << 16 | y as u32,
        std::sync::atomic::Ordering::SeqCst,
    );
}

// --- Non-WASI: shared state for winit mouse ---

#[cfg(not(target_os = "wasi"))]
use std::sync::RwLock;
#[cfg(not(target_os = "wasi"))]
use std::collections::HashSet;

#[cfg(not(target_os = "wasi"))]
pub(crate) struct MouseSystem {
    pub mouse_position: RwLock<Xy<Px>>,
    pub pressing_buttons: RwLock<HashSet<MouseButton>>,
}

#[cfg(not(target_os = "wasi"))]
pub(crate) static MOUSE_SYSTEM: std::sync::LazyLock<MouseSystem> =
    std::sync::LazyLock::new(|| MouseSystem {
        mouse_position: RwLock::new(Xy::new(px(0.0), px(0.0))),
        pressing_buttons: RwLock::new(HashSet::new()),
    });
