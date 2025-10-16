use crate::cli::Target;
use crate::*;
use services::build_status_service::{BuildStatusCategory, BuildStatusService};
use services::runtime_project::{GenerateRuntimeProjectArgs, wasm::generate_runtime_project};
use services::rust_build_service::{self, BuildOption};
use services::rust_project_watch_service::RustProjectWatchService;
use services::vite_config::{ViteConfig, prepare_vite_env};
use tokio::process::Child;
use util::get_cli_root_path;

pub async fn start(
    manifest_path: impl AsRef<std::path::Path>,
    start_option: StartOption,
) -> Result<()> {
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

    let vite_env_vars = prepare_vite_env(&vite_config).await?;

    let _web_builder = start_web_code(&vite_env_vars).await?;

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
        let _ = prepare_vite_env(&vite_config).await?;
    }

    Ok(())
}

async fn start_web_code(vite_env_vars: &services::vite_config::ViteEnvVars) -> Result<Child> {
    let mut process = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["ci"])
        .spawn()?;
    process.wait().await?;

    let process = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["run", "dev"])
        .env("NAMUI_RUNTIME_WASM_PATH", &vite_env_vars.namui_runtime_wasm_path)
        .env("NAMUI_CLI_ROOT", &vite_env_vars.namui_cli_root)
        .env("NAMUI_BUNDLE_SQLITE_PATH", &vite_env_vars.namui_bundle_sqlite_path)
        .env("NAMUI_DRAWER_WASM_PATH", &vite_env_vars.namui_drawer_wasm_path)
        .env("NAMUI_HOST", &vite_env_vars.namui_host)
        .env("NAMUI_ASSET_DIR", &vite_env_vars.namui_asset_dir)
        .env("NAMUI_TARGET_DIR", &vite_env_vars.namui_target_dir)
        .env("NAMUI_SERVER_ALLOW", &vite_env_vars.namui_server_allow)
        .env("NAMUI_SERVER_FS_ALLOW", &vite_env_vars.namui_server_fs_allow)
        .spawn()?;

    Ok(process)
}
