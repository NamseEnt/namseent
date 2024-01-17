#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(target_family = "wasm")]
pub use wasm::*;
