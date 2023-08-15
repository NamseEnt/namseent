use super::{
    build_status_service::BuildStatusService,
    electron_package_service::{Arch, ElectronPackageService, Platform},
    wasm_watch_build_service::WasmWatchBuildService,
};
use crate::cli::Target;
use crate::*;
use std::path::Path;

pub fn build(manifest_path: &Path, arch: Option<Arch>, platform: Platform) -> Result<()> {
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let build_status_service = BuildStatusService::new();

    let target = match platform {
        Platform::Win32 => Target::WasmWindowsElectron,
        Platform::Linux => Target::WasmLinuxElectron,
    };

    let package_result = ElectronPackageService::package_electron_app(arch, platform)?;

    let release_path = project_root_path.join("target").join("namui").join(format!(
        "wasm_{platform}_{arch}_electron",
        platform = (match platform {
            Platform::Win32 => "windows",
            Platform::Linux => "linux",
        }),
        arch = package_result.arch
    ));

    WasmWatchBuildService::just_build(build_status_service, project_root_path.clone(), target)?;

    let namui_bundle_manifest =
        super::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;

    super::resource_collect_service::collect_all(
        &project_root_path,
        &release_path,
        target,
        namui_bundle_manifest,
        Some(&package_result.output_path.into()),
    )?;

    Ok(())
}
