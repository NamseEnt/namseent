fn main() {
    // Allow extern "C" symbols to be undefined at link time.
    // They are provided at runtime by the cdylib loaded via dlopen.
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg-bin=native-runner=-Wl,-undefined,dynamic_lookup");

    let crash_env_mappings = [
        ("NAMSH_BUILD_ID", "NAMUI_CRASH_BUILD_ID"),
        ("NAMSH_HMAC_KEY", "NAMUI_CRASH_HMAC_KEY"),
        ("NAMSH_URL", "NAMUI_CRASH_NAMSH_URL"),
    ];
    for (src, dst) in crash_env_mappings {
        println!("cargo:rerun-if-env-changed={src}");
        if let Ok(value) = std::env::var(src) {
            println!("cargo:rustc-env={dst}={value}");
        }
    }
}
