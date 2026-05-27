//! End-to-end smoke test for `namui-crash-reporter`.
//!
//! ```sh
//! NAMSH_BUILD_ID=smoke-XYZ \
//! NAMSH_HMAC_KEY=<hex> \
//! NAMSH_URL=https://g6vldebf.fn0.dev \
//!     cargo run --example smoke -p namui-crash-reporter
//! ```
//!
//! Parent → spawns self as `--namui-crash-server` → installs crash handler →
//! sleeps briefly → null-deref triggers SIGSEGV → child writes the minidump,
//! parses out `stack_hash`, and POSTs `intake_crash` (+ R2 PUT).

fn config() -> namui_crash_reporter::Config {
    namui_crash_reporter::Config {
        build_id: std::env::var("NAMSH_BUILD_ID").expect("set NAMSH_BUILD_ID"),
        hmac_key_hex: std::env::var("NAMSH_HMAC_KEY").expect("set NAMSH_HMAC_KEY"),
        namsh_url: std::env::var("NAMSH_URL")
            .expect("set NAMSH_URL")
            .trim_end_matches('/')
            .to_string(),
        app_name: "namui-crash-reporter-smoke".to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--namui-crash-server") {
        let socket = args
            .get(2)
            .expect("--namui-crash-server <socket-path>");
        if let Err(e) = namui_crash_reporter::server_main(socket, &config()) {
            eprintln!("[smoke:child] {e}");
            std::process::exit(1);
        }
        return;
    }

    let cfg = config();
    let _log = namui_crash_reporter::start_log_capture(&cfg.app_name)
        .map_err(|e| eprintln!("[smoke] log_capture failed: {e}"))
        .ok();
    let _guard = namui_crash_reporter::init(&cfg).expect("init");
    println!("[smoke] hello from stdout (this line should appear in log_tail)");
    eprintln!("[smoke] hello from stderr (this line too)");
    eprintln!("[smoke] crash-reporter initialized; triggering SIGSEGV in 500ms…");
    std::thread::sleep(std::time::Duration::from_millis(500));
    unsafe {
        std::ptr::null_mut::<i32>().write_volatile(42);
    }
}
