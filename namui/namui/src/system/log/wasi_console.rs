#![cfg(target_os = "wasi")]

use std::fmt::Write;
use tracing::{Event, Subscriber, field::Visit};
use tracing_subscriber::{Layer, layer::Context, registry::LookupSpan};

unsafe extern "C" {
    fn _namui_console_log(level: u32, ptr: *const u8, len: usize);
}

pub(super) struct WasiConsoleLayer;

#[derive(Default)]
struct ConsoleVisitor {
    message: String,
}

impl Visit for ConsoleVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            let _ = write!(self.message, "{value:?}");
        } else {
            if !self.message.is_empty() {
                self.message.push(' ');
            }
            let _ = write!(self.message, "{}={:?}", field.name(), value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message.push_str(value);
        } else {
            if !self.message.is_empty() {
                self.message.push(' ');
            }
            let _ = write!(self.message, "{}={}", field.name(), value);
        }
    }
}

impl<S> Layer<S> for WasiConsoleLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let level_num = match *metadata.level() {
            tracing::Level::ERROR => 1u32,
            tracing::Level::WARN => 2,
            tracing::Level::INFO => 3,
            tracing::Level::DEBUG => 4,
            tracing::Level::TRACE => 5,
        };

        let mut visitor = ConsoleVisitor::default();
        event.record(&mut visitor);

        let formatted = format!(
            "{level} {target}: {msg}",
            level = metadata.level(),
            target = metadata.target(),
            msg = visitor.message,
        );

        let bytes = formatted.as_bytes();
        unsafe { _namui_console_log(level_num, bytes.as_ptr(), bytes.len()) };
    }
}
