use crate::{util::recreate_dir_all, *};
use std::path::Path;

pub fn generate_dylib_wrapper_project(app_project_path: &Path) -> Result<()> {
    let app_name = get_app_name(app_project_path);
    let namui_rel_path = get_namui_relative_path(app_project_path)?;
    let wrapper_project_path = app_project_path.join("target/namui-native");

    recreate_dir_all(
        &wrapper_project_path,
        Some(vec![wrapper_project_path.join("target")]),
    )?;

    // The wrapper project is at {project}/target/namui-native/,
    // so we need ../../ prefix to reach the project root.
    let app_dep_path = "../../";
    // namui_rel_path is relative from the user project root,
    // so from wrapper it's ../../{namui_rel_path}
    let namui_dep_path = format!("../../{namui_rel_path}");

    std::fs::write(
        wrapper_project_path.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-native-dylib"
version = "0.0.1"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
{app_name} = {{ path = "{app_dep_path}" }}
namui = {{ path = "{namui_dep_path}" }}

[profile.dev]
opt-level = 1
strip = "debuginfo"
debug = "line-tables-only"
"#,
        ),
    )?;

    recreate_dir_all(wrapper_project_path.join("src"), None)?;

    let project_name_underscored = app_name.replace('-', "_");
    std::fs::write(
        wrapper_project_path.join("src/lib.rs"),
        format!(
            r#"#[unsafe(no_mangle)]
pub extern "C" fn namui_main() {{
    {project_name_underscored}::asset::init_native_assets();
    {project_name_underscored}::main();
}}
"#,
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

fn get_namui_relative_path(project_path: &Path) -> Result<String> {
    let manifest_path = project_path.join("Cargo.toml");
    let manifest_contents = std::fs::read_to_string(&manifest_path)?;

    // Parse namui = { path = "..." } from Cargo.toml
    for line in manifest_contents.lines() {
        let line = line.trim();
        if line.starts_with("namui") && line.contains("path") {
            // Extract the path value
            if let Some(path_start) = line.find("path") {
                let rest = &line[path_start..];
                if let Some(quote_start) = rest.find('"') {
                    let after_quote = &rest[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        return Ok(after_quote[..quote_end].to_string());
                    }
                }
            }
        }
    }

    anyhow::bail!(
        "Could not find namui path dependency in {}",
        manifest_path.display()
    )
}
