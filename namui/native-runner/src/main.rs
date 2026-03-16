#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

fn install_signal_handlers() {
    unsafe {
        libc::signal(libc::SIGSEGV, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGBUS, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGABRT, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGILL, signal_handler as libc::sighandler_t);
        libc::signal(libc::SIGFPE, signal_handler as libc::sighandler_t);
    }
}

fn main() {
    install_signal_handlers();
    let dylib_path = std::env::args()
        .nth(1)
        .expect("Usage: native-runner <dylib-path>");

    eprintln!("[runner] dylib path: {dylib_path}");
    eprintln!("[runner] dylib exists: {}", std::path::Path::new(&dylib_path).exists());

    // Load the dylib with RTLD_GLOBAL so that its symbols are visible to
    // the dynamic linker for -undefined dynamic_lookup resolution.
    // RTLD_LOCAL (the default) would keep symbols private, leaving extern "C"
    // declarations as null pointers.
    let _lib = match unsafe {
        libloading::os::unix::Library::open(
            Some(&dylib_path),
            libc::RTLD_LAZY | libc::RTLD_GLOBAL,
        )
    } {
        Ok(lib) => {
            eprintln!("[runner] dylib loaded successfully (RTLD_GLOBAL)");
            lib
        }
        Err(e) => {
            eprintln!("[runner] Failed to load dylib: {e}");
            std::process::exit(1);
        }
    };

    // Verify key symbols are available in the loaded dylib
    unsafe {
        let check_sym = |name: &[u8]| {
            let name_str = std::str::from_utf8(name).unwrap();
            match _lib.get::<*const ()>(name) {
                Ok(_) => eprintln!("[runner] symbol '{name_str}' found"),
                Err(e) => eprintln!("[runner] symbol '{name_str}' MISSING: {e}"),
            }
        };
        check_sym(b"namui_init_system");
        check_sym(b"namui_set_screen_size");
        check_sym(b"namui_on_screen_redraw");
        check_sym(b"namui_on_screen_resize");
        check_sym(b"namui_on_mouse_move");
    }

    eprintln!("[runner] calling native_runner::run()");
    std::panic::set_hook(Box::new(|info| {
        eprintln!("[runner] PANIC: {info}");
    }));
    let result = std::panic::catch_unwind(|| {
        native_runner::run();
    });
    match result {
        Ok(()) => eprintln!("[runner] run() returned normally"),
        Err(e) => eprintln!("[runner] run() panicked: {e:?}"),
    }
}
