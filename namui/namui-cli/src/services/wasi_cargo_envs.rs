use crate::util::get_cli_root_path;
use std::path::PathBuf;

pub fn wasi_cargo_envs() -> [(&'static str, PathBuf); 9] {
    let cli_root_path = get_cli_root_path();

    [
        (
            "CLANGCC_wasm32-wasip1-threads",
            cli_root_path.join("wasi-sdk/bin/clang"),
        ),
        (
            "CLANGCXX_wasm32-wasip1-threads",
            cli_root_path.join("wasi-sdk/bin/clang++"),
        ),
        (
            "CC_wasm32-wasip1-threads",
            cli_root_path.join("wasi-sdk/bin/clang"),
        ),
        (
            "CXX_wasm32-wasip1-threads",
            cli_root_path.join("wasi-sdk/bin/clang++"),
        ),
        ("WASI_SDK", cli_root_path.join("wasi-sdk")),
        (
            "WASI_SYSROOT",
            cli_root_path.join("wasi-sdk/share/wasi-sysroot"),
        ),
        (
            "EMSDK_SYSTEM_INCLUDE",
            cli_root_path.join("emscripten/system/include"),
        ),
        ("CLANG_PATH", cli_root_path.join("wasi-sdk/bin/clang")),
        (
            "CARGO_TARGET_WASM32_WASIP1_THREADS_LINKER",
            cli_root_path.join("wasi-sdk/bin/wasm-ld"),
        ),
    ]
}
