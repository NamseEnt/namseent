use super::InitResult;
use std::sync::atomic::AtomicBool;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

pub(super) async fn init() -> InitResult {
    Ok(())
}

static PANICKED: AtomicBool = AtomicBool::new(false);

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn panicked() {
    PANICKED.store(true, std::sync::atomic::Ordering::Relaxed);
}

pub(crate) fn is_panicked() -> bool {
    PANICKED.load(std::sync::atomic::Ordering::Relaxed)
}
