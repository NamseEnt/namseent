#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(not(target_family = "wasm"))]
pub(crate) use non_wasm::*;
#[cfg(target_family = "wasm")]
pub(crate) use wasm::*;
