use crate::cli::Target;
use crate::services::wasi_cargo_envs::{WasiType, wasi_cargo_envs};
use crate::*;
use services::build_status_service::{BuildStatusCategory, BuildStatusService};
use services::runtime_project::{GenerateRuntimeProjectArgs, wasm::generate_runtime_project};
use services::rust_build_service::{self, BuildOption};
use services::rust_project_watch_service::RustProjectWatchService;
use services::vite_config::{ViteConfig, update_vite_config};
use tokio::process::Child;
use util::get_cli_root_path;

pub async fn start(
    manifest_path: impl AsRef<std::path::Path>,
    start_option: StartOption,
) -> Result<()> {
    build_drawer().await?;

    let manifest_path = manifest_path.as_ref();
    let target = Target::Wasm32WasiWeb;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let build_status_service = BuildStatusService::new();
    let runtime_target_dir = project_root_path.join("target/namui");

    generate_runtime_project(GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
        strip_debug_info: start_option.strip_debug_info,
    })?;

    build_status_service
        .build_started(BuildStatusCategory::Namui)
        .await;

    let result = rust_build_service::build(BuildOption {
        target,
        project_root_path: runtime_target_dir.clone(),
        release: start_option.release,
        watch: true,
    })
    .await??;

    build_status_service
        .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
        .await;

    let vite_config = ViteConfig {
        project_root_path: &project_root_path,
        release: start_option.release,
        host: start_option.host,
    };

    build_status_service
        .build_started(BuildStatusCategory::WebRuntime)
        .await;

    update_vite_config(&vite_config).await?;

    let target_project_path = project_root_path.join(format!(
        "target/namui/target/wasm32-wasip1-threads/{}",
        if start_option.release { "release" } else { "debug" }
    ));
    let namui_runtime_wasm_path = target_project_path.join("namui-runtime-wasm.wasm");

    let _web_builder = start_web_code(&namui_runtime_wasm_path).await?;

    build_status_service
        .build_finished(BuildStatusCategory::WebRuntime, vec![], vec![])
        .await;

    let mut rust_project_watch = RustProjectWatchService::new(manifest_path)?;

    while let Some(()) = rust_project_watch.next().await? {
        build_status_service
            .build_started(BuildStatusCategory::Namui)
            .await;
        let result = rust_build_service::build(BuildOption {
            target,
            project_root_path: runtime_target_dir.clone(),
            release: start_option.release,
            watch: true,
        })
        .await??;
        build_status_service
            .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
            .await;
        update_vite_config(&vite_config).await?;
    }

    Ok(())
}

async fn start_web_code(namui_runtime_wasm_path: &std::path::Path) -> Result<Child> {
    let npm_check = tokio::process::Command::new("npm")
        .arg("--version")
        .output()
        .await;

    if npm_check.is_err() {
        return Err(anyhow::anyhow!(
            "npm is not installed. Please install Node.js"
        ));
    }
    let mut process = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["ci"])
        .spawn()?;
    process.wait().await?;

    let process = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["run", "dev"])
        .env("NAMUI_RUNTIME_WASM_PATH", namui_runtime_wasm_path.to_string_lossy().to_string())
        .spawn()?;

    Ok(process)
}

async fn build_drawer() -> Result<()> {
    let drawer_target_dir = get_cli_root_path().join("../namui-drawer");
    let output = tokio::process::Command::new("cargo")
        .args(["build", "--target", "wasm32-wasip1-threads", "--release"])
        .current_dir(drawer_target_dir)
        .envs(wasi_cargo_envs(WasiType::Drawer))
        .spawn()?
        .wait_with_output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to build drawer {}", stderr));
    }

    Ok(())
}
