pub mod aarch64_apple_darwin;
pub mod wasm;
pub mod x86_64_pc_windows_msvc;

use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RuntimeProjectMode {
    /// Static linking, single binary (namui build)
    Binary,
    /// Dynamic library for hot reload (namui start)
    Cdylib,
}

pub struct GenerateRuntimeProjectArgs {
    pub target_dir: PathBuf,
    pub project_path: PathBuf,
    pub strip_debug_info: bool,
    pub mode: RuntimeProjectMode,
}

fn get_project_name(project_path: PathBuf) -> String {
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

/// Extract the namui dependency path from the user's Cargo.toml
fn get_namui_dep_path(project_path: &std::path::Path) -> Option<PathBuf> {
    let manifest_path = project_path.join("Cargo.toml");
    let manifest_contents = std::fs::read_to_string(manifest_path).ok()?;

    // Look for: namui = { path = "..." }
    for line in manifest_contents.lines() {
        let line = line.trim();
        if line.starts_with("namui") && line.contains("path") {
            // Extract path value
            if let Some(path_start) = line.find("path") {
                let rest = &line[path_start..];
                if let Some(quote_start) = rest.find('"') {
                    let after_quote = &rest[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        let path_str = &after_quote[..quote_end];
                        return Some(project_path.join(path_str));
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::wasm::generate_runtime_project;
    use crate::services::runtime_project::GenerateRuntimeProjectArgs;

    #[test]
    fn generate_runtime_project_wasm_should_work() {
        let project_path = std::env::temp_dir().join("namui-cli/test/wasm");
        let manifest_path = project_path.join("Cargo.toml");

        std::fs::create_dir_all(&project_path).unwrap();
        std::fs::write(
            manifest_path,
            r#"[package]
    name = "namui-runtime-wasm"#,
        )
        .unwrap();

        generate_runtime_project(GenerateRuntimeProjectArgs {
            target_dir: std::env::temp_dir(),
            project_path,
            strip_debug_info: false,
            mode: super::RuntimeProjectMode::Binary,
        })
        .unwrap();
    }

    #[test]
    fn get_project_name_should_work() {
        let project_path = std::env::temp_dir().join("namui-cli/test/wasm2");
        let manifest_path = project_path.join("Cargo.toml");

        std::fs::create_dir_all(&project_path).unwrap();
        std::fs::write(
            manifest_path,
            r#"[package]
    name = "namui-runtime-wasm"#,
        )
        .unwrap();

        assert_eq!("namui-runtime-wasm", super::get_project_name(project_path));
    }
}
