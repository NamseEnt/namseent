use crate::{util::recreate_dir_all, *};
use std::path::Path;

pub fn generate_app_wrapper_project(app_project_path: &Path) -> Result<()> {
    let app_name = get_app_name(app_project_path);
    let wrapper_project_path = app_project_path.join("target/namui");

    recreate_dir_all(
        &wrapper_project_path,
        Some(vec![wrapper_project_path.join("target")]),
    )?;

    std::fs::write(
        wrapper_project_path.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-runtime-wasm"
version = "0.0.1"
edition = "2024"

[dependencies]
{app_name} = {{ path = "../../" }}

[profile.release]
opt-level = 3
lto = true
codegen-units = 1 # Reduce number of codegen units to increase optimizations

[profile.dev]
opt-level = 1
# opt-level = 3
"#,
        ),
    )?;

    recreate_dir_all(wrapper_project_path.join("src"), None)?;

    std::fs::write(
        wrapper_project_path.join("src/main.rs"),
        format!(
            r#"
fn main() {{
    {project_name_underscored}::main();
}}
"#,
            project_name_underscored = app_name.replace('-', "_"),
        ),
    )?;

    Ok(())
}

fn get_app_name(project_path: &Path) -> String {
    let manifest_path = project_path.join("Cargo.toml");
    let manifest_contents = std::fs::read_to_string(manifest_path).unwrap();
    manifest_contents
        .split("name = ")
        .nth(1)
        .unwrap()
        .split('\n')
        .next()
        .unwrap()
        .trim()
        .replace('"', "")
        .to_string()
}
