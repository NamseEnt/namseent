use crate::cli::Target;
use crate::services::wasm_watch_build_service::{WasmWatchBuildService, WatchAndBuildArgs};
use crate::{
    services::electron_dev_service::{start_electron_dev_service, CrossPlatform},
    util::NamuiDeepLinkManifest,
};
use std::path::Path;
use wsl::is_wsl;

pub fn start(manifest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !is_wsl() {
        return Err(format!("linux to windows build is only supported on wsl for now").into());
    }
    const PORT: u16 = 8080;

    let project_root_path = manifest_path.parent().unwrap().to_path_buf();

    let deep_link_schemes = match NamuiDeepLinkManifest::try_load(&project_root_path)? {
        Some(namui_deep_link_manifest) => namui_deep_link_manifest.deep_link_schemes().clone(),
        None => Vec::new(),
    };
    WasmWatchBuildService::watch_and_build(WatchAndBuildArgs {
        project_root_path: project_root_path.clone(),
        port: PORT,
        target: Target::WasmWindowsElectron,
        after_first_build: Some(move || {
            start_electron_dev_service(
                &PORT,
                CrossPlatform::WslToWindows,
                &project_root_path,
                &deep_link_schemes,
            )
            .unwrap();
        }),
    })
}
