use crate::cli::Target;
use crate::*;
use services::build_status_service::{BuildStatusCategory, BuildStatusService};
use services::bundle::NamuiBundleManifest;
use services::runtime_project::{wasm::generate_runtime_project, GenerateRuntimeProjectArgs};
use services::rust_build_service::{self, BuildOption};
use services::rust_project_watch_service::RustProjectWatchService;
use std::path::Path;
use tokio::fs::{create_dir_all, remove_dir_all};
use tokio::process::Child;
use util::get_cli_root_path;

pub async fn start(manifest_path: impl AsRef<std::path::Path>, release: bool) -> Result<()> {
    let manifest_path = manifest_path.as_ref();
    let target = Target::Wasm32WasiWeb;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let build_status_service = BuildStatusService::new();
    let runtime_target_dir = project_root_path.join("target/namui");

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

    build_status_service
        .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
        .await;

    let vite_config = ViteConfig {
        project_root_path: &project_root_path,
        release,
    };

    build_status_service
        .build_started(BuildStatusCategory::WebRuntime)
        .await;

    update_vite_config(&vite_config).await?;
    let _web_code = start_web_code().await?;

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
            release,
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

async fn start_web_code() -> Result<Child> {
    let mut process = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["ci"])
        .spawn()?;
    process.wait().await?;

    let process = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["run", "dev"])
        .spawn()?;

    Ok(process)
}

struct ViteConfig<'a> {
    project_root_path: &'a Path,
    release: bool,
}
async fn update_vite_config(config: &ViteConfig<'_>) -> Result<()> {
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

    tokio::fs::write(
        get_cli_root_path().join("webCode/vite.config.js"),
        format!(
            r#"
import {{ defineConfig }} from "vite";

export default defineConfig({{
    clearScreen: false,
    server: {{
        headers: {{
            "Cross-Origin-Resource-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin",
        }},
        allow: [
            "{namui_runtime_wasm}",
            "{cli_root}/",
        ],
    }},
    resolve: {{
        alias: {{
            "namui-runtime-wasm.wasm?url": "{namui_runtime_wasm}?url",
            "bundle.sqlite?url": "{bundle_sqlite}?url",
        }}
    }},
}});
"#,
            namui_runtime_wasm = namui_runtime_wasm_path.to_string_lossy(),
            cli_root = get_cli_root_path().to_string_lossy(),
            bundle_sqlite = bundle_sqlite_path.to_string_lossy(),
        ),
    )
    .await?;

    Ok(())
}
