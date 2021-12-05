#[cfg(target_family = "wasm")]
pub mod namui;
#[cfg(target_family = "wasm")]
pub use namui::*;
#[cfg(target_family = "wasm")]
pub mod prelude;

pub mod build;
