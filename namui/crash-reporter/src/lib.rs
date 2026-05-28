//! namui crash reporter.
//!
//! Captures minidumps via Embark `crash-handler` + `minidumper` (out-of-process
//! child) and uploads them to a [namsh](https://github.com/NamseEnt/namseent)
//! deployment.
//!
//! The host binary is expected to detect its mode at startup. When invoked with
//! `--namui-crash-server <socket-name>`, call [`server_main`]. Otherwise, call
//! [`init`] once before running the rest of the program, and hold the returned
//! [`CrashGuard`] for the lifetime of the process.

mod child;
mod context;
mod error;
mod install_id;
mod log_capture;
mod namsh;
mod parent;
mod queue;
mod stack_hash;

pub use log_capture::{LogCapture, start as start_log_capture};

pub use child::server_main;
pub use error::Error;
pub use parent::{CrashGuard, init};

#[derive(Clone)]
pub struct Config {
    pub build_id: String,
    pub hmac_key_hex: String,
    pub namsh_url: String,
    pub app_name: String,
}
