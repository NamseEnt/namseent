mod config;
mod init;
mod macros;
mod paths;
mod ring_buffer;
#[cfg(target_os = "wasi")]
mod wasi_console;

pub use config::{LogConfig, LogConfigBuilder};
pub use init::{init_log_plugin, init_log_plugin_with_default};
pub use ring_buffer::{LogEntry, dump_recent_logs, dump_ring_buffer, is_ring_buffer_installed};

pub use tracing::Level;
