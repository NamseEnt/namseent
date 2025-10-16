use crate::{services::bundle::NamuiBundleManifest, util::get_cli_root_path};
use anyhow::Result;
use std::path::Path;
use tokio::fs::{create_dir_all, remove_dir_all};

pub struct ViteConfig<'a> {
    pub project_root_path: &'a Path,
    pub release: bool,
    pub host: Option<String>,
}

pub struct ViteEnvVars {
    pub namui_runtime_wasm_path: String,
    pub namui_cli_root: String,
    pub namui_bundle_sqlite_path: String,
    pub namui_drawer_wasm_path: String,
    pub namui_host: String,
    pub namui_asset_dir: String,
    pub namui_target_dir: String,
    pub namui_server_allow: String,
    pub namui_server_fs_allow: String,
}

pub async fn prepare_vite_env(config: &ViteConfig<'_>) -> Result<ViteEnvVars> {
    let bundle_manifest = NamuiBundleManifest::parse(config.project_root_path)?;

    let target_project_path = config.project_root_path.join(format!(
        "target/namui/target/wasm32-wasip1-threads/{}",
        if config.release { "release" } else { "debug" }
    ));
    let namui_runtime_wasm_path = target_project_path.join("namui-runtime-wasm.wasm");
    let bundle_sqlite_path = target_project_path.join("bundle.sqlite");
    bundle_manifest.bundle_to_sqlite(&bundle_sqlite_path)?;

    let generated_dist = get_cli_root_path().join("webCode/src/__generated__");

    let _ = remove_dir_all(&generated_dist).await;
    create_dir_all(&generated_dist).await?;

    let asset_dir = config.project_root_path.join("asset");
    let cli_root = get_cli_root_path();
    let drawer_wasm_path = cli_root.join("../namui-drawer/target/wasm32-wasip1-threads/release/namui-drawer.wasm");

    // Prepare server.allow array
    let server_allow = serde_json::json!([
        namui_runtime_wasm_path.to_string_lossy().to_string(),
        format!("{}/", cli_root.to_string_lossy()),
    ]);

    // Prepare server.fs.allow array
    let server_fs_allow = serde_json::json!([
        "./",
        asset_dir.to_string_lossy().to_string(),
        target_project_path.to_string_lossy().to_string(),
        format!("{}/system_bundle", cli_root.to_string_lossy()),
    ]);

    Ok(ViteEnvVars {
        namui_runtime_wasm_path: namui_runtime_wasm_path.to_string_lossy().to_string(),
        namui_cli_root: cli_root.to_string_lossy().to_string(),
        namui_bundle_sqlite_path: bundle_sqlite_path.to_string_lossy().to_string(),
        namui_drawer_wasm_path: drawer_wasm_path.to_string_lossy().to_string(),
        namui_host: config.host.as_deref().unwrap_or("localhost").to_string(),
        namui_asset_dir: asset_dir.to_string_lossy().to_string(),
        namui_target_dir: target_project_path.to_string_lossy().to_string(),
        namui_server_allow: server_allow.to_string(),
        namui_server_fs_allow: server_fs_allow.to_string(),
    })
}

// Deprecated: Use prepare_vite_env instead
pub async fn update_vite_config(config: &ViteConfig<'_>) -> Result<()> {
    let _ = prepare_vite_env(config).await?;
    Ok(())
}
