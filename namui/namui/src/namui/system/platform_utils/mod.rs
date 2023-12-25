#[cfg(target_family = "wasm")]
pub mod web;
#[cfg(target_family = "wasm")]
pub use web::*;
