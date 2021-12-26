use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct RunWasmBindgenOption {
    pub wasm_path: String,
}

pub struct RunWasmBindgenResult {
    pub result_js_path: String,
    pub result_wasm_path: String,
}

#[derive(Debug)]
pub enum RunWasmBindgenError {
    IoError(std::io::Error),
}

pub fn run_wasm_bindgen(
    option: RunWasmBindgenOption,
) -> Result<RunWasmBindgenResult, RunWasmBindgenError> {
    let mut out_dir = PathBuf::from(&option.wasm_path);
    let file_name = out_dir.file_name().unwrap().to_string_lossy().to_string();
    let file_name = String::from(&file_name[..file_name.len() - 5]);
    out_dir.pop();

    let result_wasm_path = out_dir
        .join(format!("{}_bg.wasm", &file_name))
        .to_string_lossy()
        .to_string();
    let result_js_path = out_dir
        .join(format!("{}.js", &file_name))
        .to_string_lossy()
        .to_string();

    let out_dir = out_dir.to_str().unwrap();

    let command = Command::new("wasm-bindgen")
        .args([
            "--target",
            "no-modules",
            "--no-typescript",
            "--no-modules-global",
            "bundle",
            "--out-dir",
            out_dir,
            option.wasm_path.as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn();
    match command {
        Ok(mut child) => match child.wait() {
            Ok(_) => Ok(RunWasmBindgenResult {
                result_js_path,
                result_wasm_path,
            }),
            Err(error) => Err(RunWasmBindgenError::IoError(error)),
        },
        Err(error) => Err(RunWasmBindgenError::IoError(error)),
    }
}
