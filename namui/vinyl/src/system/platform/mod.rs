//! Do something about platform specific things.

#[cfg(target_os = "wasi")]
pub mod wasi;
