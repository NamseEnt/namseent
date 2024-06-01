use crate::cli::Target;
use crate::services::build_status_service::BuildStatusService;
use crate::*;
use services::runtime_project::{wasm::generate_runtime_project, GenerateRuntimeProjectArgs};
use services::rust_build_service::{self, BuildOption};
use services::rust_project_watch_service::RustProjectWatchService;
use std::path::Path;
use tokio::process::Child;
use util::get_cli_root_path;

pub async fn start(manifest_path: &Path, release: bool) -> Result<()> {
    let target = Target::Wasm32WasiWeb;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let build_status_service = BuildStatusService::new();
    let runtime_target_dir = project_root_path.join("target/namui");

    generate_runtime_project(GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
    })?;

    build_status_service
        .build_started(services::build_status_service::BuildStatusCategory::Namui)
        .await;
    let result = rust_build_service::build(BuildOption {
        target,
        project_root_path: runtime_target_dir.clone(),
        release,
        watch: true,
    })
    .await??;

    build_status_service
        .build_finished(
            services::build_status_service::BuildStatusCategory::Namui,
            result.error_messages,
            vec![],
        )
        .await;

    build_status_service
        .build_started(services::build_status_service::BuildStatusCategory::WebRuntime)
        .await;

    let vite_config = ViteConfig {
        project_root_path: &project_root_path,
        release,
    };

    update_vite_config(&vite_config).await?;
    let _web_code = start_web_code().await?;

    build_status_service
        .build_finished(
            services::build_status_service::BuildStatusCategory::WebRuntime,
            vec![],
            vec![],
        )
        .await;

    let mut rust_project_watch = RustProjectWatchService::new(manifest_path)?;

    while let Some(()) = rust_project_watch.next().await? {
        build_status_service
            .build_started(services::build_status_service::BuildStatusCategory::Namui)
            .await;
        let result = rust_build_service::build(BuildOption {
            target,
            project_root_path: runtime_target_dir.clone(),
            release,
            watch: true,
        })
        .await??;
        build_status_service
            .build_finished(
                services::build_status_service::BuildStatusCategory::Namui,
                result.error_messages,
                vec![],
            )
            .await;
        update_vite_config(&vite_config).await?;
    }

    Ok(())
}

async fn start_web_code() -> Result<Child> {
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
    let namui_runtime_wasm_path = config.project_root_path.join(format!(
        "target/namui/target/wasm32-wasip1-threads/{}/namui-runtime-wasm.wasm",
        if config.release { "release" } else { "debug" }
    ));

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
    }},
    resolve: {{
        alias: {{
            "namui-runtime-wasm.wasm?url": "{namui_runtime_wasm}?url",
        }}
    }},
}});
"#,
            namui_runtime_wasm = namui_runtime_wasm_path.to_string_lossy()
        ),
    )
    .await?;

    Ok(())
}
