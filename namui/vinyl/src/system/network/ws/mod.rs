#[cfg(target_os = "wasi")]
mod wasi;

#[cfg(target_os = "wasi")]
pub use wasi::*;
