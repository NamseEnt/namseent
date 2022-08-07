pub mod wasm;

use std::path::PathBuf;

pub struct GenerateRuntimeProjectArgs {
    pub target_dir: PathBuf,
    pub project_path: PathBuf,
}
