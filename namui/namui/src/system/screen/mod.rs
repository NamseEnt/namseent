#[cfg(not(target_os = "wasi"))]
mod non_wasm;
// #[cfg(target_os = "wasi")]
mod wasm;

#[cfg(not(target_os = "wasi"))]
pub use non_wasm::*;
#[cfg(target_os = "wasi")]
pub use wasm::*;
