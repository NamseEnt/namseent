use crate::cli::Target;
use crate::*;
use services::build_status_service::{BuildStatusCategory, BuildStatusService};
use services::runtime_project::{wasm::generate_runtime_project, GenerateRuntimeProjectArgs};
use services::rust_build_service::{self, BuildOption};
use services::rust_project_watch_service::RustProjectWatchService;

pub async fn start(manifest_path: impl AsRef<std::path::Path>, release: bool) -> Result<()> {
    let manifest_path = manifest_path.as_ref();
    let target = Target::Wasm32WasiWeb;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let build_status_service = BuildStatusService::new();
    let runtime_target_dir = project_root_path.join("target/namui");
    let bundle_path = {
        let opt_level = match release {
            true => "release",
            false => "debug",
        };
        project_root_path
            .join("target")
            .join("namui")
            .join("target")
            .join("x86_64-pc-windows-msvc")
            .join(opt_level)
            .join("bundle.sqlite")
    };

    generate_runtime_project(GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
    })?;

    build_status_service
        .build_started(BuildStatusCategory::Namui)
        .await;
    let result = rust_build_service::build(BuildOption {
        target,
        project_root_path: runtime_target_dir.clone(),
        release,
        watch: true,
    })
    .await??;
    let bundle_manifest =
        crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;
    bundle_manifest.bundle_to_sqlite(&bundle_path)?;
    build_status_service
        .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
        .await;
    println!("In this environment, namui does not run built binary. It works like `build --watch`. Run the binary yourself");

    let mut rust_project_watch = RustProjectWatchService::new(manifest_path)?;

    while let Some(()) = rust_project_watch.next().await? {
        build_status_service
            .build_started(BuildStatusCategory::Namui)
            .await;
        let result = rust_build_service::build(BuildOption {
            target,
            project_root_path: runtime_target_dir.clone(),
            release,
            watch: true,
        })
        .await??;
        let bundle_manifest =
            crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;
        bundle_manifest.bundle_to_sqlite(&bundle_path)?;
        build_status_service
            .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
            .await;
    }

    Ok(())
}
