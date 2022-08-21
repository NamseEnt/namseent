use crate::util::get_electron_root_path;
use serde::Deserialize;
use std::process::{Command, Output};

pub struct ElectronPackageService;

impl ElectronPackageService {
    pub fn package_electron_app(
        arch: Option<Arch>,
        platform: Platform,
    ) -> Result<PackageResult, crate::Error> {
        println!("start package electron app");
        let mut args = vec!["run".to_string(), "package".to_string(), "--".to_string()];

        if let Some(arch) = arch {
            args.push(format!("--arch={}", arch));
        }
        args.push(format!("--platform={}", platform));

        let output = Command::new("npm")
            .current_dir(get_electron_root_path())
            .args(args)
            .output()
            .map_err(|error| format!("electron package fail: {}", error))?;
        let package_result = parse_package_output(&output)?;
        Ok(package_result)
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Arch {
    X64,
}
impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Arch::X64 => "x64",
            }
        )
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Platform {
    Win32,
    Linux,
}
impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Platform::Win32 => "win32",
                Platform::Linux => "linux",
            }
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageResult {
    pub arch: Arch,
    pub output_path: String,
}

fn parse_package_output(output: &Output) -> Result<PackageResult, crate::Error> {
    let output_string = String::from_utf8(output.stdout.clone())
        .map_err(|error| format!("npm package command result read fail: {}", error,))?;
    let output_string = trim_json_string(&output_string)?;
    let package_result: PackageResult = serde_json::from_str(&output_string)
        .map_err(|error| format!("Package result parse fail: {}", error))?;
    Ok(package_result)
}

fn trim_json_string(json_string: &String) -> Result<String, crate::Error> {
    let json_start_position = json_string.find("{");
    let json_end_position = json_string.rfind("}");
    if json_start_position.is_none() || json_end_position.is_none() {
        return Err("Invalid json string".into());
    }
    Ok(json_string[json_start_position.unwrap()..json_end_position.unwrap() + 1].to_string())
}
