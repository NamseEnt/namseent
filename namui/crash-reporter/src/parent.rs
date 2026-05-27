use crate::{Config, Error, install_id, namsh, queue};
use crash_handler::CrashHandler;
use minidumper::{Client, SocketName};
use std::{
    process::{Child, Command},
    sync::Arc,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub struct CrashGuard {
    _handler: CrashHandler,
    _child: Child,
}

pub fn init(config: &Config) -> Result<CrashGuard, Error> {
    let root = queue::root_dir(&config.app_name)?;
    std::fs::create_dir_all(&root)?;
    let _ = install_id::get_or_create(&config.app_name)?;

    let pending_config = config.clone();
    thread::Builder::new()
        .name("crash-reporter-uploader".into())
        .spawn(move || {
            if let Err(e) = namsh::flush_queue(&pending_config) {
                eprintln!("[crash-reporter] flush_queue error: {e}");
            }
        })?;

    let pid = std::process::id();
    let socket_name_owned = format!("/tmp/namui-crash-{pid}.sock");

    let parent_start_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let exe = std::env::current_exe()?;
    let child = Command::new(exe)
        .arg("--namui-crash-server")
        .arg(&socket_name_owned)
        .env("NAMUI_CRASH_PARENT_PID", pid.to_string())
        .env(
            "NAMUI_CRASH_PARENT_START_UNIX",
            parent_start_unix.to_string(),
        )
        .spawn()?;

    let socket_path: std::path::PathBuf = socket_name_owned.into();
    let client = connect_with_retry(&socket_path, Duration::from_secs(5))?;
    let client = Arc::new(client);

    let client_for_handler = client.clone();
    let handler = CrashHandler::attach(unsafe {
        crash_handler::make_crash_event(move |crash_context: &crash_handler::CrashContext| {
            let _ = client_for_handler.ping();
            crash_handler::CrashEventResult::Handled(
                client_for_handler.request_dump(crash_context).is_ok(),
            )
        })
    })?;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    handler.set_ptracer(Some(child.id()));

    install_panic_hook();

    Ok(CrashGuard {
        _handler: handler,
        _child: child,
    })
}

/// Translate a Rust `panic!()` into a signal/exception the installed
/// `CrashHandler` can capture.
///
/// Without this, panic starts a Rust unwind that drops `CrashGuard` (and with
/// it the `CrashHandler` + minidumper child) *before* the process actually
/// aborts, so the SIGABRT raised at the end of unwind has no handler left to
/// receive it — and no dump is produced. Calling `process::abort()` from
/// inside the panic hook skips the unwind entirely, leaving `CrashGuard`
/// alive when the signal fires.
fn install_panic_hook() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        prev(info);
        std::process::abort();
    }));
}

fn connect_with_retry(
    socket_path: &std::path::Path,
    timeout: Duration,
) -> Result<Client, Error> {
    let deadline = Instant::now() + timeout;
    loop {
        match Client::with_name(SocketName::Path(socket_path)) {
            Ok(c) => return Ok(c),
            Err(e) => {
                if Instant::now() >= deadline {
                    eprintln!("[crash-reporter] last connect error: {e:?}");
                    return Err(Error::ChildConnectTimeout);
                }
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}
