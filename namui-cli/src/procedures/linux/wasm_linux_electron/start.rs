use crate::services::wasm_watch_build_service::{StartArgs, WasmWatchBuildService};
use crate::{
    services::electron_dev_service::{start_electron_dev_service, CrossPlatform},
    util::NamuiDeepLinkManifest,
};
use std::path::Path;

pub fn start(manifest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    const PORT: u16 = 8080;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();

    let deep_link_schemes = match NamuiDeepLinkManifest::try_load(&project_root_path)? {
        Some(namui_deep_link_manifest) => namui_deep_link_manifest.deep_link_schemes().clone(),
        None => Vec::new(),
    };
    start_electron_dev_service(
        &PORT,
        CrossPlatform::None,
        &project_root_path,
        &deep_link_schemes,
    )?;
    WasmWatchBuildService::start(StartArgs {
        project_root_path,
        port: PORT,
    })
}
