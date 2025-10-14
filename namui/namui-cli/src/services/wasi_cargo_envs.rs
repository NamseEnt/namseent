use crate::util::get_cli_root_path;
use itertools::Itertools;

pub enum WasiType {
    App,
    Drawer,
}
pub fn wasi_cargo_envs(wasi_type: WasiType) -> [(&'static str, String); 10] {
    let cli_root_path = get_cli_root_path();
    [
        (
            "CLANGCC",
            cli_root_path
                .join("wasi-sdk/bin/clang")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "CLANGCXX",
            cli_root_path
                .join("wasi-sdk/bin/clang++")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "CC",
            cli_root_path
                .join("wasi-sdk/bin/clang")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "CXX",
            cli_root_path
                .join("wasi-sdk/bin/clang++")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "WASI_SDK",
            cli_root_path.join("wasi-sdk").to_str().unwrap().to_string(),
        ),
        (
            "WASI_SYSROOT",
            cli_root_path
                .join("wasi-sdk/share/wasi-sysroot")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "EMSDK_SYSTEM_INCLUDE",
            cli_root_path
                .join("emscripten/system/include")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "CLANG_PATH",
            cli_root_path
                .join("wasi-sdk/bin/clang")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "CARGO_TARGET_WASM32_WASIP1_THREADS_LINKER",
            cli_root_path
                .join("wasi-sdk/bin/clang++")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        (
            "RUSTFLAGS",
            [
                "-Ctarget-feature=-crt-static".to_string(),
                format!(
                    "-Clink-arg=-L{}",
                    cli_root_path
                        .join("wasi-sdk/share/wasi-sysroot/lib/wasm32-wasip1-threads")
                        .to_str()
                        .unwrap()
                ),
                format!(
                    "-Clink-arg=-L{}",
                    cli_root_path
                        .join("wasi-sdk/lib/clang/19/lib/wasip1")
                        .to_str()
                        .unwrap()
                ),
                format!(
                    "-Clink-arg=--sysroot={}",
                    cli_root_path
                        .join("wasi-sdk/share/wasi-sysroot")
                        .to_str()
                        .unwrap()
                ),
                "-Clink-arg=-lwasi-emulated-process-clocks".to_string(),
                "-Clink-arg=-lwasi-emulated-signal".to_string(),
                "-Clink-arg=-Wl,--initial-memory=8388608".to_string(),
                "-Clink-arg=-Wl,--max-memory=4294967296".to_string(),
                "-Clink-arg=-Wl,--stack-first".to_string(),
                "-Clink-arg=-Wl,--export=__heap_base".to_string(),
                "-Clink-arg=-Wl,--export=__data_end".to_string(),
                "-Clink-arg=-Wl,--export=malloc".to_string(),
                "-Clink-arg=-Wl,--export=free".to_string(),
                match wasi_type {
                    WasiType::App => ["_on_event"].iter(),
                    WasiType::Drawer => ["_register_image", "_malloc_image_buffer"].iter(),
                }
                .map(|e| format!("-Clink-arg=-Wl,--export={}", e))
                .join(" "),
                "-Ctarget-feature=+simd128".to_string(),
            ]
            .join(" "),
        ),
    ]
}
