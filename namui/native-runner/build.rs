fn main() {
    // Allow extern "C" symbols to be undefined at link time.
    // They are provided at runtime by the cdylib loaded via dlopen.
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg-bin=native-runner=-Wl,-undefined,dynamic_lookup");
}
