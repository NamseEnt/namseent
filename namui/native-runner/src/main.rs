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
    unsafe {
        libc::signal(libc::SIGSEGV, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGBUS, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGABRT, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGILL, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGFPE, signal_handler as libc::sighandler_t);
    }
}

#[cfg(windows)]
fn install_signal_handlers() {
    // On Windows, winit and the OS handle structured exceptions.
    // No additional setup needed for the runner.
}

fn main() {
    install_signal_handlers();
    let args: Vec<String> = std::env::args().collect();
    let dylib_path = args
        .get(1)
        .expect("Usage: native-runner <dylib-path> <project-path> <font-dir>");
    let font_dir = args
        .get(3)
        .expect("Usage: native-runner <dylib-path> <project-path> <font-dir>");
    let font_dir = std::path::Path::new(font_dir);

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
