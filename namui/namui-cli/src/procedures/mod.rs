mod build;
mod check;
mod clippy;
#[cfg(target_os = "linux")]
pub mod linux;
mod start;
mod test;
#[cfg(target_os = "windows")]
pub mod windows;

pub use build::*;
pub use check::*;
pub use clippy::*;
pub use start::*;
pub use test::*;
