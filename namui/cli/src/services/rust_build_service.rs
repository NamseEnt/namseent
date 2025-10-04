use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::process::Command;

pub async fn build_wasm(project_root: &Path) -> Result<PathBuf> {
    let cli_root = get_cli_root();
    let wasi_sdk_path = cli_root.join("wasi-sdk");

    // rustflags 구성
    let rustflags = format!(
        "-Ctarget-feature=-crt-static \
        -L{}/share/wasi-sysroot/lib/wasm32-wasip1-threads \
        -L{}/lib/clang/19/lib/wasip1 \
        -Clink-arg=--initial-memory=8388608 \
        -Clink-arg=--max-memory=4294967296 \
        -Clink-arg=--stack-first \
        -Clink-arg=--export=__heap_base \
        -Clink-arg=--export=__data_end \
        -Clink-arg=--export=malloc \
        -Clink-arg=--export=free \
        -Ctarget-feature=+simd128",
        wasi_sdk_path.display(),
        wasi_sdk_path.display()
    );

    let output = Command::new("cargo")
        .current_dir(project_root)
        .args(["build", "--target", "wasm32-wasip1-threads"])
        .env("RUSTFLAGS", rustflags)
        .env("CLANGCC", wasi_sdk_path.join("bin/clang"))
        .env("CLANGCXX", wasi_sdk_path.join("bin/clang++"))
        .env("CC", wasi_sdk_path.join("bin/clang"))
        .env("CXX", wasi_sdk_path.join("bin/clang++"))
        .env("WASI_SDK", &wasi_sdk_path)
        .env("WASI_SYSROOT", wasi_sdk_path.join("share/wasi-sysroot"))
        .env("CLANG_PATH", wasi_sdk_path.join("bin/clang"))
        .env("CARGO_TARGET_WASM32_WASIP1_THREADS_LINKER", wasi_sdk_path.join("bin/wasm-ld"))
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Build failed:\n{}", stderr);
    }

    // wasm 파일 경로 찾기
    let wasm_path = project_root
        .join("target/wasm32-wasip1-threads/debug")
        .read_dir()?
        .filter_map(|entry| entry.ok())
        .find(|entry| {
            entry.path().extension().and_then(|s| s.to_str()) == Some("wasm")
        })
        .map(|entry| entry.path())
        .ok_or_else(|| anyhow::anyhow!("WASM file not found"))?;

    println!("✓ WASM built: {}", wasm_path.display());
    Ok(wasm_path)
}

fn get_cli_root() -> PathBuf {
    // 빌드 시점에 설정된 manifest 디렉토리 사용
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
