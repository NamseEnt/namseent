#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(unix)]
unsafe extern "C" fn signal_handler(sig: libc::c_int) {
    // Only async-signal-safe operations here
    let msg = match sig {
        libc::SIGSEGV => b"[runner] FATAL: SIGSEGV (segmentation fault)\n" as &[u8],
        libc::SIGBUS => b"[runner] FATAL: SIGBUS (bus error)\n",
        libc::SIGABRT => b"[runner] FATAL: SIGABRT (abort)\n",
        libc::SIGILL => b"[runner] FATAL: SIGILL (illegal instruction)\n",
        libc::SIGFPE => b"[runner] FATAL: SIGFPE (floating point exception)\n",
        _ => b"[runner] FATAL: unknown signal\n",
    };
    unsafe { libc::write(2, msg.as_ptr() as *const libc::c_void, msg.len()) };
    unsafe { libc::_exit(128 + sig) };
}

#[cfg(unix)]
fn install_signal_handlers() {
    let handler = signal_handler as *const () as libc::sighandler_t;
    unsafe {
        libc::signal(libc::SIGSEGV, handler);
        libc::signal(libc::SIGBUS, handler);
        libc::signal(libc::SIGABRT, handler);
        libc::signal(libc::SIGILL, handler);
        libc::signal(libc::SIGFPE, handler);
    }
}

#[cfg(windows)]
fn install_signal_handlers() {
    // On Windows, winit and the OS handle structured exceptions.
    // No additional setup needed for the runner.
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.get(1).map(String::as_str) == Some("--namui-crash-server") {
        let socket_name = raw_args
            .get(2)
            .expect("Usage: native-runner --namui-crash-server <socket-path>");
        let Some(config) = native_runner::build_crash_config() else {
            eprintln!("[runner] --namui-crash-server requires NAMSH_* compile-time env");
            std::process::exit(1);
        };
        match namui_crash_reporter::server_main(socket_name, &config) {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                eprintln!("[runner] crash server error: {e}");
                std::process::exit(1);
            }
        }
    }

    let config = native_runner::build_crash_config();
    let _log_capture = config.as_ref().and_then(|_| {
        namui_crash_reporter::start_log_capture(native_runner::CRASH_APP_NAME)
            .inspect_err(|e| eprintln!("[runner] log_capture start failed: {e}"))
            .ok()
    });
    let _crash_guard = config.as_ref().and_then(|c| {
        namui_crash_reporter::init(c)
            .inspect_err(|e| eprintln!("[runner] crash-reporter init failed, falling back: {e}"))
            .ok()
    });
    if _crash_guard.is_none() {
        install_signal_handlers();
    }

    let args = raw_args;
    let dylib_path = args
        .get(1)
        .expect("Usage: native-runner <dylib-path> <project-path> <font-dir>");
    let project_path = args
        .get(2)
        .expect("Usage: native-runner <dylib-path> <project-path> <font-dir>");
    let font_dir = args
        .get(3)
        .expect("Usage: native-runner <dylib-path> <project-path> <font-dir>");
    let font_dir = std::path::Path::new(font_dir);

    // SAFETY: still single-threaded at this point (crash-reporter spawned its
    // child but not yet any in-process thread that reads the env).
    unsafe {
        std::env::set_var(
            "NAMUI_ASSET_DIR",
            std::path::Path::new(project_path).join("asset"),
        );
    }

    #[cfg(unix)]
    let _lib = match unsafe {
        libloading::os::unix::Library::open(
            Some(dylib_path.as_str()),
            libc::RTLD_LAZY | libc::RTLD_GLOBAL,
        )
    } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("[runner] Failed to load dylib: {e}");
            std::process::exit(1);
        }
    };

    #[cfg(windows)]
    let _lib = match unsafe { libloading::os::windows::Library::new(dylib_path.as_str()) } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("[runner] Failed to load dll: {e}");
            std::process::exit(1);
        }
    };

    // Note: do NOT install a panic hook here or wrap `run_with_font_dir` in
    // `catch_unwind`. crash-reporter::init() installs the panic→abort hook
    // that ferries Rust panics into the minidumper; overriding it or
    // swallowing the unwind would prevent the dump from ever being produced.
    native_runner::run_with_font_dir(font_dir);
}
