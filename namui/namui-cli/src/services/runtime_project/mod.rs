pub mod wasm;
pub mod x86_64_pc_windows_msvc;

use std::path::PathBuf;

pub struct GenerateRuntimeProjectArgs {
    pub target_dir: PathBuf,
    pub project_path: PathBuf,
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
