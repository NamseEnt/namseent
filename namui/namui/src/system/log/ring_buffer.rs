use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    fmt::Write,
    sync::{Arc, OnceLock},
    time::SystemTime,
};
use tracing::{Event, Subscriber, field::Visit};
use tracing_subscriber::{Layer, layer::Context, registry::LookupSpan};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub level: tracing::Level,
    pub target: String,
    pub message: String,
}

#[derive(Clone)]
pub(crate) struct RingBufferHandle {
    buffer: Arc<Mutex<VecDeque<LogEntry>>>,
    capacity: usize,
}

impl RingBufferHandle {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity.min(1024)))),
            capacity,
        }
    }

    pub(crate) fn push(&self, entry: LogEntry) {
        let mut buf = self.buffer.lock();
        if buf.len() >= self.capacity {
            buf.pop_front();
        }
        buf.push_back(entry);
    }

    pub(crate) fn snapshot(&self, n: usize) -> Vec<LogEntry> {
        let buf = self.buffer.lock();
        let len = buf.len();
        let start = len.saturating_sub(n);
        buf.iter().skip(start).cloned().collect()
    }
}

static RING_BUFFER: OnceLock<RingBufferHandle> = OnceLock::new();

pub(crate) fn install_ring_buffer(handle: RingBufferHandle) {
    let _ = RING_BUFFER.set(handle);
}

pub fn dump_ring_buffer() -> Vec<LogEntry> {
    match RING_BUFFER.get() {
        Some(h) => h.snapshot(usize::MAX),
        None => Vec::new(),
    }
}

pub fn dump_recent_logs(n: usize) -> Vec<LogEntry> {
    match RING_BUFFER.get() {
        Some(h) => h.snapshot(n),
        None => Vec::new(),
    }
}

pub fn is_ring_buffer_installed() -> bool {
    RING_BUFFER.get().is_some()
}

pub struct RingBufferLayer {
    handle: RingBufferHandle,
}

impl RingBufferLayer {
    pub(crate) fn new(handle: RingBufferHandle) -> Self {
        Self { handle }
    }
}

#[derive(Default)]
struct MessageVisitor {
    message: String,
}

impl Visit for MessageVisitor {
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

impl<S> Layer<S> for RingBufferLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);
        self.handle.push(LogEntry {
            timestamp: SystemTime::now(),
            level: *metadata.level(),
            target: metadata.target().to_string(),
            message: visitor.message,
        });
    }
}
