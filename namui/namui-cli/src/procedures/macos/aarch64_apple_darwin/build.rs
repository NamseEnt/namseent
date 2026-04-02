use crate::cli::Target;
use crate::*;
use services::build_status_service::{BuildStatusCategory, BuildStatusService};
use services::runtime_project::{
    GenerateRuntimeProjectArgs, RuntimeProjectMode,
    aarch64_apple_darwin::generate_runtime_project,
};
use services::rust_build_service::{self, BuildOption};

pub async fn build(
    manifest_path: impl AsRef<std::path::Path>,
    release: bool,
) -> Result<()> {
    let manifest_path = manifest_path.as_ref();
    let target = Target::Aarch64AppleDarwin;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("aarch64-apple-darwin");
    let runtime_target_dir = project_root_path.join("target/namui");

    generate_runtime_project(GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
        strip_debug_info: false,
        mode: RuntimeProjectMode::Binary,
    })?;

    let build_status_service = BuildStatusService::new();

    build_status_service
        .build_started(BuildStatusCategory::Namui)
        .await;

    let result = rust_build_service::build(BuildOption {
        target,
        project_root_path: runtime_target_dir.clone(),
        release,
        watch: false,
    })
    .await??;

    build_status_service
        .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
        .await;

    if !result.is_successful {
        return Err(anyhow!("Build failed"));
    }

    let bundle_manifest =
        crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;

    services::resource_collect_service::collect_all(
        &project_root_path,
        &release_path,
        target,
        bundle_manifest,
        None,
        release,
    )?;

    Ok(())
}
