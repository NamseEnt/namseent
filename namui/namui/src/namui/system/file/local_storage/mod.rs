//! Local storage is the persistent key-value storage.

#[cfg(not(target_os = "wasi"))]
mod posix;
#[cfg(target_os = "wasi")]
mod wasi;

#[cfg(not(target_os = "wasi"))]
pub use posix::*;
#[cfg(target_os = "wasi")]
pub use wasi::*;
