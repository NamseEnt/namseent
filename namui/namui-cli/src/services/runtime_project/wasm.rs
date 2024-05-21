use super::{get_project_name, GenerateRuntimeProjectArgs};
use crate::{util::recreate_dir_all, *};

pub fn generate_runtime_project(args: GenerateRuntimeProjectArgs) -> Result<()> {
    let project_name = get_project_name(args.project_path.clone());

    let project_path_in_relative =
        pathdiff::diff_paths(&args.project_path, &args.target_dir).unwrap();

    recreate_dir_all(&args.target_dir, Some(vec![args.target_dir.join("target")]))?;

    std::fs::write(
        args.target_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-runtime-wasm"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
{project_name} = {{ path = "{project_path}" }}
wasm-bindgen = "0.2"
namui-panic-hook = "0.1"

[package.metadata.wasm-pack.profile.dev.wasm-bindgen]
dwarf-debug-info = true

[profile.release]
lto = true
opt-level = 3

[profile.dev]
lto = true
opt-level = 2
    "#,
            project_path = project_path_in_relative
                .to_str()
                .unwrap()
                .split('\\')
                .collect::<Vec<&str>>()
                .join("/"),
        ),
    )?;

    // src
    {
        recreate_dir_all(args.target_dir.join("src"), None)?;

        std::fs::write(
            args.target_dir.join("src/lib.rs"),
            format!(
                r#"use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {{
    namui_panic_hook::set_once();

    {project_name_underscored}::main();
}}
"#,
                project_name_underscored = project_name.replace('-', "_"),
            ),
        )?;
    }

    // .cargo
    {
        recreate_dir_all(args.target_dir.join(".cargo"), None)?;

        std::fs::write(
            args.target_dir.join(".cargo/config.toml"),
            r#"[build]
# NOTE: This may break build when user's platform doesn't support simd128.
rustflags = ["-C", "target-feature=+simd128"]
"#,
        )?;
    }

    Ok(())
}
