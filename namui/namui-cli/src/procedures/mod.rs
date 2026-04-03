mod build;
mod check;
mod clippy;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
mod start;
mod test;
#[cfg(target_os = "windows")]
pub mod windows;

pub use build::*;
#[allow(unused_imports)]
pub use check::*;
#[allow(unused_imports)]
pub use clippy::*;
#[allow(unused_imports)]
pub use start::*;
#[allow(unused_imports)]
pub use test::*;
