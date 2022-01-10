use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct RunWasmPackOption {
    pub root_dir: String,
}

pub struct RunWasmPackResult {
    pub result_js_path: String,
    pub result_wasm_path: String,
}

#[derive(Debug)]
pub enum RunWasmPackError {
    IoError(std::io::Error),
}

pub fn run_wasm_pack(option: RunWasmPackOption) -> Result<RunWasmPackResult, RunWasmPackError> {
    let mut out_dir = PathBuf::from(&option.root_dir);
    out_dir.push("pkg");
    let output = Command::new("wasm-pack")
        .args([
            "build",
            "--target",
            "no-modules",
            "--out-name",
            "bundle",
            "--dev",
            option.root_dir.as_str(),
        ])
        .stdout(Stdio::null())
        .output();

    let result_wasm_path = out_dir.join("bundle_bg.wasm").to_string_lossy().to_string();
    let result_js_path = out_dir.join("bundle.js").to_string_lossy().to_string();

    match output {
        Ok(_) => Ok(RunWasmPackResult {
            result_js_path,
            result_wasm_path,
        }),
        Err(error) => Err(RunWasmPackError::IoError(error)),
    }
}
