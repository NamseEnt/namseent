use crate::util::get_cli_root_path;
use std::path::PathBuf;

pub fn wasi_cargo_envs() -> [(&'static str, PathBuf); 7] {
    let cli_root_path = get_cli_root_path();

    [
        ("CLANGCC", cli_root_path.join("wasi-sdk/bin/clang")),
        ("CLANGCXX", cli_root_path.join("wasi-sdk/bin/clang++")),
        ("CC", cli_root_path.join("wasi-sdk/bin/clang")),
        ("CXX", cli_root_path.join("wasi-sdk/bin/clang++")),
        ("WASI_SDK_PATH", cli_root_path.join("wasi-sdk")),
        ("CLANG_PATH", cli_root_path.join("wasi-sdk/bin/clang")),
        (
            "CARGO_TARGET_WASM32_WASIP1_THREADS_LINKER",
            cli_root_path.join("wasi-sdk/bin/wasm-ld"),
        ),
    ]
}
