use super::GenerateRuntimeProjectArgs;

pub fn generate_runtime_project(
    args: GenerateRuntimeProjectArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_name = args.project_path.file_name().unwrap().to_str().unwrap();

    std::fs::create_dir_all(&args.target_dir.join("src"))?;

    let cargo_toml = format!(
        r#"[package]
name = "namui-runtime-wasm"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
{project_name} = {{ path = "{project_path}" }}
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
console_error_panic_hook = "0.1"

[profile.release]
lto = true
opt-level = 3

[profile.dev]
lto = true
opt-level = 2
    "#,
        project_path = args.project_path.display(),
    );
    std::fs::write(args.target_dir.join("Cargo.toml"), cargo_toml)?;

    let lib_rs = format!(
        r#"use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {{
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    {project_name_underscored}::main().await;
}}
"#,
        project_name_underscored = project_name.replace("-", "_"),
    );
    std::fs::write(args.target_dir.join("src/lib.rs"), lib_rs)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::services::runtime_project::GenerateRuntimeProjectArgs;

    use super::generate_runtime_project;

    #[test]
    fn generate_runtime_project_wasm_should_work() {
        let project_path = std::env::temp_dir().join("namui-cli/test/wasm");
        let manifest_path = project_path.join("Cargo.toml");

        std::fs::create_dir_all(&project_path).unwrap();
        std::fs::write(
            manifest_path,
            format!(
                r#"[package]
    name = "namui-runtime-wasm"#
            ),
        )
        .unwrap();

        generate_runtime_project(GenerateRuntimeProjectArgs {
            target_dir: std::env::temp_dir(),
            project_path,
        })
        .unwrap();
    }

    #[test]
    fn get_project_name_should_work() {
        let project_path = std::env::temp_dir().join("namui-cli/test/wasm");
        let manifest_path = project_path.join("Cargo.toml");

        std::fs::create_dir_all(&project_path).unwrap();
        std::fs::write(
            manifest_path,
            format!(
                r#"[package]
    name = "namui-runtime-wasm"#
            ),
        )
        .unwrap();

        assert_eq!("namui-runtime-wasm", super::get_project_name(project_path));
    }
}
