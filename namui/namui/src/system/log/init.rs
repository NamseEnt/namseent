use super::config::LogConfig;
use super::ring_buffer::{RingBufferHandle, RingBufferLayer, install_ring_buffer};
use std::sync::Once;
#[cfg(not(target_os = "wasi"))]
use tracing_subscriber::fmt;
use tracing_subscriber::{EnvFilter, prelude::*};

#[cfg(not(target_os = "wasi"))]
static FILE_GUARD: std::sync::OnceLock<tracing_appender::non_blocking::WorkerGuard> =
    std::sync::OnceLock::new();

static INIT: Once = Once::new();

pub fn init_log_plugin_with_default() {
    init_log_plugin(LogConfig::default());
}

pub fn init_log_plugin(config: LogConfig) {
    INIT.call_once(|| {
        install(config);
        install_panic_hook();
    });
}

fn install_panic_hook() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "<unknown>".to_string());
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "<non-string panic payload>".to_string()
        };
        tracing::error!(target: "namui::panic", "panic at {location}: {payload}");
        prev(info);
    }));
}

fn install(config: LogConfig) {
    let env_filter = match std::env::var("RUST_LOG") {
        Ok(_) => EnvFilter::from_default_env(),
        Err(_) => match &config.filter {
            Some(f) => EnvFilter::new(f.clone()),
            None => EnvFilter::new(default_filter(&config.level)),
        },
    };

    let ring_layer = if config.in_game_console {
        let handle = RingBufferHandle::new(config.ring_buffer_capacity);
        install_ring_buffer(handle.clone());
        Some(RingBufferLayer::new(handle))
    } else {
        None
    };

    install_platform(env_filter, ring_layer, &config);

    let _ = tracing_log::LogTracer::init();
}

#[cfg(target_os = "wasi")]
fn install_platform(
    env_filter: EnvFilter,
    ring_layer: Option<RingBufferLayer>,
    _config: &LogConfig,
) {
    use tracing_subscriber::util::SubscriberInitExt;
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(super::wasi_console::WasiConsoleLayer)
        .with(ring_layer)
        .try_init();
}

#[cfg(not(target_os = "wasi"))]
fn install_platform(
    env_filter: EnvFilter,
    ring_layer: Option<RingBufferLayer>,
    config: &LogConfig,
) {
    let ansi_enabled = config.ansi.unwrap_or_else(default_ansi);
    let stderr_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(ansi_enabled)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false);

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(stderr_layer)
        .with(ring_layer);

    install_with_file_layer(registry, config);
}

#[cfg(not(target_os = "wasi"))]
fn install_with_file_layer<S>(registry: S, config: &LogConfig)
where
    S: tracing::Subscriber + Send + Sync,
    S: for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    use tracing_subscriber::util::SubscriberInitExt;

    let dir = resolve_file_output_dir(config);
    let Some(dir) = dir else {
        let _ = registry.try_init();
        return;
    };
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("namui::log: failed to create log dir {dir:?}: {e}");
        let _ = registry.try_init();
        return;
    }
    let file_prefix = config
        .app_name
        .clone()
        .unwrap_or_else(|| "namui".to_string());
    let file_appender = tracing_appender::rolling::daily(&dir, file_prefix);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let _ = FILE_GUARD.set(guard);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let _ = registry.with(file_layer).try_init();
}

#[cfg(not(target_os = "wasi"))]
fn resolve_file_output_dir(config: &LogConfig) -> Option<std::path::PathBuf> {
    if let Some(p) = config.file_output.clone() {
        return Some(p);
    }
    let app_name = config.app_name.as_deref()?;
    super::paths::default_log_dir(app_name)
}

fn default_filter(level: &tracing::Level) -> String {
    let lower = level.to_string().to_lowercase();
    format!("{lower},wgpu=warn,naga=warn,winit=warn,tokio=warn")
}

#[cfg(not(target_os = "wasi"))]
fn default_ansi() -> bool {
    use std::io::IsTerminal;
    std::io::stderr().is_terminal()
}
