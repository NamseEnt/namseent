//! WASM doesn't have access to the file system, so this module is only available for non-WASM targets.

#[cfg(not(target_os = "wasi"))]
mod non_wasm;

#[cfg(not(target_os = "wasi"))]
pub use non_wasm::*;
