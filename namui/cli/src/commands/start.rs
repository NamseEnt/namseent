use anyhow::Result;
use std::path::Path;
use tokio::process::Command;

use crate::services::rust_build_service::build_wasm;
use crate::services::rust_project_watch_service::RustProjectWatchService;

pub async fn start(project_root: &Path) -> Result<()> {
    println!("🚀 Starting development server...\n");

    let manifest_path = project_root.join("Cargo.toml");
    if !manifest_path.exists() {
        anyhow::bail!("Cargo.toml not found in {}", project_root.display());
    }

    // 초기 빌드
    println!("📦 Building WASM...");
    let wasm_path = build_wasm(project_root).await?;

    // npm install 확인
    let cli_root = get_cli_root();
    let webcode_dir = cli_root.join("webCode");

    if !webcode_dir.join("node_modules").exists() {
        println!("📥 Installing npm dependencies...");
        let status = Command::new("npm")
            .current_dir(&webcode_dir)
            .args(["install"])
            .status()
            .await?;

        if !status.success() {
            anyhow::bail!("npm install failed");
        }
    }

    // vite 서버 시작
    println!("🌐 Starting Vite server...");
    let _vite_process = Command::new("npm")
        .current_dir(&webcode_dir)
        .args(["run", "dev"])
        .env("WASM_PATH", wasm_path.to_string_lossy().to_string())
        .spawn()?;

    // 파일 감시
    let mut watch_service = RustProjectWatchService::new(&manifest_path)?;

    println!("\n👀 Watching for changes...\n");

    while let Some(()) = watch_service.next().await? {
        println!("🔄 Changes detected, rebuilding...");
        match build_wasm(project_root).await {
            Ok(wasm_path) => {
                println!("✓ WASM rebuilt: {}", wasm_path.display());
                println!("   HMR will reload the module\n");
            }
            Err(e) => {
                eprintln!("❌ Build failed: {}\n", e);
            }
        }
    }

    Ok(())
}

fn get_cli_root() -> std::path::PathBuf {
    // 빌드 시점에 설정된 manifest 디렉토리 사용
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
