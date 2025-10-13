use crate::{services::bundle::NamuiBundleManifest, util::get_cli_root_path};
use anyhow::Result;
use std::path::Path;
use tokio::fs::{create_dir_all, remove_dir_all};

pub struct ViteConfig<'a> {
    pub project_root_path: &'a Path,
    pub release: bool,
    pub host: Option<String>,
}
pub async fn update_vite_config(config: &ViteConfig<'_>) -> Result<()> {
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
import expressPlugin from './expressPlugin'
import path from 'path'

export default defineConfig({{
    clearScreen: false,
    server: {{
        headers: {{
            "Cross-Origin-Resource-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin",
            "Referrer-Policy": "no-referrer-when-downgrade",
        }},
        allow: [
            "{namui_runtime_wasm}",
            "{cli_root}/",
        ],
        host: "{host}",
    }},
    resolve: {{
        alias: {{
            "namui-runtime-wasm.wasm?url": "{namui_runtime_wasm}?url",
            "bundle.sqlite?url": "{bundle_sqlite}?url",
            "namui-drawer.wasm?url": "{drawer_runtime_wasm}?url",
            "@": path.resolve(__dirname, "./src"),
        }},
    }},
    plugins: [
        expressPlugin(),
    ],
}});
"#,
            namui_runtime_wasm = namui_runtime_wasm_path.to_string_lossy(),
            cli_root = get_cli_root_path().to_string_lossy(),
            bundle_sqlite = bundle_sqlite_path.to_string_lossy(),
            drawer_runtime_wasm = get_cli_root_path()
                .join("../namui-drawer/target/wasm32-wasip1-threads/release/namui-drawer.wasm")
                .to_string_lossy(),
            host = config.host.as_deref().unwrap_or("localhost"),
        ),
    )
    .await?;

    Ok(())
}
