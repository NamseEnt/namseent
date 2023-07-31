use crate::{
    cli::Target,
    services::{
        resource_collect_service, wasm_watch_build_service::WasmWatchBuildService,
        wasm_web_runtime_prepare_service,
    },
    util::overwrite_hot_reload_script_with_empty_file,
};
use std::path::Path;

pub fn build(manifest_path: &Path) -> Result<(), crate::Error> {
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("wasm_unknown_web");

    wasm_web_runtime_prepare_service::build_browser_runtime()?;
    WasmWatchBuildService::just_build(project_root_path.clone(), Target::WasmUnknownWeb)?;

    let bundle_manifest =
        crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;

    resource_collect_service::collect_all(
        &project_root_path,
        &release_path,
        Target::WasmUnknownWeb,
        bundle_manifest,
        None,
    )?;

    overwrite_hot_reload_script_with_empty_file(&release_path)?;

    Ok(())
}
