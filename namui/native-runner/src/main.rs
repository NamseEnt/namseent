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

const CRASH_APP_NAME: &str = "namui-game";

fn build_crash_config() -> Option<namui_crash_reporter::Config> {
    let build_id = option_env!("NAMUI_CRASH_BUILD_ID")?;
    let hmac_key_hex = option_env!("NAMUI_CRASH_HMAC_KEY")?;
    let namsh_url = option_env!("NAMUI_CRASH_NAMSH_URL")?;
    if build_id.is_empty() || hmac_key_hex.is_empty() || namsh_url.is_empty() {
        return None;
    }
    Some(namui_crash_reporter::Config {
        build_id: build_id.into(),
        hmac_key_hex: hmac_key_hex.into(),
        namsh_url: namsh_url.trim_end_matches('/').into(),
        app_name: CRASH_APP_NAME.into(),
    })
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.get(1).map(String::as_str) == Some("--namui-crash-server") {
        let socket_name = raw_args
            .get(2)
            .expect("Usage: native-runner --namui-crash-server <socket-path>");
        let Some(config) = build_crash_config() else {
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

    let _log_capture = if build_crash_config().is_some() {
        match namui_crash_reporter::start_log_capture(CRASH_APP_NAME) {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("[runner] log_capture start failed: {e}");
                None
            }
        }
    } else {
        None
    };

    let _crash_guard = match build_crash_config() {
        Some(config) => match namui_crash_reporter::init(&config) {
            Ok(guard) => Some(guard),
            Err(e) => {
                eprintln!("[runner] crash-reporter init failed, falling back: {e}");
                install_signal_handlers();
                None
            }
        },
        None => {
            install_signal_handlers();
            None
        }
    };

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

    std::panic::set_hook(Box::new(|info| {
        eprintln!("[runner] PANIC: {info}");
    }));
    let result = std::panic::catch_unwind(|| {
        native_runner::run_with_font_dir(font_dir);
    });
    if let Err(e) = result {
        eprintln!("[runner] run() panicked: {e:?}");
    }
}
